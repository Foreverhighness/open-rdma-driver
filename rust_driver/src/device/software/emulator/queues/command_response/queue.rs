use super::DESCRIPTOR_SIZE;
use crate::device::software::emulator::dma::{Client, PointerMut};
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::Emulator;

#[derive(Debug)]
pub(crate) struct CommandResponseQueue<'q, UA: Agent> {
    dev: &'q Emulator<UA>,
}

impl<'q, UA: Agent> CommandResponseQueue<'q, UA> {
    pub(crate) fn new(dev: &'q Emulator<UA>) -> Self {
        Self { dev }
    }
}

impl<UA: Agent> CommandResponseQueue<'_, UA> {
    pub(crate) unsafe fn push<T>(&self, response: T) {
        const { assert!(size_of::<T>() <= DESCRIPTOR_SIZE) };

        let addr = self.dev.csrs.cmd_response.addr.read();
        let head = self.dev.csrs.cmd_response.head.read();
        let tail = self.dev.csrs.cmd_response.tail.read();
        assert!(tail <= head);

        let addr = addr
            .checked_add(u64::from(head) * u64::try_from(DESCRIPTOR_SIZE).unwrap())
            .unwrap()
            .into();
        let ptr = self.dev.dma_client.with_addr::<T>(addr);
        // Safety: src and dst is valid
        unsafe {
            ptr.write(response);
        }

        self.dev.csrs.cmd_response.head.write(head + 1);
    }
}

impl<UA: Agent> Emulator<UA> {
    pub(crate) fn command_response_queue(&self) -> CommandResponseQueue<'_, UA> {
        CommandResponseQueue::new(self)
    }
}
