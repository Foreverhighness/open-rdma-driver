use core::marker::PhantomData;

use super::common::DESCRIPTOR_SIZE;
use crate::device::software::emulator::dma::{Client, PointerMut};
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::queues::send::descriptors::DescriptorRef;
use crate::device::software::emulator::queues::work_queue::WorkQueue;
use crate::device::software::emulator::Emulator;

// SendQueue is same type as RegistersSendHandle
#[derive(Debug)]
pub(crate) struct SendQueue<'q, UA: Agent, Desc = [u8; DESCRIPTOR_SIZE]> {
    dev: &'q Emulator<UA>,
    _descriptors: PhantomData<*mut [Desc]>,
}

impl<'q, UA: Agent> SendQueue<'q, UA> {
    pub(crate) fn new(dev: &'q Emulator<UA>) -> Self {
        Self {
            dev,
            _descriptors: PhantomData,
        }
    }
}

impl<UA: Agent, Desc> WorkQueue for SendQueue<'_, UA, Desc> {
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

impl<UA: Agent> SendQueue<'_, UA> {
    pub(crate) fn doorbell(&self, _head: u32) {
        let raw = unsafe { self.pop() };

        let descriptor_ref = DescriptorRef::parse(&raw).unwrap();

        // match descriptor_ref {
        //     DescriptorRef::Seg0(req) => self.dev.handle(req).unwrap(),
        //     DescriptorRef::Seg1(req) => self.dev.handle(req).unwrap(),
        //     DescriptorRef::VariableLengthSGE(req) => self.dev.handle(req).unwrap(),
        // }
        todo!()
    }
}

impl<UA: Agent> Emulator<UA> {
    pub(crate) fn send_queue(&self) -> SendQueue<'_, UA> {
        SendQueue::new(self)
    }
}
