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

use super::super::Result;

pub(crate) trait Message<Dev> {
    fn handle(&self, dev: &Dev) -> Result;
}

// which is better?
pub(crate) trait HandleMessage<Msg> {
    fn handle(&self, msg: &Msg) -> Result;
}

mod handler {
    use core::net::{IpAddr, Ipv4Addr};

    use super::{HandleMessage, Message};
    use crate::device::software::emulator::errors::Error;
    use crate::device::software::emulator::net::message::acknowledge::Acknowledge;
    use crate::device::software::emulator::net::message::write_first::WriteFirst;
    use crate::device::software::emulator::net::message::write_last::WriteLast;
    use crate::device::software::emulator::net::util::generate_ack;
    use crate::device::software::emulator::net::Agent;
    use crate::device::software::emulator::DeviceInner;
    use crate::device::software::types::RdmaMessage;
    use crate::device::ToHostWorkRbDescOpcode;

    impl<UA: Agent> DeviceInner<UA> {
        pub(crate) fn handle_message(&self, msg: &RdmaMessage, src: IpAddr) -> Result<(), Error> {
            log::debug!("handle network message {msg:?}");
            match msg.meta_data.common_meta().opcode {
                ToHostWorkRbDescOpcode::RdmaWriteFirst => WriteFirst::parse(msg)?.handle(self)?,
                ToHostWorkRbDescOpcode::RdmaWriteMiddle => todo!(),
                ToHostWorkRbDescOpcode::RdmaWriteLast => self.handle(&WriteLast::parse(msg)?)?,
                ToHostWorkRbDescOpcode::Acknowledge => self.handle_acknowledge(Acknowledge::parse(msg)?),
                _ => todo!(),
            }

            let need_ack = msg.meta_data.common_meta().ack_req;
            if need_ack {
                let buf = generate_ack(&msg);
                let _ = self.udp_agent.get().unwrap().send_to(&buf, src);
            }

            // TODO(fh): validate part

            // TODO(fh): dma part
            // {
            //     let data = &msg.payload.sg_list;
            //     assert_eq!(data.len(), 1, "currently only consider one Sge");
            //     let data = data[0];

            //     let data = unsafe { core::slice::from_raw_parts(data.data, data.len) };

            //     let Metadata::General(ref header) = msg.meta_data else {
            //         panic!("currently only consider write first and write last packet");
            //     };
            //     let key = header.reth.rkey.get().into();
            //     let va = VirtualAddress(header.reth.va);
            //     let access_flag = header.needed_permissions();

            //     let dma_addr = self
            //         .memory_region_table()
            //         .query(key, va, access_flag, &self.page_table)
            //         .expect("validation failed");

            //     let ptr = self.dma_client.with_dma_addr::<u8>(dma_addr);
            //     unsafe { ptr.write_bytes(data) };
            // }

            // // TODO(fh): parse from raw part, currently RdmaMessage don't contains this field
            // let need_auto_ack = true;
            // let is_write_last = msg.meta_data.common_meta().opcode == ToHostWorkRbDescOpcode::RdmaWriteLast;
            // if need_auto_ack && is_write_last {
            //     let buf = generate_ack(&msg);
            //     let _ = self
            //         .udp_agent
            //         .get()
            //         .unwrap()
            //         .send_to(&buf, core::net::IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2)));
            // }

            // let descriptor = message_to_bthreth(msg);
            // log::debug!("push meta report: {descriptor:?}");
            // unsafe { self.meta_report_queue().push(descriptor) };

            Ok(())
        }
    }
}
