use crate::device::software::emulator::device_api::RawDevice;
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::Emulator;

pub trait CommandRequestQueueAbility: RawDevice {
    fn new_command_request(&self, head: u32);
}

impl<UA: Agent> CommandRequestQueueAbility for Emulator<UA> {
    fn new_command_request(&self, head: u32) {
        todo!()
    }
}
