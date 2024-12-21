use papaya::HashMap;

use super::types::{MemoryAccessFlag, PathMtuKind, ProtectDomainHandler, QueuePairNumber, QueuePairType};

#[derive(Debug, PartialEq)]
pub struct Context {
    queue_pair_number: QueuePairNumber,
    protect_domain_handler: ProtectDomainHandler,
    queue_pair_type: QueuePairType,
    access_flag: MemoryAccessFlag,
    path_mtu_kind: PathMtuKind,
}

impl Context {
    pub fn new(
        queue_pair_number: QueuePairNumber,
        protect_domain_handler: ProtectDomainHandler,
        queue_pair_type: QueuePairType,
        access_flag: MemoryAccessFlag,
        path_mtu_kind: PathMtuKind,
    ) -> Self {
        Self {
            queue_pair_number,
            protect_domain_handler,
            queue_pair_type,
            access_flag,
            path_mtu_kind,
        }
    }
}

#[derive(Debug, Default)]
pub struct Table(HashMap<QueuePairNumber, Context>);

impl Table {
    pub fn insert(&self, qp_context: Context) -> bool {
        log::debug!("insert qp_table with {qp_context:?}");

        let qp_table = self.0.pin();
        let exist = qp_table.insert(qp_context.queue_pair_number, qp_context).is_some();

        log::trace!("after insertion qp_table: {self:?}");

        exist
    }

    pub fn remove(&self, qpn: QueuePairNumber) -> bool {
        log::debug!("remove qp_table with {qpn:?}");

        let qp_table = self.0.pin();
        let exist = qp_table.remove(&qpn).is_some();

        log::trace!("after removal qp_table: {self:?}");

        exist
    }
}
