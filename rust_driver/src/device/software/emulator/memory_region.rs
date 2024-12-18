use papaya::HashMap;

use super::address::VirtualAddress;
use super::mr_table::MemoryRegionTable;
use super::types::{MemoryAccessFlag, MemoryRegionKey, ProtectDomainHandler};

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
    pub(crate) fn new(
        addr: VirtualAddress,
        len: u32,
        key: MemoryRegionKey,
        protect_domain_handler: u32,
        access_flag: MemoryAccessFlag,
        page_table_offset: u32,
    ) -> Self {
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
    fn update(&self, mr_context: Context) -> super::Result<()> {
        log::debug!("update mr_table with {mr_context:?}");

        let mr_table = self.0.pin();
        let _ = mr_table.insert(mr_context.key, mr_context);

        log::trace!("after update {self:#?}");

        Ok(())
    }

    fn query(&self, va: VirtualAddress) -> super::Result<super::address::DmaAddress> {
        todo!()
    }
}
