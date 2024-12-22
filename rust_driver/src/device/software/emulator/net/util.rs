use core::net::Ipv4Addr;

use smoltcp::wire::{Ipv4Packet, UdpPacket};

use crate::device::software::emulator::net::RDMA_PROT;
use crate::device::software::emulator::queues::{
    AckExtendedTransportHeader, BaseTransportHeader, BthAeth, BthReth, RdmaExtendedTransportHeader,
};
use crate::device::software::packet::{AETH, BTH};
use crate::device::software::packet_processor::PacketWriter;
use crate::device::software::types::{AethHeader, Metadata, PayloadInfo, RdmaMessage};
use crate::device::{
    ToHostWorkRbDescAethCode, ToHostWorkRbDescOpcode, ToHostWorkRbDescStatus, ToHostWorkRbDescTransType,
};

pub(super) fn message_to_bthreth(msg: &RdmaMessage) -> BthReth {
    let meta = &msg.meta_data;
    match meta {
        Metadata::General(header) => match header.common_meta.opcode {
            ToHostWorkRbDescOpcode::RdmaWriteFirst | ToHostWorkRbDescOpcode::RdmaWriteLast => {
                // TODO(fh): Add helper function to all operation
                // Operation -> Descriptor
                let descriptor = {
                    let trans_type = ToHostWorkRbDescTransType::Rc.into();
                    let opcode = header.common_meta.opcode.clone().into();
                    let qpn = header.common_meta.dqpn.get();
                    let psn = header.common_meta.psn.get();
                    let pad_cnt = msg.payload.get_pad_cnt() as u8;
                    // from software copy snip
                    let solicited = false;
                    let is_ack_req = false;

                    let bth = BaseTransportHeader::new(trans_type, opcode, qpn, psn, solicited, is_ack_req, pad_cnt);

                    let local_va = header.reth.va.into();
                    let local_key = header.reth.rkey.get().into();
                    let len = header.reth.len;
                    let reth = RdmaExtendedTransportHeader::new(local_va, local_key, len);

                    let expect_psn = header.common_meta.psn.get();
                    let req_status = ToHostWorkRbDescStatus::Normal.into();
                    let msn = header.common_meta.pkey.get().into();
                    let can_auto_ack = true;
                    BthReth::new(expect_psn, req_status, bth, reth, msn, can_auto_ack)
                };
                descriptor
            }
            _ => todo!(),
        },
        Metadata::Acknowledge(_header) => todo!(),
    }
}

pub(super) fn message_to_bthaeth(msg: &RdmaMessage) -> BthAeth {
    let Metadata::Acknowledge(ref header) = msg.meta_data else {
        unreachable!();
    };

    assert_eq!(
        header.aeth_code,
        ToHostWorkRbDescAethCode::Ack,
        "currently only support normal Ack"
    );

    let trans_type = ToHostWorkRbDescTransType::Rc.into();
    let opcode = header.common_meta.opcode.clone().into();
    let qpn = header.common_meta.dqpn.get();
    let psn = header.common_meta.psn.get();
    let pad_cnt = msg.payload.get_pad_cnt() as u8;
    // from software copy snip
    let solicited = false;
    let is_ack_req = false;
    let bth = BaseTransportHeader::new(trans_type, opcode, qpn, psn, solicited, is_ack_req, pad_cnt);

    let msn = header.msn as u16;
    let value = header.aeth_value;
    let code = header.aeth_code.clone();
    let aeth = AckExtendedTransportHeader::new(psn, msn, value, code);

    let req_status = ToHostWorkRbDescStatus::Normal.into();
    BthAeth::new(req_status, bth, aeth)
}

