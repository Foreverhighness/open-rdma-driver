use super::HandleMessage;
use crate::device::software::emulator::address::VirtualAddress;
use crate::device::software::emulator::dma::{Client, PointerMut};
use crate::device::software::emulator::mr_table::MemoryRegionTable;
use crate::device::software::emulator::net::util::message_to_bthreth;
use crate::device::software::emulator::net::{Agent, Error};
use crate::device::software::emulator::queues::complete_queue::CompleteQueue;
use crate::device::software::emulator::DeviceInner;
use crate::device::software::types::{Metadata, RdmaMessage};

#[derive(Debug, Clone, Copy)]
pub(crate) struct WriteLast<'msg> {
    // TODO(fh): replace with BaseTransportHeader
    bth: &'msg RdmaMessage,
    reth: &'msg RdmaMessage,
}

type Message<'a> = WriteLast<'a>;

impl<'msg> WriteLast<'msg> {
    pub const fn parse<'input>(msg: &'input RdmaMessage) -> Result<Self, Error>
    where
        'input: 'msg,
    {
        Ok(Self { bth: msg, reth: msg })
    }
}

impl<UA: Agent> HandleMessage<Message<'_>> for DeviceInner<UA> {
    fn handle(&self, msg: &Message) -> crate::device::software::emulator::Result {
        let msg = msg.bth;
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

            let dma_addr = self
                .memory_region_table()
                .query(key, va, access_flag, &self.page_table)
                .expect("validation failed");

            let ptr = self.dma_client.with_dma_addr::<u8>(dma_addr);
            unsafe { ptr.copy_from_nonoverlapping(data.data, data.len) };
        }

        let descriptor = message_to_bthreth(msg);
        log::debug!("push meta report: {descriptor:?}");
        unsafe { self.meta_report_queue().push(descriptor) };
        Ok(())
    }
}
