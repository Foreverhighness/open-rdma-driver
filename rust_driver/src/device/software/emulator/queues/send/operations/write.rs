use core::net::Ipv4Addr;

use smoltcp::wire::{EthernetFrame, Ipv4Packet, UdpPacket};

use super::common::Common;
use crate::device::software::emulator::address::VirtualAddress;
use crate::device::software::emulator::dma::{self, Client, PointerMut};
use crate::device::software::emulator::mr_table::MemoryRegionTable;
use crate::device::software::emulator::net::util::generate_payload_from_msg;
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::queues::send::descriptors::{
    ScatterGatherElement, Seg0, Seg1, VariableLengthSge,
};
use crate::device::software::emulator::{DeviceInner, Result};
use crate::device::software::types::{
    Key, Metadata, PKey, PayloadInfo, Qpn, RdmaGeneralMeta, RdmaMessage, RdmaMessageMetaCommon, RethHeader,
};
use crate::device::ToHostWorkRbDescOpcode;
use crate::types::{MemAccessTypeFlag, Psn};

#[derive(Debug)]
pub struct Write {
    common: Common,
    last: bool,
    first: bool,
    sge: ScatterGatherElement,
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
}

fn generate_segments_from_request(mut va: u64, len: u32, path_mtu: u32) -> Vec<Segment> {
    let mut segments = Vec::new();

    let mut remainder = len;

    // special judge on first element
    let len = remainder.min(path_mtu - ((va as u32) % path_mtu));
    let seg = Segment::new(va.into(), len);
    segments.push(seg);

    va += len as u64;
    remainder -= len;

    while remainder > 0 {
        let len = remainder.min(path_mtu);
        let seg = Segment::new(va.into(), len);
        segments.push(seg);

        va += len as u64;
        remainder -= len;
    }

    segments
}

impl<UA: Agent, DC: Client> HandleDescriptor<Write> for DeviceInner<UA, DC> {
    type Context = ();
    type Output = ();

    fn handle(&self, req: &Write, _: &mut ()) -> Result<Self::Output> {
        log::info!("handle write op: {req:#?}");

        let path_mtu = u32::from(&req.common.path_mtu_kind);
        let segments = generate_segments_from_request(req.sge.local_addr.0, req.sge.len, path_mtu);
        let rkey = Key::new(req.common.remote_key.get());
        let dst = req.common.dest_ip;
        let src = Ipv4Addr::new(192, 168, 0, 2);
        let mut psn = req.common.psn;
        let key = req.sge.local_key;

        let common_meta = move |opcode, psn, ack_req| {
            RdmaMessageMetaCommon {
                tran_type: req.common.qp_type.into(),
                opcode,
                solicited: false,
                // We use the pkey to store msn
                pkey: PKey::new(req.common.msn),
                dqpn: Qpn::new(req.common.dest_qpn),
                ack_req,
                psn: Psn::new(psn),
            }
        };

        match segments.as_slice() {
            [_only] => todo!(),
            [first, _middles @ .., last] => {
                let mut remote_va = req.common.remote_addr.0;
                let dma_addr = self
                    .mr_table
                    .query(key, first.va, MemAccessTypeFlag::empty(), &self.page_table)
                    .unwrap();
                let ptr = self.dma_client.with_dma_addr::<u8>(dma_addr);
                let len = first.len as usize;
                let mut data = vec![0u8; len];
                unsafe { ptr.copy_to_nonoverlapping(data.as_mut_ptr(), len) };
                let payload = PayloadInfo::new_with_data(data.as_ptr(), len);
                let write_first_msg = RdmaMessage {
                    meta_data: Metadata::General(RdmaGeneralMeta {
                        common_meta: common_meta(ToHostWorkRbDescOpcode::RdmaWriteFirst, psn, false),
                        reth: RethHeader {
                            va: remote_va,
                            rkey,
                            len: req.common.total_len,
                        },
                        imm: None,
                        secondary_reth: None,
                    }),
                    payload,
                };
                let payload = generate_payload_from_msg(&write_first_msg, src, dst);
                let _ = self
                    .udp_agent
                    .get()
                    .unwrap()
                    .send_to(&payload, dst.into())
                    .expect("send error");

                remote_va += first.len as u64;
                psn = psn.wrapping_add(1);

                let dma_addr = self
                    .mr_table
                    .query(key, last.va, MemAccessTypeFlag::empty(), &self.page_table)
                    .unwrap();
                let ptr = self.dma_client.with_dma_addr::<u8>(dma_addr);
                let len = last.len as usize;
                let mut data = vec![0u8; len];
                unsafe { ptr.copy_to_nonoverlapping(data.as_mut_ptr(), len) };

                let payload = PayloadInfo::new_with_data(data.as_ptr(), last.len as usize);
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
                let payload = generate_payload_from_msg(&write_last_msg, src, dst);
                let _ = self
                    .udp_agent
                    .get()
                    .unwrap()
                    .send_to(&payload, dst.into())
                    .expect("send error");
            }
            [] => todo!(),
        }

        // let addr = req.common.dest_ip;

        // let files = vec![
        //     ".cache/captures/ethernet-frame-0.bin",
        //     ".cache/captures/ethernet-frame-1.bin",
        // ];

        // for file in files {
        //     let buffer = std::fs::read(file).unwrap();

        //     let eth_frame = EthernetFrame::new_checked(buffer.as_slice()).unwrap();
        //     let ipv4_packet = Ipv4Packet::new_checked(eth_frame.payload()).unwrap();
        //     let udp_packet = UdpPacket::new_checked(ipv4_packet.payload()).unwrap();

        //     let payload = udp_packet.payload();
        //     let amount = self.udp_agent.get().unwrap().send_to(payload, addr.into())?;
        // }

        Ok(())
    }
}

