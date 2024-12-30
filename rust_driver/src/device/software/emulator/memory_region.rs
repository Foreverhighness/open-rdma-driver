use papaya::HashMap;

use super::address::{DmaAddress, VirtualAddress};
use super::mr_table::MemoryRegionTable;
use super::types::{MemoryAccessFlag, MemoryRegionKey, ProtectDomainHandler};
use crate::device::software::emulator::mr_table::Error;

#[derive(Debug, PartialEq)]
pub struct Context {
    addr: VirtualAddress,
    len: u32,
    key: MemoryRegionKey,
    protect_domain_handler: ProtectDomainHandler,
    access_flag: MemoryAccessFlag,
    page_table_offset: u32,
}

impl Context {
    pub(crate) const fn new(
        addr: VirtualAddress,
        len: u32,
        key: MemoryRegionKey,
        protect_domain_handler: u32,
        access_flag: MemoryAccessFlag,
        page_table_offset: u32,
    ) -> Self {
        assert!(!addr.0.overflowing_add(len as u64).1);
        Self {
            addr,
            len,
            key,
            protect_domain_handler,
            access_flag,
            page_table_offset,
        }
    }
}

#[derive(Debug, Default)]
pub struct Table(HashMap<MemoryRegionKey, Context>);

impl Table {
    pub(crate) fn new() -> Self {
        Self(HashMap::new())
    }
}

impl MemoryRegionTable for Table {
    fn update(&self, mr_context: Context) -> Result<(), Error> {
        log::debug!("update mr_table with {mr_context:?}");

        let mr_table = self.0.pin();
        let _ = mr_table.insert(mr_context.key, mr_context);

        log::trace!("after update {self:?}");

        Ok(())
    }

    fn query(
        &self,
        key: MemoryRegionKey,
        va: VirtualAddress,
        access_flag: MemoryAccessFlag,
        page_table: &HashMap<u32, Vec<DmaAddress>>,
    ) -> Result<DmaAddress, Error> {
        let mr_table = self.0.pin();
        let mr_context = mr_table.get(&key).ok_or(Error::KeyNotFound(key))?;

        let permit_access_flag = mr_context.access_flag;
        if !permit_access_flag.contains(access_flag) {
            return Err(Error::PermissionDeny {
                give: access_flag,
                permit: permit_access_flag,
            });
        }

        let addr = mr_context.addr;
        let len = mr_context.len;
        // SAFETY: (addr + len) should be checked in `MemoryRegionContext`
        let end = unsafe { addr.0.unchecked_add(len as u64) };
        if !(addr.0 <= va.0 && va.0 < end) {
            return Err(Error::OutOfBound { va, addr, len });
        }

        const PAGE_SIZE: u64 = 2 * 1024 * 1024; // 2MiB
        const PAGE_SIZE_BITS: u64 = PAGE_SIZE.trailing_zeros() as _;
        let idx = (va.0 - addr.0) >> PAGE_SIZE_BITS;
        let addr = (va.0 - addr.0) & (PAGE_SIZE - 1);

        let offset = mr_context.page_table_offset;
        let page_table = page_table.pin();

        // check when construct `MemoryRegionContext`?
        let dma_address = page_table
            .get(&offset)
            .expect("logic error: page table entry not found");

        let dma_address = dma_address
            .get(usize::try_from(idx).unwrap())
            .expect("logic error: page table offset out of bound")
            .0
            .checked_add(addr)
            .unwrap();

        Ok(dma_address.into())
    }
}
