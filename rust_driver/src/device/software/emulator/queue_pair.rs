use core::sync::atomic::{AtomicBool, AtomicU32};

use papaya::HashMap;

use super::types::{
    MemoryAccessFlag, PacketSequenceNumber, PathMtuKind, ProtectDomainHandler, QueuePairNumber, QueuePairType,
};

#[derive(Debug)]
pub struct Context {
    queue_pair_number: QueuePairNumber,
    protect_domain_handler: ProtectDomainHandler,
    queue_pair_type: QueuePairType,
    access_flag: MemoryAccessFlag,
    path_mtu_kind: PathMtuKind,
    error_state: AtomicBool,
    expected_psn: AtomicU32,
}

impl Context {
    pub const fn new(
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
            error_state: AtomicBool::new(false),
            expected_psn: AtomicU32::new(0),
        }
    }

    /// try recover from error state, return true if current state is not error state
    pub fn try_recover(&self, psn: PacketSequenceNumber) -> bool {
        let expected_psn = self.expected_psn();
        let is_error = self.is_error();
        if !is_error {
            panic!("logic_error: not error state but get recover")
        }
        if expected_psn == psn {
            self.clear_error();
            true
        } else {
            false
        }
    }

    /// set to error state
    pub fn set_error(&self) {
        self.error_state.store(true, core::sync::atomic::Ordering::SeqCst);
    }

    /// clear error state
    pub fn clear_error(&self) {
        self.error_state.store(false, core::sync::atomic::Ordering::SeqCst);
    }

    /// get error state
    pub fn is_error(&self) -> bool {
        self.error_state.load(core::sync::atomic::Ordering::SeqCst)
    }

    /// set expected_psn
    pub fn expected_psn(&self) -> PacketSequenceNumber {
        self.expected_psn.load(core::sync::atomic::Ordering::SeqCst)
    }

    pub fn set_expect_psn(&self, psn: PacketSequenceNumber) {
        self.expected_psn.store(psn, core::sync::atomic::Ordering::SeqCst);
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

    // TODO(fh): remove `'_` when Rust 2024 edition
    pub fn guard(&self) -> impl papaya::Guard + '_ {
        self.0.guard()
    }

    pub fn get<'guard>(&self, qpn: QueuePairNumber, guard: &'guard impl papaya::Guard) -> Option<&'guard Context> {
        log::debug!("get qp_table with {qpn:?}");

        self.0.get(&qpn, guard)
    }
}
