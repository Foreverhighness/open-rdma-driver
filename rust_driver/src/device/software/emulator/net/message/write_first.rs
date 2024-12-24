use core::net::Ipv4Addr;

use super::Message;
use crate::device::software::emulator::address::VirtualAddress;
use crate::device::software::emulator::dma::{Client, PointerMut};
use crate::device::software::emulator::mr_table::MemoryRegionTable;
use crate::device::software::emulator::net::util::{generate_ack, message_to_bthreth};
use crate::device::software::emulator::net::{Agent, Error};
use crate::device::software::emulator::queues::complete_queue::CompleteQueue;
use crate::device::software::emulator::DeviceInner;
use crate::device::software::types::{Metadata, RdmaMessage};
use crate::device::ToHostWorkRbDescOpcode;

#[derive(Debug, Clone, Copy)]
pub(crate) struct WriteFirst<'msg> {
    // TODO(fh): replace with BaseTransportHeader
    bth: &'msg RdmaMessage,
    reth: &'msg RdmaMessage,
}

impl<'msg> WriteFirst<'msg> {
    pub const fn parse<'input>(msg: &'input RdmaMessage) -> Result<Self, Error>
    where
        'input: 'msg,
    {
        Ok(Self { bth: msg, reth: msg })
    }
}

impl<UA: Agent, DC: Client> Message<DeviceInner<UA, DC>> for WriteFirst<'_> {
    fn handle(&self, dev: &DeviceInner<UA, DC>) -> crate::device::software::emulator::Result {
        let msg = self.bth;
        // TODO(fh): dma part
        {
            let data = &msg.payload.sg_list;
            assert_eq!(data.len(), 1, "currently only consider one Sge");
            let data = data[0];

            let Metadata::General(ref header) = msg.meta_data else {
                panic!("currently only consider write first and write last packet");
            };
            let key = header.reth.rkey.get().into();
            let va = VirtualAddress(header.reth.va);
            let access_flag = header.needed_permissions();

            let dma_addr = dev
                .memory_region_table()
                .query(key, va, access_flag, &dev.page_table)
                .expect("validation failed");

            let ptr = dev.dma_client.with_dma_addr::<u8>(dma_addr);
            unsafe { ptr.copy_from_nonoverlapping(data.data, data.len) };
        }

        let descriptor = message_to_bthreth(msg);
        log::debug!("push meta report: {descriptor:?}");
        unsafe { dev.meta_report_queue().push(descriptor) };
        Ok(())
    }
}
