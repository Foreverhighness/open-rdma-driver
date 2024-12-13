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

    fn with_addr<T>(&self, addr: DmaAddress) -> impl PointerMut<Output = T>;
}

pub trait PointerMut: Clone + Copy {
    type Output;

    /// Reads the value from `self` without moving it. This leaves the
    /// memory in `self` unchanged.
    ///
    /// See [`core::ptr::read`] for safety concerns and examples.
    unsafe fn read(self) -> Self::Output;

    /// Overwrites a memory location with the given value without reading or
    /// dropping the old value.
    ///
    /// See [`core::ptr::write`] for safety concerns and examples.
    unsafe fn write(self, val: Self::Output);

    /// Copies `count * size_of<T>` bytes from `src` to `self`. The source
    /// and destination may *not* overlap.
    ///
    /// NOTE: this has the *opposite* argument order of [`core::ptr::copy_nonoverlapping`].
    ///
    /// See [`core::ptr::copy_nonoverlapping`] for safety concerns and examples.
    unsafe fn copy_nonoverlapping(self, src: *const Self::Output, count: usize);
}

// For zero copy, may not use
pub trait PointerMutExt: PointerMut {
    unsafe fn with<F, T>(self, f: F) -> T
    where
        F: FnOnce(&Self::Output) -> T;

    unsafe fn with_mut<F, T>(self, f: F) -> T
    where
        F: FnOnce(&mut Self::Output) -> T;
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
