use crate::device::software::types::RdmaMessage;

mod acknowledge;
mod read_request;
mod read_response;
mod write;

mod handler {
    use crate::device::software::emulator::net::Agent;
    use crate::device::software::emulator::queues::complete_queue::CompleteQueue;
    use crate::device::software::emulator::queues::{BaseTransportHeader, BthReth, RdmaExtendedTransportHeader};
    use crate::device::software::emulator::Emulator;
    use crate::device::software::types::{Metadata, RdmaMessage};
    use crate::device::{
        ToHostWorkRbDescCommon, ToHostWorkRbDescOpcode, ToHostWorkRbDescStatus, ToHostWorkRbDescTransType,
        ToHostWorkRbDescWriteOrReadResp,
    };
    use crate::types::{Msn, Psn};

    impl<UA: Agent> Emulator<UA> {
        pub(crate) fn handle_message(&self, msg: &RdmaMessage) {
            let meta = &msg.meta_data;

            // TODO(fh): validate part
            // TODO(fh): dma part
            let descriptor = message_to_descriptor(msg);
            log::debug!("push meta report: {descriptor:?}");
            unsafe { self.meta_report_queue().push(descriptor) };
        }
    }
    fn message_to_descriptor(msg: &RdmaMessage) -> BthReth {
        let meta = &msg.meta_data;
        match meta {
            Metadata::General(header) => match header.common_meta.opcode {
                ToHostWorkRbDescOpcode::RdmaWriteFirst | ToHostWorkRbDescOpcode::RdmaWriteLast => {
                    let op = ToHostWorkRbDescWriteOrReadResp {
                        common: ToHostWorkRbDescCommon {
                            // TODO(fh): remove hard code value
                            status: ToHostWorkRbDescStatus::Normal,
                            // TODO(fh): remove hard code value
                            trans: ToHostWorkRbDescTransType::Rc,
                            dqpn: crate::types::Qpn::new(msg.meta_data.common_meta().dqpn.get()),
                            msn: Msn::new(msg.meta_data.common_meta().pkey.get()),
                            // TODO(fh): remove hard code value
                            expected_psn: Psn::new(0),
                        },
                        is_read_resp: header.common_meta.opcode.is_read_resp(),
                        write_type: header
                            .common_meta
                            .opcode
                            .write_type()
                            .unwrap_or(crate::device::ToHostWorkRbDescWriteType::Only),
                        psn: header.common_meta.psn,
                        addr: header.reth.va,
                        len: header.reth.len,
                        can_auto_ack: true,
                    };
                    // Operation -> Descriptor
                    let descriptor = {
                        let expect_psn = header.common_meta.psn.get();
                        let req_status = op.common.status.into();
                        let msn = op.common.msn.get();
                        let can_auto_ack = op.can_auto_ack;

                        let trans_type = op.common.trans.into();
                        let opcode = header.common_meta.opcode.clone().into();
                        let qpn = op.common.dqpn.get();
                        let psn = op.psn.get();
                        let pad_cnt = msg.payload.get_pad_cnt() as u8;
                        // from software copy snip
                        let solicited = false;
                        let is_ack_req = false;

                        let bth =
                            BaseTransportHeader::new(trans_type, opcode, qpn, psn, solicited, is_ack_req, pad_cnt);

                        let local_va = header.reth.va.into();
                        let local_key = header.reth.rkey.get().into();
                        let len = header.reth.len;
                        let reth = RdmaExtendedTransportHeader::new(local_va, local_key, len);

                        BthReth::new(expect_psn, req_status, bth, reth, msn, can_auto_ack)
                    };
                    descriptor
                }
                _ => todo!(),
            },
            Metadata::Acknowledge(_header) => todo!(),
        }
    }

    #[cfg(test)]
    mod tests {
        use smoltcp::wire::{EthernetFrame, Ipv4Packet, UdpPacket};

        use crate::device::software::emulator::net::message::handler::message_to_descriptor;
        use crate::device::software::emulator::queues::BthReth;
        use crate::device::software::packet_processor::PacketProcessor;

        #[test]
        fn test_message_to_descriptor() {
            let expected = vec![
                [
                    0x00, 0x00, 0x00, 0x01, 0x30, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x8F,
                    0x7E, 0x7F, 0x00, 0x00, 0xD2, 0xE7, 0x03, 0x02, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
                ],
                // [
                //     0x00, 0x00, 0x00, 0x01, 0x88, 0x02, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                //     0x00, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                // ],
            ];
            let expected = expected.into_iter().map(BthReth::from_ne_bytes).collect::<Vec<_>>();
            // println!("{expected:#?}");

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

                let bth_reth = message_to_descriptor(&msg);

                println!("{:#?}", bth_reth);
            }
        }
    }
}
