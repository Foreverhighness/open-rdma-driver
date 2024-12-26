//! Secondary RDMA Extended Transport Header

use super::common::SecondaryRdmaExtendedTransportHeader;
use super::{DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};
use crate::device::software::emulator::address::VirtualAddress;
use crate::device::software::emulator::types::MemoryRegionKey;

#[derive(Debug)]
#[repr(C, align(32))]
pub struct SecondaryReth {
    secondary_reth: SecondaryRdmaExtendedTransportHeader,
    _reserved: core::mem::MaybeUninit<[u8; 16]>,
}
type Descriptor = SecondaryReth;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);

impl SecondaryReth {
    pub const fn new(addr: VirtualAddress, local_key: MemoryRegionKey) -> Self {
        Self {
            secondary_reth: SecondaryRdmaExtendedTransportHeader::new(addr, local_key),
            _reserved: core::mem::MaybeUninit::uninit(),
        }
    }
}
