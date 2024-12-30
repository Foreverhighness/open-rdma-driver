use core::fmt;

use derive_more::derive::{From, Into};

/// Virtual Address, used in memory region
#[derive(Clone, Copy, Default, PartialEq, Eq, From, Into)]
pub struct VirtualAddress(pub u64);

impl fmt::Debug for VirtualAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VA({:#018X})", self.0)
    }
}

/// DMA Address, store in emulator
#[derive(Clone, Copy, Default, PartialEq, Eq, From, Into)]
pub struct DmaAddress(pub u64);

impl fmt::Debug for DmaAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DA({:#018X})", self.0)
    }
}

#[expect(unused, reason = "for explain idea reason, may not used")]
mod physical {
    use derive_more::From;

    use super::DmaAddress;
    /// Physical Address
    #[derive(Clone, Copy, Default, From)]
    pub struct PhysicalAddress(u64);

    pub trait ToPhysicalAddress: Clone + Copy {
        fn to_physical_address(self) -> PhysicalAddress;
    }

    impl ToPhysicalAddress for PhysicalAddress {
        fn to_physical_address(self) -> PhysicalAddress {
            self
        }
    }

    impl ToPhysicalAddress for DmaAddress {
        fn to_physical_address(self) -> PhysicalAddress {
            PhysicalAddress(self.0)
        }
    }

    impl core::fmt::Debug for PhysicalAddress {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PA: {:#018X}", self.0)
        }
    }

    impl PhysicalAddress {
        pub fn into_inner(self) -> u64 {
            self.0
        }
    }
}
