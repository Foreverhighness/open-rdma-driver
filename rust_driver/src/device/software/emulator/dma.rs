//! Direct Memory Access, I only concern DMA in emulator

use derive_more::{From, Into};

/// DMA Address, store in emulator
#[derive(Clone, Copy, Default, From, Into)]
pub struct DmaAddress(u64);

impl core::fmt::Debug for DmaAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DA({:#018X})", self.0)
    }
}

pub trait Client {
    // type Ref;
    // type RefMut
    // type Ptr;
    // Currently use only one ptr type for simplicity
    type PtrMut<'a, T>: PointerMut<Output = T>
    where
        Self: 'a;

    fn new_ptr_mut<T>(&self, addr: DmaAddress) -> Self::PtrMut<'_, T>;
}

pub trait PointerMut: Clone + Copy {
    type Output;
    unsafe fn read(self) -> Self::Output;
    unsafe fn write(self, val: Self::Output);
}

// May not use
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
