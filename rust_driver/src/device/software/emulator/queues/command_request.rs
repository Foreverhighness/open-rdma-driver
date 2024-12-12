pub mod descriptors;

pub(super) mod common;
mod opcode;
pub(super) mod queue;

use descriptors::DescriptorRef;

use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::Emulator;

pub trait CommandRequestQueueAbility {
    /// Notify device there is new descriptor, should not blocking
    fn doorbell(&self, head: u32);
}

impl<UA: Agent> CommandRequestQueueAbility for Emulator<UA> {
    fn doorbell(&self, head: u32) {
        let raw = unsafe { self.command_request_queue().pop(head).unwrap() };

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
