use core::marker::PhantomData;

use super::descriptors::DESCRIPTOR_SIZE;
use crate::DeviceInner;
use crate::dma::{Client, PointerMut};
use crate::net::Agent;
use crate::queues::complete_queue::CompleteQueue;

#[derive(Debug)]
pub(crate) struct MetaReportQueue<'q, UA: Agent, DC: Client, Desc = [u8; DESCRIPTOR_SIZE]> {
    dev: &'q DeviceInner<UA, DC>,
    _descriptors: PhantomData<*mut [Desc]>,
}

impl<'q, UA: Agent, DC: Client> MetaReportQueue<'q, UA, DC> {
    pub(crate) fn new(dev: &'q DeviceInner<UA, DC>) -> Self {
        Self {
            dev,
            _descriptors: PhantomData,
        }
    }
}

impl<UA: Agent, DC: Client, Desc> CompleteQueue for MetaReportQueue<'_, UA, DC, Desc> {
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
        let val = (old + 1) % 128;
        log::trace!("advance meta_report head {old:010x} -> {val:010x}");
        self.dev.csrs.meta_report.head.write(val);
    }
}

impl<UA: Agent, DC: Client> DeviceInner<UA, DC> {
    pub(crate) fn meta_report_queue(&self) -> MetaReportQueue<'_, UA, DC> {
        MetaReportQueue::new(self)
    }
}
