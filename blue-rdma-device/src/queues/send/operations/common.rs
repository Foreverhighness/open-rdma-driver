use core::net::Ipv4Addr;

use eui48::MacAddress;

use crate::address::VirtualAddress;
use crate::dma::PointerMut;
use crate::mr_table::MemoryRegionTable;
use crate::net::util::generate_payload_from_msg;
use crate::queues::send::descriptors::{Seg0, Seg1};
use crate::third_party::net::{
    Key, Metadata, PKey, PayloadInfo, Qpn, RdmaGeneralMeta, RdmaMessage, RdmaMessageMetaCommon, RethHeader,
};
use crate::third_party::queues::meta_report::ToHostWorkRbDescOpcode;
use crate::third_party::rdma::{MemAccessTypeFlag, Psn};
use crate::types::{
    MemoryRegionKey, MessageSequenceNumber, PacketSequenceNumber, PathMtuKind, QueuePairNumber, QueuePairType, SendFlag,
};
use crate::{DeviceInner, dma, net};

#[derive(Clone, Debug)]
pub(super) struct Common {
    pub total_len: u32,
    pub remote_addr: VirtualAddress,
    pub remote_key: MemoryRegionKey,
    pub dest_ip: Ipv4Addr,
    pub dest_qpn: QueuePairNumber,
    pub dest_mac: MacAddress,
    pub path_mtu_kind: PathMtuKind,
    pub send_flag: SendFlag,
    pub qp_type: QueuePairType,
    pub psn: PacketSequenceNumber,
    pub msn: MessageSequenceNumber,
}

impl Common {
    /// Construct Common part from Seg0
    pub fn from_seg0(seg0: Seg0) -> Self {
        let total_len = seg0.header.total_len();

        let remote_addr = seg0.remote_addr;
        let remote_key = seg0.remote_key;
        let dest_ip = seg0.dest_ip();
        let message_sequence_number = seg0.partition_key;

        Self {
            total_len,
            remote_addr,
            remote_key,
            dest_ip,
            dest_qpn: 0,
            dest_mac: MacAddress::default(),
            path_mtu_kind: PathMtuKind::default(),
            send_flag: SendFlag::default(),
            qp_type: QueuePairType::Rc,
            psn: 0,
            msn: message_sequence_number,
        }
    }

    /// Update Common part from Seg1
    pub fn with_seg1(&mut self, seg1: Seg1) {
        self.dest_qpn = seg1.dest_queue_pair_number();
        self.dest_mac = seg1.mac();
        self.path_mtu_kind = seg1.path_mtu_kind().unwrap();
        self.send_flag = seg1.send_flag();
        self.qp_type = seg1.queue_pair_type().unwrap();
        self.psn = seg1.packet_sequence_number();
    }
}

