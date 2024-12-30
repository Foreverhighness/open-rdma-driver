use core::net::Ipv4Addr;

use eui48::MacAddress;

use crate::address::VirtualAddress;
use crate::queues::send::descriptors::{Seg0, Seg1};
use crate::types::{
    MemoryRegionKey, MessageSequenceNumber, PacketSequenceNumber, PathMtuKind, QueuePairNumber, QueuePairType, SendFlag,
};

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
    pub fn from_seg0(seg0: &Seg0) -> Self {
        let total_len = seg0.header.total_len();

        let remote_addr = seg0.remote_addr;
        let remote_key = seg0.remote_key;
        let dest_ip = seg0.dest_ip();
        let message_sequence_number = seg0.partition_key;

        Common {
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
    pub fn with_seg1(&mut self, seg1: &Seg1) {
        self.dest_qpn = seg1.dest_queue_pair_number();
        self.dest_mac = seg1.mac();
        self.path_mtu_kind = seg1.path_mtu_kind().unwrap();
        self.send_flag = seg1.send_flag();
        self.qp_type = seg1.queue_pair_type().unwrap();
        self.psn = seg1.packet_sequence_number();
    }
}
