use smoltcp::wire::{EthernetFrame, Ipv4Packet, UdpPacket};

use super::common::Common;
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::queues::send::descriptors::{
    ScatterGatherElement, Seg0, Seg1, VariableLengthSge,
};
use crate::device::software::emulator::{Emulator, Result};

#[derive(Debug)]
pub struct Write {
    common: Common,
    last: bool,
    first: bool,
    sge: ScatterGatherElement,
}

impl<UA: Agent> HandleDescriptor<Write> for Emulator<UA> {
    type Context = ();
    type Output = ();

    fn handle(&self, req: &Write, _: &mut ()) -> Result<Self::Output> {
        log::info!("handle write op: {req:#?}");

        // let path_mtu = u32::from(&req.common.path_mtu_kind);
        let addr = req.common.dest_ip;

        let files = vec![
            ".cache/captures/ethernet-frame-0.bin",
            ".cache/captures/ethernet-frame-1.bin",
        ];

        for file in files {
            let buffer = std::fs::read(file).unwrap();

            let eth_frame = EthernetFrame::new_checked(buffer.as_slice()).unwrap();
            let ipv4_packet = Ipv4Packet::new_checked(eth_frame.payload()).unwrap();
            let udp_packet = UdpPacket::new_checked(ipv4_packet.payload()).unwrap();

            let payload = udp_packet.payload();
            let amount = self.udp_agent.get().unwrap().send_to(payload, addr.into())?;
        }

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
