//! Address type conceptions
//!
//! Each address type should be represented as `usize`, `*const u8`, `*mut u8`, `NonNull<u8>` or
//! `u64`.
//! But this module is just to help with understanding, so I set each type to a unit type.

#![expect(unexpected_cfgs, reason = "Currently I don't have a feature named iommu")]

pub(super) struct CPUVirtualAddress;
pub(super) struct PhysicalAddress;

pub(super) struct DmaAddress;
pub(super) struct IOVirtualAddress;
pub(super) struct BusAddress;

#[cfg(features = "iommu")]
pub(super) type BlueRdmaPhysicalAddress = IOVirtualAddress;
#[cfg(not(features = "iommu"))]
pub(super) type BlueRdmaPhysicalAddress = BusAddress;

/// Hardware address
///
/// Used to access physical memory, but not relay on `fn to_physical_address`.
/// It can be translate to physical address by the hardware(bypass CPU) support. (or kernel support?
/// which not bypass CPU).
/// It may stored in `QueuePairContext` etc.
///
/// One could even impl HardwareAddress for CPUVirtualAddress for testing?
pub(super) trait HardwareAddress {
    /// `HardwareAddress` can be translate into physical address.
    #[cfg(test)]
    fn to_physical_address(self) -> PhysicalAddress;
}

impl HardwareAddress for PhysicalAddress {
    #[cfg(test)]
    fn to_physical_address(self) -> PhysicalAddress {
        self
    }
}
impl HardwareAddress for DmaAddress {
    #[cfg(test)]
    fn to_physical_address(self) -> PhysicalAddress {
        unimplemented!()
    }
}
impl HardwareAddress for IOVirtualAddress {
    #[cfg(test)]
    fn to_physical_address(self) -> PhysicalAddress {
        unimplemented!()
    }
}
impl HardwareAddress for BusAddress {
    #[cfg(test)]
    fn to_physical_address(self) -> PhysicalAddress {
        unimplemented!()
    }
}
