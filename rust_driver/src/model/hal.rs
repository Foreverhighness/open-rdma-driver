//! Hardware Abstract Layer
//! Inspired by <https://github.com/KuangjuX/ixgbe-driver/blob/main/src/hal.rs>
//!
//! This module is intend to hide different platform distinctions, e.g. `linux`, `ArceOS`.
//! For conceptional model, we may not need this.

use super::address::{BlueRdmaPhysicalAddress, CPUVirtualAddress, HardwareAddress};
pub(super) unsafe trait KernelFunctional<HA: HardwareAddress = BlueRdmaPhysicalAddress>
where
    Self: Sized,
{
    fn dma_alloc(size: usize) -> (HA, CPUVirtualAddress);
    unsafe fn dma_dealloc(paddr: BlueRdmaPhysicalAddress, vaddr: CPUVirtualAddress, size: usize);
    unsafe fn mmio_phys_to_virt(paddr: BlueRdmaPhysicalAddress, size: usize) -> CPUVirtualAddress;
    unsafe fn mmio_virt_to_phys(vaddr: CPUVirtualAddress, size: usize) -> BlueRdmaPhysicalAddress;

    // /// Wait until reaching the given deadline.
    // fn wait_until(duration: Duration) -> Result<(), &'static str>;
}
