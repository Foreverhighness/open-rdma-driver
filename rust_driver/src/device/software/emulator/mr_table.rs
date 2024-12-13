use super::memory_region::Context;
use super::Result;

pub trait MemoryRegionTable {
    fn update(&self, mr_context: Context) -> Result<()>;
}