#[derive(Debug)]
/// Write Builder
// TODO(fh): use strict state machine representation?
pub struct Builder(Write);

impl Builder {
    /// Initialize builder from valid seg0
    pub fn from_seg0(seg0: Seg0) -> Self {
        let first = seg0.header.first();
        let last = seg0.header.last();
        Self(Write {
            common: Common::from_seg0(&seg0),
            last,
            first,
            sge: ScatterGatherElement {
                local_key: 0.into(),
                len: 0,
                local_addr: 0.into(),
            },
        })
    }

    /// Update valid seg1, assuming only seg0 is processed
    pub fn with_seg1(mut self, seg1: Seg1) -> Self {
        self.0.common.with_seg1(&seg1);

        self
    }

    /// Update sge, assuming seg0 and seg1 are processed
    pub fn with_sge(mut self, sge: VariableLengthSge) -> Write {
        let sge0 = sge.sge1;
        let sge1 = sge.sge2;
        assert!(
            sge1.local_addr == 0.into() && sge1.len == 0 && sge1.local_key == 0.into(),
            "support only one sge for now"
        );

        self.0.sge = sge0;

        self.0
    }
}

#[cfg(test)]
mod tests {
    use core::ffi::{c_int, c_void};

    use libc::{off_t, size_t};

    use super::*;
    use crate::device::software::packet_processor::PacketProcessor;

    #[test]
    fn test_generate_segments_from_request() {
        let va = 0x00007F7E8EE00000;
        let path_mtu = 4096;
        let len = 6144;

        let expected = vec![
            Segment::new(va.into(), 4096),
            Segment::new((va + path_mtu as u64).into(), 2048),
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
        let mut psn = 0;

        let segments = generate_segments_from_request(va, len, path_mtu);
        let (first, last) = (&segments[0], &segments[1]);
        let common_meta = move |opcode, psn, ack_req| {
            RdmaMessageMetaCommon {
                tran_type: crate::device::ToHostWorkRbDescTransType::Rc,
                solicited: false,
                opcode,
                // We use the pkey to store msn
                pkey: PKey::new(0),
                dqpn: Qpn::new(2),
                ack_req,
                psn: Psn::new(psn),
            }
        };

        extern "C" {
            fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off_t)
                -> *mut c_void;
        }

        let va = first.va.0;
        let data = unsafe { mmap(va as usize as _, 8192, 1 | 2, 0x02 | 0x20, -1, 0) };
        let data = data.cast::<u8>();
        let slice = unsafe { core::slice::from_raw_parts_mut(data, 6144) };
        slice.iter_mut().enumerate().for_each(|(i, e)| *e = i as u8);

        let payload = PayloadInfo::new_with_data(first.va.0 as *const u8, first.len as usize);
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

        remote_va += first.len as u64;
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

        let filename = &format!(".cache/captures/ethernet-frame-0.bin");
        let buffer = std::fs::read(filename).unwrap();

        let eth_frame = EthernetFrame::new_checked(buffer.as_slice()).unwrap();
        let ipv4_packet = Ipv4Packet::new_checked(eth_frame.payload()).unwrap();
        let udp_packet = UdpPacket::new_checked(ipv4_packet.payload()).unwrap();

        let expected = udp_packet.payload();
        assert_eq!(expected, payload_first);

        let filename = &format!(".cache/captures/ethernet-frame-1.bin");
        let buffer = std::fs::read(filename).unwrap();

        let eth_frame = EthernetFrame::new_checked(buffer.as_slice()).unwrap();
        let ipv4_packet = Ipv4Packet::new_checked(eth_frame.payload()).unwrap();
        let udp_packet = UdpPacket::new_checked(ipv4_packet.payload()).unwrap();

        let expected = udp_packet.payload();
        assert_eq!(expected, payload_last);
    }
}
