use papaya::HashMap;

use super::address::VirtualAddress;
use super::mr_table::MemoryRegionTable;

pub(crate) type AccessFlag = crate::types::MemAccessTypeFlag;
pub(crate) type Key = crate::types::Key;

#[derive(Debug, PartialEq)]
pub struct Context {
    addr: VirtualAddress,
    len: u32,
    key: Key,
    protect_domain_handler: u32,
    access_flag: AccessFlag,
    page_table_offset: u32,
}

impl Context {
    pub(crate) fn new(
        addr: VirtualAddress,
        len: u32,
        key: Key,
        protect_domain_handler: u32,
        access_flag: AccessFlag,
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
pub struct Table(HashMap<Key, Context>);

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

        log::trace!("after update {self:?}");
        Ok(())
    }
}
