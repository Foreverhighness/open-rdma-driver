use super::HandleMessage;
use crate::device::software::emulator::dma::Client;
use crate::device::software::emulator::net::util::{message_to_bthreth, message_to_secondary_reth};
use crate::device::software::emulator::net::{Agent, Error};
use crate::device::software::emulator::queues::complete_queue::CompleteQueue;
use crate::device::software::emulator::DeviceInner;
use crate::device::software::types::RdmaMessage;

#[derive(Debug)]
pub struct ReadRequest<'msg> {
    // TODO(fh): replace with BaseTransportHeader
    bth: &'msg RdmaMessage,
    reth: &'msg RdmaMessage,
    secondary_reth: &'msg RdmaMessage,
}
type Message<'msg> = ReadRequest<'msg>;

impl<'msg> Message<'msg> {
    pub const fn parse<'input>(msg: &'input RdmaMessage) -> Result<Self, Error>
    where
        'input: 'msg,
    {
        Ok(Self {
            bth: msg,
            reth: msg,
            secondary_reth: msg,
        })
    }
}

impl<UA: Agent, DC: Client> HandleMessage<Message<'_>> for DeviceInner<UA, DC> {
    fn handle(&self, msg: Message) -> crate::device::software::emulator::Result {
        let msg = msg.bth;

        let descriptor0 = message_to_bthreth(msg);
        log::debug!("push meta report: {descriptor0:?}");
        unsafe { self.meta_report_queue().push(descriptor0) };

        let descriptor1 = message_to_secondary_reth(msg);
        log::debug!("push meta report: {descriptor1:?}");
        unsafe { self.meta_report_queue().push(descriptor1) };

        Ok(())
    }
}
