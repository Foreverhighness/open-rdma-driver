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

pub(crate) trait Message<Dev> {
    fn handle(&self, dev: &Dev) -> super::super::Result;
}

// which is better?
pub(crate) trait HandleMessage<Msg> {
    fn handle(&self, msg: Msg) -> super::super::Result;
}

mod handler {
    use core::net::IpAddr;

    use super::*;
    use crate::device::software::emulator::dma::Client;
    use crate::device::software::emulator::errors::Error;
    use crate::device::software::emulator::net::util::generate_ack;
    use crate::device::software::emulator::net::Agent;
    use crate::device::software::emulator::DeviceInner;
    use crate::device::software::types::RdmaMessage;
    use crate::device::ToHostWorkRbDescOpcode;

    impl<UA: Agent, DC: Client> DeviceInner<UA, DC> {
        pub(crate) fn handle_message(&self, msg: &RdmaMessage, src: IpAddr) -> Result<(), Error> {
            log::debug!("handle network message {msg:?}");
            match msg.meta_data.common_meta().opcode {
                ToHostWorkRbDescOpcode::RdmaWriteFirst => self.handle(WriteFirst::parse(msg)?)?,
                ToHostWorkRbDescOpcode::RdmaWriteMiddle => self.handle(WriteMiddle::parse(msg)?)?,
                ToHostWorkRbDescOpcode::RdmaWriteLast => self.handle(WriteLast::parse(msg)?)?,
                ToHostWorkRbDescOpcode::RdmaWriteLastWithImmediate => {
                    self.handle(WriteLastWithImmediate::parse(msg)?)?
                }
                ToHostWorkRbDescOpcode::RdmaWriteOnly => self.handle(WriteOnly::parse(msg)?)?,
                ToHostWorkRbDescOpcode::RdmaWriteOnlyWithImmediate => {
                    self.handle(WriteOnlyWithImmediate::parse(msg)?)?
                }
                ToHostWorkRbDescOpcode::RdmaReadResponseFirst => self.handle(ReadResponseFirst::parse(msg)?)?,
                ToHostWorkRbDescOpcode::RdmaReadResponseMiddle => self.handle(ReadResponseMiddle::parse(msg)?)?,
                ToHostWorkRbDescOpcode::RdmaReadResponseLast => self.handle(ReadResponseLast::parse(msg)?)?,
                ToHostWorkRbDescOpcode::RdmaReadResponseOnly => self.handle(ReadResponseOnly::parse(msg)?)?,
                ToHostWorkRbDescOpcode::RdmaReadRequest => self.handle(ReadRequest::parse(msg)?)?,
                ToHostWorkRbDescOpcode::Acknowledge => self.handle(Acknowledge::parse(msg)?)?,
            }

            let need_ack = msg.meta_data.common_meta().ack_req;
            if need_ack {
                let buf = generate_ack(&msg);
                let _ = self.udp_agent.get().unwrap().send_to(&buf, src);
            }

            Ok(())
        }
    }
}
