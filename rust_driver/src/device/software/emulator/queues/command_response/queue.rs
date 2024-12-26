use core::marker::PhantomData;

use crate::device::software::emulator::dma::{Client, PointerMut};
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::command_request::common::Unknown;
use crate::device::software::emulator::queues::complete_queue::CompleteQueue;
use crate::device::software::emulator::DeviceInner;

#[derive(Debug)]
pub(crate) struct CommandResponseQueue<'q, UA: Agent, DC: Client, Desc = Unknown> {
    dev: &'q DeviceInner<UA, DC>,
    _descriptors: PhantomData<*mut [Desc]>,
}

impl<'q, UA: Agent, DC: Client> CommandResponseQueue<'q, UA, DC> {
    pub(crate) fn new(dev: &'q DeviceInner<UA, DC>) -> Self {
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
    pub(crate) fn command_response_queue(&self) -> CommandResponseQueue<'_, UA, DC> {
        CommandResponseQueue::new(self)
    }
}
