use core::marker::PhantomData;

use crate::DeviceInner;
use crate::dma::{Client, PointerMut};
use crate::net::Agent;
use crate::queues::command_request::common::Unknown;
use crate::queues::complete_queue::CompleteQueue;

#[derive(Debug)]
pub struct CommandResponseQueue<'q, UA: Agent, DC: Client, Desc = Unknown> {
    dev: &'q DeviceInner<UA, DC>,
    _descriptors: PhantomData<*mut [Desc]>,
}

impl<'q, UA: Agent, DC: Client> CommandResponseQueue<'q, UA, DC> {
    pub const fn new(dev: &'q DeviceInner<UA, DC>) -> Self {
        Self {
            dev,
            _descriptors: PhantomData,
        }
    }
}

impl<UA: Agent, DC: Client, Desc> CompleteQueue for CommandResponseQueue<'_, UA, DC, Desc> {
    type Descriptor = Desc;

    fn addr(&self) -> u64 {
        self.dev.csrs.cmd_response.addr.read()
    }

    fn head(&self) -> u32 {
        self.dev.csrs.cmd_response.head.read()
    }

    fn tail(&self) -> u32 {
        self.dev.csrs.cmd_response.tail.read()
    }

    fn index<T>(&self, index: u32) -> impl PointerMut<Output = T> {
        let addr = self
            .addr()
            .checked_add(u64::from(index) * u64::try_from(size_of::<Self::Descriptor>()).unwrap())
            .unwrap()
            .into();
        self.dev.dma_client.with_dma_addr::<T>(addr)
    }

    fn advance(&self) {
        let old = self.head();
        let val = (old + 1) % 128;
        log::trace!("advance command_response head {old:010x} -> {val:010x}");
        self.dev.csrs.cmd_response.head.write(val);
    }
}

impl<UA: Agent, DC: Client> DeviceInner<UA, DC> {
    pub(crate) const fn command_response_queue(&self) -> CommandResponseQueue<'_, UA, DC> {
        CommandResponseQueue::new(self)
    }
}
