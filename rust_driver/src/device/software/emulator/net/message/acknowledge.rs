use super::super::Error;
use super::HandleMessage;
use crate::device::software::emulator::dma::Client;
use crate::device::software::emulator::net::{util, Agent};
use crate::device::software::emulator::queues::complete_queue::CompleteQueue;
use crate::device::software::emulator::DeviceInner;
use crate::device::software::types::RdmaMessage;

#[derive(Debug, Clone, Copy)]
pub struct Acknowledge<'msg> {
    // TODO(fh): replace with BaseTransportHeader
    bth: &'msg RdmaMessage,
    aeth: &'msg RdmaMessage,
}
type Message<'msg> = Acknowledge<'msg>;

impl<'msg> Message<'msg> {
    pub const fn parse<'input>(msg: &'input RdmaMessage) -> Result<Self, Error>
    where
        'input: 'msg,
    {
        Ok(Self { bth: msg, aeth: msg })
    }
}

impl<UA: Agent, DC: Client> HandleMessage<Message<'_>> for DeviceInner<UA, DC> {
    fn handle(&self, msg: Acknowledge) -> crate::device::software::emulator::Result {
        let descriptor = util::message_to_bthaeth(msg.aeth);

        log::debug!("push meta report: {descriptor:#?}");
        unsafe { self.meta_report_queue().push(descriptor) };

        Ok(())
    }
}
