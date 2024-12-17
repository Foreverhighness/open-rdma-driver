use core::net::Ipv4Addr;

use eui48::MacAddress;

use crate::device::software::emulator::address::DmaAddress;
use crate::device::software::emulator::queues::send::descriptors::{Seg0, Seg1};
use crate::device::software::emulator::types::{
    MemoryRegionKey, MessageSequenceNumber, PacketMtuKind, PacketSequenceNumber, QueuePairNumber, QueuePairType,
    SendFlag,
};

#[derive(Clone, Debug)]
pub(super) struct Common {
    pub total_len: u32,
    pub remote_addr: DmaAddress,
    pub remote_key: MemoryRegionKey,
    pub dest_ip: Ipv4Addr,
    pub dest_qpn: QueuePairNumber,
    pub mac: MacAddress,
    pub packet_mtu_kind: PacketMtuKind,
    pub send_flag: SendFlag,
    pub qp_type: QueuePairType,
    pub psn: PacketSequenceNumber,
    pub msn: MessageSequenceNumber,
}

impl Common {
    /// Construct Common part from Seg0
    pub fn from_seg0(seg0: &Seg0) -> Self {
        let total_len = seg0.header.total_len();

        let remote_addr = seg0.remote_addr;
        let remote_key = seg0.remote_key;
        let dest_ip = seg0.dest_ip;
        let message_sequence_number = seg0.partition_key;

        Common {
            total_len,
            remote_addr,
            remote_key,
            dest_ip,
            dest_qpn: 0,
            mac: MacAddress::default(),
            packet_mtu_kind: PacketMtuKind::default(),
            send_flag: SendFlag::default(),
            qp_type: QueuePairType::Rc,
            psn: 0,
            msn: message_sequence_number,
        }
    }

    /// Update Common part from Seg1
    pub fn with_seg1(&mut self, seg1: &Seg1) {
        self.dest_qpn = seg1.dest_queue_pair_number();
        self.mac = seg1.mac;
        self.packet_mtu_kind = seg1.packet_mtu_kind().unwrap();
        self.send_flag = seg1.send_flag();
        self.qp_type = seg1.queue_pair_type().unwrap();
        self.psn = seg1.packet_sequence_number();
    }
}