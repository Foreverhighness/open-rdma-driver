use core::fmt::Debug;
use core::marker::PhantomData;

use super::common::Unknown;
use super::descriptors::DescriptorRef;
use crate::device::software::emulator::dma::{Client, PointerMut};
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::queues::work_queue::WorkQueue;
use crate::device::software::emulator::Emulator;

// CommandRequestQueue is same type as RegistersCommandRequestHandle
#[derive(Debug)]
pub(crate) struct CommandRequestQueue<'q, UA: Agent, Desc = Unknown> {
    dev: &'q Emulator<UA>,
    _descriptors: PhantomData<*mut [Desc]>,
    // addr: u64,
    // head: u32,
    // tail: u32,
    // dev: Arc<Emulator<UA>>,
}

impl<'q, UA: Agent> CommandRequestQueue<'q, UA> {
    pub(crate) fn new(dev: &'q Emulator<UA>) -> Self {
        Self {
            dev,
            _descriptors: PhantomData,
        }
    }
}

impl<UA: Agent, Desc> WorkQueue for CommandRequestQueue<'_, UA, Desc> {
    type Descriptor = Desc;

    fn addr(&self) -> u64 {
        self.dev.csrs.cmd_request.addr.read()
    }

    fn head(&self) -> u32 {
        self.dev.csrs.cmd_request.head.read()
    }

    fn tail(&self) -> u32 {
        self.dev.csrs.cmd_request.tail.read()
    }

    fn index(&self, index: u32) -> impl PointerMut<Output = Self::Descriptor> {
        let addr = self
            .addr()
            .checked_add(u64::from(index) * u64::try_from(size_of::<Self::Descriptor>()).unwrap())
            .unwrap()
            .into();

        self.dev.dma_client.with_addr::<Self::Descriptor>(addr)
    }

    fn advance(&self) {
        self.dev.csrs.cmd_request.tail.write(self.tail() + 1);
    }
}

impl<UA: Agent> CommandRequestQueue<'_, UA> {
    pub(crate) fn doorbell(&self, _head: u32) {
        let raw = unsafe { self.pop() };

        let descriptor_ref = DescriptorRef::parse(&raw).unwrap();

        match descriptor_ref {
            DescriptorRef::UpdateMemoryRegionTable(req) => self.dev.handle(req).unwrap(),
            DescriptorRef::UpdatePageTable(req) => self.dev.handle(req).unwrap(),
            DescriptorRef::QueuePairManagement(req) => self.dev.handle(req).unwrap(),
            DescriptorRef::SetNetworkParameter(req) => self.dev.handle(req).unwrap(),
            DescriptorRef::SetRawPacketReceiveMeta(req) => self.dev.handle(req).unwrap(),
            DescriptorRef::UpdateErrorPacketSequenceNumberRecoverPoint(req) => self.dev.handle(req).unwrap(),
        }
    }
}

impl<UA: Agent> Emulator<UA> {
    pub(crate) fn command_request_queue(&self) -> CommandRequestQueue<'_, UA> {
        CommandRequestQueue::new(self)
    }
}
