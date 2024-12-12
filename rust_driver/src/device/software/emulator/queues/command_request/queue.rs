use super::common::Unknown;
use crate::device::software::emulator::dma::{Client, PointerMut};
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::command_request::common::DESCRIPTOR_SIZE;
use crate::device::software::emulator::Emulator;

#[derive(Debug)]
pub(crate) struct CommandRequestQueue<'q, UA: Agent> {
    dev: &'q Emulator<UA>,
    // addr: u64,
    // head: u32,
    // tail: u32,
    // dev: Arc<Emulator<UA>>,
}

impl<'q, UA: Agent> CommandRequestQueue<'q, UA> {
    pub(crate) unsafe fn new(dev: &'q Emulator<UA>) -> Self {
        Self { dev }
    }
}

// pub(crate) trait CommandRequestQueue {
//     fn pop(&self) -> Result<()>;
// }

impl<UA: Agent> CommandRequestQueue<'_, UA> {
    // argument `head` is for debugging purpose
    pub(crate) unsafe fn pop(&mut self, head: u32) -> Option<Unknown> {
        let addr = self.dev.csrs.cmd_request.addr.read();
        let read_head = self.dev.csrs.cmd_request.head.read();
        let tail = self.dev.csrs.cmd_request.tail.read();
        assert_eq!(read_head, head);

        if tail == head {
            return None;
        }

        let addr = addr
            .checked_add(u64::from(tail) * u64::try_from(DESCRIPTOR_SIZE).unwrap())
            .unwrap()
            .into();

        let ptr = self.dev.dma_client.new_ptr_mut::<Unknown>(addr);
        let raw = unsafe { ptr.read() };

        // pop item
        self.dev.csrs.cmd_request.tail.write(tail + 1);

        log::trace!("raw descriptor @ {addr:?}[{head}]: {raw:02X?}");

        Some(raw)
    }
}

impl<UA: Agent> Emulator<UA> {
    pub(crate) fn command_request_queue(&self) -> CommandRequestQueue<'_, UA> {
        unsafe { CommandRequestQueue::new(self) }
    }
}
