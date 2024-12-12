pub mod descriptors;

pub(super) mod common;
mod opcode;

use common::Unknown;
use descriptors::DescriptorRef;

use crate::device::software::emulator::device_api::csr::{RegistersQueue, RegistersQueueAddress};
use crate::device::software::emulator::device_api::{ControlStatusRegisters, RawDevice};
use crate::device::software::emulator::dma::{Client, PointerMut};
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queue::descriptor::HandleDescriptor;
use crate::device::software::emulator::Emulator;

pub trait CommandRequestQueueAbility: RawDevice {
    /// Notify device there is new descriptor, should not blocking
    fn doorbell(&self, head: u32);
}

impl<UA: Agent> CommandRequestQueueAbility for Emulator<UA> {
    fn doorbell(&self, head: u32) {
        let base_addr = self.csrs().cmd_request().addr().read();

        let addr = base_addr.into();

        let ptr = self.dma_client.new_ptr_mut::<Unknown>(addr);
        let raw = unsafe { ptr.read() };

        log::trace!("raw descriptor @ {addr:?}[{head}]: {raw:02X?}");

        let descriptor_ref = dbg!(DescriptorRef::parse(&raw).unwrap());

        match descriptor_ref {
            DescriptorRef::UpdateMemoryRegionTable(_) => todo!(),
            DescriptorRef::UpdatePageTable(descriptor) => self.handle(descriptor).unwrap(),
            DescriptorRef::QueuePairManagement(_) => todo!(),
            DescriptorRef::SetNetworkParameter(_) => todo!(),
            DescriptorRef::SetRawPacketReceiveMeta(_) => todo!(),
            DescriptorRef::UpdateErrorPacketSequenceNumberRecoverPoint(_) => todo!(),
        }
    }
}
