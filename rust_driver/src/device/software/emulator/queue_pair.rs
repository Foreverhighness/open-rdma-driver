use papaya::HashMap;

use super::types::{MemoryAccessFlag, PacketMtuKind, ProtectDomainHandler, QueuePairNumber, QueuePairType};

#[derive(Debug, PartialEq)]
pub struct Context {
    queue_pair_number: QueuePairNumber,
    protect_domain_handler: ProtectDomainHandler,
    queue_pair_type: QueuePairType,
    access_flag: MemoryAccessFlag,
    packet_mtu_kind: PacketMtuKind,
}

impl Context {
    pub fn new(
        queue_pair_number: QueuePairNumber,
        protect_domain_handler: ProtectDomainHandler,
        queue_pair_type: QueuePairType,
        access_flag: MemoryAccessFlag,
        packet_mtu_kind: PacketMtuKind,
    ) -> Self {
        Self {
            queue_pair_number,
            protect_domain_handler,
            queue_pair_type,
            access_flag,
            packet_mtu_kind,
        }
    }
}

#[derive(Debug, Default)]
pub struct Table(HashMap<QueuePairNumber, Context>);

impl Table {
    pub fn insert(&self, qp_context: Context) -> bool {
        let qp_table = self.0.pin();

        qp_table.insert(qp_context.queue_pair_number, qp_context).is_some()
    }

    pub fn remove(&self, qpn: QueuePairNumber) -> bool {
        let qp_table = self.0.pin();

        qp_table.remove(&qpn).is_some()
    }
}
