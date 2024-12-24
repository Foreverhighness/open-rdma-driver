use core::marker::PhantomData;

use super::descriptors::DESCRIPTOR_SIZE;
use crate::device::software::emulator::dma::{Client, PointerMut};
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::complete_queue::CompleteQueue;
use crate::device::software::emulator::DeviceInner;

#[derive(Debug)]
pub(crate) struct MetaReportQueue<'q, UA: Agent, Desc = [u8; DESCRIPTOR_SIZE]> {
    dev: &'q DeviceInner<UA>,
    _descriptors: PhantomData<*mut [Desc]>,
}

impl<'q, UA: Agent> MetaReportQueue<'q, UA> {
    pub(crate) fn new(dev: &'q DeviceInner<UA>) -> Self {
        Self {
            dev,
            _descriptors: PhantomData,
        }
    }
}

impl<UA: Agent, Desc> CompleteQueue for MetaReportQueue<'_, UA, Desc> {
    type Descriptor = Desc;

    fn addr(&self) -> u64 {
        self.dev.csrs.meta_report.addr.read()
    }

    fn head(&self) -> u32 {
        self.dev.csrs.meta_report.head.read()
    }

    fn tail(&self) -> u32 {
        self.dev.csrs.meta_report.tail.read()
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
        let val = old + 1;
        log::trace!("advance meta_report head {old:010x} -> {val:010x}");
        self.dev.csrs.meta_report.head.write(val);
    }
}

impl<UA: Agent> DeviceInner<UA> {
    pub(crate) fn meta_report_queue(&self) -> MetaReportQueue<'_, UA> {
        MetaReportQueue::new(self)
    }
}
