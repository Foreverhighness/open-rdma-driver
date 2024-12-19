use super::address::{DmaAddress, VirtualAddress};
use super::memory_region::Context;
use super::types::{MemoryAccessFlag, MemoryRegionKey};

pub trait MemoryRegionTable {
    fn update(&self, mr_context: Context) -> Result<(), Error>;
    // fn update(&self, key: MemoryRegionKey, mr_context: Context) -> Result<()>;

    fn query(
        &self,
        key: MemoryRegionKey,
        va: VirtualAddress,
        access_flag: MemoryAccessFlag,
        // TODO(fh): Replace with generic args <PageTable>
        page_table: &papaya::HashMap<u32, Vec<DmaAddress>>,
    ) -> Result<DmaAddress, Error>;
}

#[expect(variant_size_differences, reason = "TODO(fh): move into Box")]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("key not found: {0:?}")]
    KeyNotFound(MemoryRegionKey),

    #[error("permission error: give {give:?}, permit {permit:?}")]
    PermissionDeny {
        give: MemoryAccessFlag,
        permit: MemoryAccessFlag,
    },

    #[error("virtual address out of bound: {va:?} not within (addr: {addr:?}, len: {len:#010X})")]
    OutOfBound {
        va: VirtualAddress,
        addr: VirtualAddress,
        len: u32,
    },
}
