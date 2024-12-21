use core::marker::PhantomData;

use super::descriptors::{Seg0, Seg1, VariableLengthSge, DESCRIPTOR_SIZE};
use super::operations::WriteBuilder;
use crate::device::software::emulator::dma::{Client, PointerMut};
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::queues::send::operations::Opcode;
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
        self.dev.csrs.send.addr.read()
    }

    fn head(&self) -> u32 {
        self.dev.csrs.send.head.read()
    }

    fn tail(&self) -> u32 {
        self.dev.csrs.send.tail.read()
    }

    fn index(&self, index: u32) -> impl PointerMut<Output = Self::Descriptor> {
        let addr = self
            .addr()
            .checked_add(u64::from(index) * u64::try_from(size_of::<Self::Descriptor>()).unwrap())
            .unwrap()
            .into();

        self.dev.dma_client.with_dma_addr::<Self::Descriptor>(addr)
    }

    fn advance(&self) {
        let old = self.tail();
        let val = old + 1;
        log::trace!("advance send tail {old:010x} -> {val:010x}");
        self.dev.csrs.send.tail.write(self.tail() + 1);
    }
}

impl<UA: Agent> SendQueue<'_, UA> {
    pub(crate) fn doorbell(&self, _head: u32) {
        self.dev.tx_send.send(()).unwrap();
    }

    pub(crate) fn run(&self) {
        while let Ok(()) = self.dev.rx_send.recv() {
            let raw = unsafe { self.pop() };

            let seg0 = Seg0::from_bytes(raw);
            // TODO(fh): move assertions into `Seg0::from_bytes_checked`.
            assert!(seg0.header.valid());
            let opcode = seg0.header.opcode().expect("send opcode parse failed");

            match opcode {
                Opcode::Write => {
                    // write use 3 descriptors
                    let builder = WriteBuilder::from_seg0(seg0);

                    self.dev.rx_send.recv().expect("send recv failed");
                    let raw = unsafe { self.pop() };
                    let seg1 = Seg1::from_bytes(raw);

                    let builder = builder.with_seg1(seg1);

                    self.dev.rx_send.recv().expect("send recv failed");
                    let raw = unsafe { self.pop() };
                    let sge = VariableLengthSge::from_bytes(raw);

                    let write_req = builder.with_sge(sge);

                    self.dev.handle(&write_req, &mut ()).unwrap();
                }
                Opcode::WriteWithImm => todo!(),
                Opcode::Read => todo!(),
                Opcode::ReadResp => todo!(),
            }
        }
    }
}

impl<UA: Agent> Emulator<UA> {
    pub(crate) fn send_queue(&self) -> SendQueue<'_, UA> {
        SendQueue::new(self)
    }
}
