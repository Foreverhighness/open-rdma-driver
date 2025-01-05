use super::super::Error;
use super::HandleMessage;
use crate::DeviceInner;
use crate::dma::Client;
use crate::net::{Agent, util};
use crate::queues::complete_queue::CompleteQueue;
use crate::third_party::net::RdmaMessage;

#[expect(dead_code, reason = "need refactor")]
#[derive(Debug)]
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
    fn handle(&self, msg: Acknowledge, _: core::net::IpAddr) -> crate::Result {
        let descriptor = util::message_to_bthaeth(msg.aeth);

        log::debug!("push meta report: {descriptor:?}");
        unsafe { self.meta_report_queue().push(descriptor) };

        Ok(())
    }
}
