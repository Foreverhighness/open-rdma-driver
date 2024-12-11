pub mod descriptors;

mod common;
mod opcode;

use crate::device::software::emulator::device_api::RawDevice;
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::Emulator;

pub trait CommandRequestQueueAbility: RawDevice {
    /// Notify device there is new descriptor, should not blocking
    fn doorbell(&self, head: u32);
}

impl<UA: Agent> CommandRequestQueueAbility for Emulator<UA> {
    fn doorbell(&self, head: u32) {
        todo!()
    }
}
