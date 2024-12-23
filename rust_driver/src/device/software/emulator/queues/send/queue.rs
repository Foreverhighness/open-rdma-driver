use core::marker::PhantomData;

use super::descriptors::{Seg0, Seg1, VariableLengthSge, DESCRIPTOR_SIZE};
use super::operations::WriteBuilder;
use crate::device::software::emulator::dma::{Client, PointerMut};
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::queues::send::operations::Opcode;
use crate::device::software::emulator::queues::work_queue::WorkQueue;
use crate::device::software::emulator::DeviceInner;

// SendQueue is same type as RegistersSendHandle
#[derive(Debug)]
pub(crate) struct SendQueue<'q, UA: Agent, DC: Client, Desc = [u8; DESCRIPTOR_SIZE]> {
    dev: &'q DeviceInner<UA, DC>,
    _descriptors: PhantomData<*mut [Desc]>,
}

impl<'q, UA: Agent, DC: Client> SendQueue<'q, UA, DC> {
    pub(crate) fn new(dev: &'q DeviceInner<UA, DC>) -> Self {
        Self {
            dev,
            _descriptors: PhantomData,
        }
    }
}

impl<UA: Agent, DC: Client, Desc> WorkQueue for SendQueue<'_, UA, DC, Desc> {
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

impl<UA: Agent, DC: Client> SendQueue<'_, UA, DC> {
    pub(crate) fn doorbell(&self, _head: u32) {
        self.dev.tx_send.send(()).unwrap();
    }

    pub(crate) fn run(&self) {
        while let Ok(()) = self.dev.rx_send.recv() {
            let raw = unsafe { self.pop() };

            let seg0 = Seg0::from_bytes(raw);
            // TODO(fh): move assertions into `Seg0::from_bytes_checked`.
            assert!(seg0.header.valid());
            log::info!("recv send seg0: {seg0:?}");
            let opcode = seg0.header.opcode().expect("send opcode parse failed");

            match opcode {
                Opcode::Write => {
                    // write use 3 descriptors
                    let builder = WriteBuilder::from_seg0(seg0);

                    let raw = unsafe { self.pop() };
                    let seg1 = Seg1::from_bytes(raw);

                    let builder = builder.with_seg1(seg1);

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

impl<UA: Agent, DC: Client> DeviceInner<UA, DC> {
    pub(crate) fn send_queue(&self) -> SendQueue<'_, UA, DC> {
        SendQueue::new(self)
    }
}