impl<UA: net::Agent, DC: dma::Client> DeviceInner<UA, DC> {
    #[expect(clippy::too_many_arguments, reason = "this function may removed later")]
    pub(super) fn send_write_message<Req: AsRef<Common>>(
        &self,
        req: Req,
        opcode: ToHostWorkRbDescOpcode,
        psn: PacketSequenceNumber,
        ack_req: bool,
        key: crate::types::MemoryRegionKey,
        remote_va: u64,
        segment: &Segment,
    ) {
        let common = req.as_ref();
        let rkey = Key::new(common.remote_key.get());
        let dst = common.dest_ip;

        let meta = RdmaMessageMetaCommon {
            tran_type: common.qp_type.into(),
            opcode,
            solicited: false,
            // We use the pkey to store msn
            pkey: PKey::new(common.msn),
            dqpn: Qpn::new(common.dest_qpn),
            ack_req,
            psn: Psn::new(psn),
        };

        let dma_addr = self
            .mr_table
            .query(key, segment.va, MemAccessTypeFlag::empty(), &self.page_table)
            .expect("verify key failed");
        let ptr = self.dma_client.with_dma_addr::<u8>(dma_addr);
        let len = segment.len as usize;
        let mut data = vec![0u8; len];
        // SAFETY: caller should guarantee ptr is valid dma ptr
        unsafe { ptr.copy_to_nonoverlapping(data.as_mut_ptr(), len) };

        let payload = PayloadInfo::new_with_data(data.as_ptr(), len);

        let write_msg = RdmaMessage {
            meta_data: Metadata::General(RdmaGeneralMeta {
                common_meta: meta,
                reth: RethHeader {
                    va: remote_va,
                    rkey,
                    len: common.total_len,
                },
                imm: None,
                secondary_reth: None,
            }),
            payload,
        };
        // FIXME(fh): hardcode for calculate Invariant CRC, should remove
        let src = Ipv4Addr::new(192, 168, 0, 2);
        let payload = generate_payload_from_msg(&write_msg, src, dst);
        let _ = self
            .udp_agent
            .get()
            .unwrap()
            .send_to(&payload, dst.into())
            .expect("send error");
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Segment {
    va: VirtualAddress,
    len: u32,
}

impl Segment {
    pub const fn new(va: VirtualAddress, len: u32) -> Self {
        Self { va, len }
    }

    pub const fn len(&self) -> u32 {
        self.len
    }
}

// TODO(fh): rewrite with gen block in Rust Edition 2024
pub(super) fn generate_segments_from_request(mut va: u64, len: u32, path_mtu: u32) -> Vec<Segment> {
    let mut segments = Vec::new();

    let mut remainder = len;

    // special judge on first element
    #[expect(clippy::cast_possible_truncation, reason = "truncate va into u32")]
    let len = remainder.min(path_mtu - ((va as u32) % path_mtu));
    let seg = Segment::new(va.into(), len);
    segments.push(seg);

    va += u64::from(len);
    remainder -= len;

    while remainder > 0 {
        let len = remainder.min(path_mtu);
        let seg = Segment::new(va.into(), len);
        segments.push(seg);

        va += u64::from(len);
        remainder -= len;
    }

    segments
}

#[cfg(test)]
mod tests {
    use core::ffi::{c_int, c_void};
    use core::ptr;

    use libc::{off_t, size_t};
    use smoltcp::wire::{EthernetFrame, Ipv4Packet, UdpPacket};

    use super::*;
    use crate::third_party::queues::meta_report::ToHostWorkRbDescTransType;

    #[test]
    fn test_generate_segments_from_request() {
        let va = 0x00007F7E8EE00000;
        let path_mtu = 4096;
        let len = 6144;

        let expected = vec![
            Segment::new(va.into(), 4096),
            Segment::new((va + u64::from(path_mtu)).into(), 2048),
        ];

        let segments = generate_segments_from_request(va, len, path_mtu);
        assert_eq!(expected, segments);
    }

    #[test]
    fn test_generate_write_first_and_last() {
        let mut remote_va = 0x00007F7E8FC00000;
        let va = 0x00007F7E8EE00000;
        let path_mtu = 4096;
        let len = 6144;
        let rkey = Key::new(33554435);
        let src = Ipv4Addr::new(192, 168, 0, 2);
        let dst = Ipv4Addr::new(192, 168, 0, 3);
        let mut psn = 0u32;

        let segments = generate_segments_from_request(va, len, path_mtu);
        let (first, last) = (&segments[0], &segments[1]);
        let common_meta = move |opcode, psn, ack_req| {
            RdmaMessageMetaCommon {
                tran_type: ToHostWorkRbDescTransType::Rc,
                solicited: false,
                opcode,
                // We use the pkey to store msn
                pkey: PKey::new(0),
                dqpn: Qpn::new(2),
                ack_req,
                psn: Psn::new(psn),
            }
        };

        unsafe extern "C" {
            fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off_t)
            -> *mut c_void;
        }

        let addr = ptr::null_mut::<c_void>().with_addr(first.va.0.try_into().unwrap());
        let data = unsafe { mmap(addr, 8192, 1 | 2, 0x02 | 0x20, -1, 0) };
        assert!(ptr::addr_eq(addr, data));
        let data = data.cast::<u8>();
        let slice = unsafe { core::slice::from_raw_parts_mut(data, 6144) };
        slice.iter_mut().enumerate().for_each(|(i, e)| *e = i as u8);

        let payload = PayloadInfo::new_with_data(data, first.len as usize);
        let write_first_msg = RdmaMessage {
            meta_data: Metadata::General(RdmaGeneralMeta {
                common_meta: common_meta(ToHostWorkRbDescOpcode::RdmaWriteFirst, psn, false),
                reth: RethHeader {
                    va: remote_va,
                    rkey,
                    len,
                },
                imm: None,
                secondary_reth: None,
            }),
            payload,
        };
        let payload_first = generate_payload_from_msg(&write_first_msg, src, dst);

        remote_va += u64::from(first.len);
        psn = psn.wrapping_add(1);

        let payload = PayloadInfo::new_with_data(last.va.0 as *const u8, last.len as usize);
        let write_last_msg = RdmaMessage {
            meta_data: Metadata::General(RdmaGeneralMeta {
                common_meta: common_meta(ToHostWorkRbDescOpcode::RdmaWriteLast, psn, true),
                reth: RethHeader {
                    va: remote_va,
                    rkey,
                    len: last.len,
                },
                imm: None,
                secondary_reth: None,
            }),
            payload,
        };
        let payload_last = generate_payload_from_msg(&write_last_msg, src, dst);

        // let msg_first = PacketProcessor::to_rdma_message(&payload_first).unwrap();
        // let msg_last = PacketProcessor::to_rdma_message(&payload_last).unwrap();
        // println!("{msg_first:#?}");
        // println!("{msg_last:#?}");

        let filename = &".cache/captures/ethernet-frame-0.bin".to_owned();
        let buffer = std::fs::read(filename).unwrap();

        let eth_frame = EthernetFrame::new_checked(buffer.as_slice()).unwrap();
        let ipv4_packet = Ipv4Packet::new_checked(eth_frame.payload()).unwrap();
        let udp_packet = UdpPacket::new_checked(ipv4_packet.payload()).unwrap();

        let expected = udp_packet.payload();
        assert_eq!(expected, payload_first);

        let filename = &".cache/captures/ethernet-frame-1.bin".to_owned();
        let buffer = std::fs::read(filename).unwrap();

        let eth_frame = EthernetFrame::new_checked(buffer.as_slice()).unwrap();
        let ipv4_packet = Ipv4Packet::new_checked(eth_frame.payload()).unwrap();
        let udp_packet = UdpPacket::new_checked(ipv4_packet.payload()).unwrap();

        let expected = udp_packet.payload();
        assert_eq!(expected, payload_last);
    }
}
