use core::marker::PhantomData;

use super::common::DESCRIPTOR_SIZE;
use super::descriptors::{Seg0, Seg1, VariableLengthSge};
use crate::device::software::emulator::dma::{Client, PointerMut};
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
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
        self.dev.tx_send.send(()).unwrap();
    }

    pub(crate) fn run(&self) {
        let mut state = State::ExpectSeg0;
        let mut builder = Builder::new();
        while let Ok(()) = self.dev.rx_send.recv() {
            let raw = unsafe { self.pop() };

            state = match state {
                State::ExpectSeg0 => {
                    let req = Seg0::from_bytes(raw);
                    self.dev.handle(&req, &mut builder).unwrap();

                    State::ExpectSeg1
                }
                State::ExpectSeg1 => {
                    let req = Seg1::from_bytes(raw);
                    self.dev.handle(&req, &mut builder).unwrap();

                    State::ExpectVariableLenSge
                }
                State::ExpectVariableLenSge => {
                    let req = VariableLengthSge::from_bytes(raw);
                    self.dev.handle(&req, &mut builder).unwrap();

                    if let Some(op) = builder.try_build() {
                        State::ExpectSeg0
                    } else {
                        State::ExpectVariableLenSge
                    }
                }
            }
        }
    }
}

impl<UA: Agent> Emulator<UA> {
    pub(crate) fn send_queue(&self) -> SendQueue<'_, UA> {
        SendQueue::new(self)
    }
}

#[derive(Debug)]
pub(crate) struct Builder {}

impl Builder {
    fn new() -> Self {
        todo!()
    }

    fn try_build(&self) -> Option<i32> {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) enum State {
    ExpectSeg0,
    ExpectSeg1,
    ExpectVariableLenSge,
    // Failed,
}
