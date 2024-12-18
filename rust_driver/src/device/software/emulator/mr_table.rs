use super::address::{DmaAddress, VirtualAddress};
use super::memory_region::Context;
use super::Result;

pub trait MemoryRegionTable {
    fn update(&self, mr_context: Context) -> Result<()>;

    fn query(&self, va: VirtualAddress) -> Result<DmaAddress>;
}