pub(super) fn generate_ack(msg: &RdmaMessage) -> Vec<u8> {
    let ack = {
        let buf = [0u8; 12 + 4];
        let bth = BTH::from_bytes(&buf);
        bth.set_opcode_and_type(ToHostWorkRbDescOpcode::Acknowledge, ToHostWorkRbDescTransType::Rc);
        bth.set_destination_qpn(msg.meta_data.common_meta().dqpn.get());
        bth.set_psn(msg.meta_data.common_meta().psn.get());
        bth.set_ack_req(false);
        bth.set_flags_solicited(false);
        bth.set_pkey(msg.meta_data.common_meta().pkey.get());
        let aeth = AETH::from_bytes(&buf[12..]);
        aeth.set_aeth_code_and_value(ToHostWorkRbDescAethCode::Ack.into(), 0x1f);
        aeth.set_msn(msg.meta_data.common_meta().pkey.get().into());
        RdmaMessage {
            meta_data: Metadata::Acknowledge(AethHeader::new_from_packet(bth, aeth).unwrap()),
            payload: PayloadInfo::new(),
        }
    };
    let mut buf = [0; 48];
    let len = PacketWriter::new(&mut buf)
        .src_addr(Ipv4Addr::new(192, 168, 0, 3))
        .src_port(RDMA_PROT)
        .dest_addr(Ipv4Addr::new(192, 168, 0, 2))
        .dest_port(RDMA_PROT)
        .ip_id(1)
        .message(&ack)
        .write()
        .unwrap();
    assert_eq!(buf.len(), len);
    let ip_packet = Ipv4Packet::new_checked(&buf).unwrap();
    let udp_datagram = UdpPacket::new_checked(ip_packet.payload()).unwrap();
    udp_datagram.payload().to_vec()
}

#[cfg(test)]
mod tests {
    use smoltcp::wire::{EthernetFrame, Ipv4Packet, UdpPacket};

    use super::{generate_ack, message_to_bthreth, RdmaMessage};
    use crate::device::software::emulator::queues::{BthAeth, BthReth};
    use crate::device::software::packet_processor::PacketProcessor;

    fn write_first_message() -> RdmaMessage {
        let file = ".cache/captures/ethernet-frame-0.bin";

        let buffer = std::fs::read(file).unwrap();
        let buffer = &*buffer.leak();

        let eth_frame = EthernetFrame::new_checked(buffer).unwrap();
        let ipv4_packet = Ipv4Packet::new_checked(eth_frame.payload()).unwrap();
        let udp_packet = UdpPacket::new_checked(ipv4_packet.payload()).unwrap();

        let payload = udp_packet.payload();

        PacketProcessor::to_rdma_message(payload).unwrap()
    }

    #[test]
    fn test_dma_copy() {
        let msg = write_first_message();

        let data = &msg.payload.sg_list;
        assert_eq!(data.len(), 1, "currently only consider one Sge");
        let data = data[0];
        assert!(data.len >= 4);
        // skip invariant crc
        let slice = unsafe { core::slice::from_raw_parts(data.data, data.len - 4) };
        for (i, &v) in slice.iter().enumerate() {
            assert_eq!(i as u8, v);
        }
    }

    #[test]
    fn test_generate_ack() {
        let expected = &[
            0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x1f, 0x00, 0x00, 0x00, 0xba, 0x11,
            0xc7, 0x23,
        ];
        let msg = write_first_message();
        let ack = generate_ack(&msg);

        assert_eq!(&ack, expected);
    }

    #[test]
    fn test_message_to_descriptor() {
        let expected = vec![[
            0x00, 0x00, 0x00, 0x01, 0x30, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x8F, 0x7E, 0x7F,
            0x00, 0x00, 0xD2, 0xE7, 0x03, 0x02, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
        ]];
        let expected = expected.into_iter().map(BthReth::from_ne_bytes).collect::<Vec<_>>();
        // println!("{expected:#?}");
        let ack = [
            0x00, 0x00, 0x00, 0x01, 0x88, 0x02, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let ack = BthAeth::from_ne_bytes(ack);
        println!("{ack:#?}");

        let files = vec![
            ".cache/captures/ethernet-frame-0.bin",
            // ".cache/captures/ethernet-frame-1.bin",
        ];

        for file in files {
            let buffer = std::fs::read(file).unwrap();

            let eth_frame = EthernetFrame::new_checked(buffer.as_slice()).unwrap();
            let ipv4_packet = Ipv4Packet::new_checked(eth_frame.payload()).unwrap();
            let udp_packet = UdpPacket::new_checked(ipv4_packet.payload()).unwrap();

            let payload = udp_packet.payload();

            let msg = PacketProcessor::to_rdma_message(payload).unwrap();

            let bth_reth = message_to_bthreth(&msg);

            // println!("{:#?}", bth_reth);
        }
    }
}
