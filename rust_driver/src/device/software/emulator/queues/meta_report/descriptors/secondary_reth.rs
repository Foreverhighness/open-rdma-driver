//! Secondary RDMA Extended Transport Header

use super::common::SecondaryRdmaExtendedTransportHeader;
use super::{DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};

#[repr(C, align(32))]
struct SecondaryReth {
    secondary_reth: SecondaryRdmaExtendedTransportHeader,
    _reserved: [bool; 16],
}
type Descriptor = SecondaryReth;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);
