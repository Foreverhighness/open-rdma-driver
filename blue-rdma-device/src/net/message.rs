mod acknowledge;
mod read_request;
mod read_response_first;
mod read_response_last;
mod read_response_middle;
mod read_response_only;
mod write_first;
mod write_last;
mod write_last_with_immediate;
mod write_middle;
mod write_only;
mod write_only_with_immediate;

pub use acknowledge::Acknowledge;
pub use read_request::ReadRequest;
pub use read_response_first::ReadResponseFirst;
pub use read_response_last::ReadResponseLast;
pub use read_response_middle::ReadResponseMiddle;
pub use read_response_only::ReadResponseOnly;
pub use write_first::WriteFirst;
pub use write_last::WriteLast;
pub use write_last_with_immediate::WriteLastWithImmediate;
pub use write_middle::WriteMiddle;
pub use write_only::WriteOnly;
pub use write_only_with_immediate::WriteOnlyWithImmediate;

#[expect(
    unused,
    reason = "This trait is intended for comparison with the `HandleMessage<Msg>` trait."
)]
pub(crate) trait Message<Dev> {
    fn handle(&self, dev: &Dev) -> super::super::Result;
}

// which is better?
pub(crate) trait HandleMessage<Msg> {
    fn handle(&self, msg: Msg, src: core::net::IpAddr) -> super::super::Result;
}

mod handler {
    use super::*;
    use crate::DeviceInner;
    use crate::dma::Client;
    use crate::errors::Error;
    use crate::net::Agent;
    use crate::third_party::net::RdmaMessage;
    use crate::third_party::queues::meta_report::ToHostWorkRbDescOpcode;

    impl<UA: Agent, DC: Client> DeviceInner<UA, DC> {
        pub(crate) fn handle_message(&self, msg: &RdmaMessage, src: core::net::IpAddr) -> Result<(), Error> {
            log::debug!("handle network message {msg:#?}");
            match msg.meta_data.common_meta().opcode {
                ToHostWorkRbDescOpcode::RdmaWriteFirst => self.handle(WriteFirst::parse(msg)?, src)?,
                ToHostWorkRbDescOpcode::RdmaWriteMiddle => self.handle(WriteMiddle::parse(msg)?, src)?,
                ToHostWorkRbDescOpcode::RdmaWriteLast => self.handle(WriteLast::parse(msg)?, src)?,
                ToHostWorkRbDescOpcode::RdmaWriteLastWithImmediate => {
                    self.handle(WriteLastWithImmediate::parse(msg)?, src)?
                }
                ToHostWorkRbDescOpcode::RdmaWriteOnly => self.handle(WriteOnly::parse(msg)?, src)?,
                ToHostWorkRbDescOpcode::RdmaWriteOnlyWithImmediate => {
                    self.handle(WriteOnlyWithImmediate::parse(msg)?, src)?
                }
                ToHostWorkRbDescOpcode::RdmaReadResponseFirst => self.handle(ReadResponseFirst::parse(msg)?, src)?,
                ToHostWorkRbDescOpcode::RdmaReadResponseMiddle => self.handle(ReadResponseMiddle::parse(msg)?, src)?,
                ToHostWorkRbDescOpcode::RdmaReadResponseLast => self.handle(ReadResponseLast::parse(msg)?, src)?,
                ToHostWorkRbDescOpcode::RdmaReadResponseOnly => self.handle(ReadResponseOnly::parse(msg)?, src)?,

                ToHostWorkRbDescOpcode::RdmaReadRequest => self.handle(ReadRequest::parse(msg)?, src)?,
                ToHostWorkRbDescOpcode::Acknowledge => self.handle(Acknowledge::parse(msg)?, src)?,
            }

            Ok(())
        }
    }
}
