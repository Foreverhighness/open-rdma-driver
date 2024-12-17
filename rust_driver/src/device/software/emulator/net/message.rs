mod acknowledge;
mod read_request;
mod read_response;
mod write;

mod handler {
    use crate::device::software::emulator::net::Agent;
    use crate::device::software::emulator::queues::complete_queue::CompleteQueue;
    use crate::device::software::emulator::queues::{BaseTransportHeader, BthReth, RdmaExtendedTransportHeader};
    use crate::device::software::emulator::Emulator;
    use crate::device::software::types::{Metadata, RdmaMessage};
    use crate::device::{
        ToHostWorkRbDescCommon, ToHostWorkRbDescOpcode, ToHostWorkRbDescStatus, ToHostWorkRbDescTransType,
        ToHostWorkRbDescWriteOrReadResp,
    };
    use crate::types::{Msn, Psn};

    impl<UA: Agent> Emulator<UA> {
        pub(crate) fn handle_message(&self, msg: &RdmaMessage) {
            let meta = &msg.meta_data;

            // TODO(fh): validate part
            // TODO(fh): dma part
            match meta {
                Metadata::General(header) => match header.common_meta.opcode {
                    ToHostWorkRbDescOpcode::RdmaWriteFirst | ToHostWorkRbDescOpcode::RdmaWriteLast => {
                        let op = ToHostWorkRbDescWriteOrReadResp {
                            common: ToHostWorkRbDescCommon {
                                // TODO(fh): remove hard code value
                                status: ToHostWorkRbDescStatus::Normal,
                                // TODO(fh): remove hard code value
                                trans: ToHostWorkRbDescTransType::Rc,
                                dqpn: crate::types::Qpn::new(msg.meta_data.common_meta().dqpn.get()),
                                msn: Msn::new(msg.meta_data.common_meta().pkey.get()),
                                expected_psn: Psn::new(0),
                            },
                            is_read_resp: header.common_meta.opcode.is_read_resp(),
                            write_type: header
                                .common_meta
                                .opcode
                                .write_type()
                                .unwrap_or(crate::device::ToHostWorkRbDescWriteType::Only),
                            psn: header.common_meta.psn,
                            addr: header.reth.va,
                            len: header.reth.len,
                            can_auto_ack: false,
                        };
                        // Operation -> Descriptor
                        let descriptor = {
                            let expect_psn = op.common.expected_psn.get();
                            let req_status = op.common.status.into();
                            let msn = op.common.msn.get();
                            let can_auto_ack = op.can_auto_ack;

                            let trans_type = op.common.trans.into();
                            let opcode = header.common_meta.opcode.clone().into();
                            let qpn = op.common.dqpn.get();
                            let psn = op.psn.get();
                            let pad_cnt = msg.payload.get_pad_cnt() as u8;
                            // from software copy snip
                            let solicited = false;
                            let is_ack_req = false;

                            let bth =
                                BaseTransportHeader::new(trans_type, opcode, qpn, psn, solicited, is_ack_req, pad_cnt);

                            let local_va = header.reth.va.into();
                            let local_key = header.reth.rkey.get().into();
                            let len = header.reth.len;
                            let reth = RdmaExtendedTransportHeader::new(local_va, local_key, len);

                            BthReth::new(expect_psn, req_status, bth, reth, msn, can_auto_ack)
                        };
                        unsafe { self.meta_report_queue().push(descriptor) };
                    }
                    _ => todo!(),
                },
                Metadata::Acknowledge(header) => todo!(),
            };
        }
    }
}
