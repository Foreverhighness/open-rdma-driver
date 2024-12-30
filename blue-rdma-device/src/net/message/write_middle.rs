use super::HandleMessage;
use crate::DeviceInner;
use crate::dma::Client;
use crate::net::util::{generate_ack, message_to_bthreth};
use crate::net::{Agent, Error};
use crate::queues::complete_queue::CompleteQueue;
use crate::third_party::net::RdmaMessage;
use crate::third_party::queues::meta_report::ToHostWorkRbDescStatus;

#[expect(dead_code, reason = "need refactor")]
#[derive(Debug)]
pub struct WriteMiddle<'msg> {
    // TODO(fh): replace with BaseTransportHeader
    bth: &'msg RdmaMessage,
    reth: &'msg RdmaMessage,
}
type Message<'msg> = WriteMiddle<'msg>;

impl<'msg> Message<'msg> {
    pub const fn parse<'input>(msg: &'input RdmaMessage) -> Result<Self, Error>
    where
        'input: 'msg,
    {
        Ok(Self { bth: msg, reth: msg })
    }
}

impl<UA: Agent, DC: Client> HandleMessage<Message<'_>> for DeviceInner<UA, DC> {
    fn handle(&self, msg: Message, src: core::net::IpAddr) -> crate::Result {
        let msg = msg.bth;

        let qpn = msg.meta_data.common_meta().dqpn.get();
        let psn = msg.meta_data.common_meta().psn.get();

        let mr_error = self.copy_to_with_key(msg).is_err();

        let guard = self.queue_pair_table().guard();
        let qp_context = self.queue_pair_table().get(qpn, &guard);
        // TODO(fh): qp_error should contains validations
        let qp_error = qp_context.is_none();

        let abnormal_packet = mr_error || qp_error;

        // FIXME(fh): req_status hardcode to normal state, which is incorrect
        let req_status = ToHostWorkRbDescStatus::Normal.into();
        let can_auto_ack;
        let can_skip_report_header;
        let expected_psn_option;

        if abnormal_packet {
            can_auto_ack = false;
            can_skip_report_header = false;
            // None or directly use zero?
            expected_psn_option = None;
        } else {
            // only check psn on normal packet
            let qp_context = qp_context.expect("logic error: normal packet without qp context");
            let expected_psn = qp_context.expected_psn();
            let is_error_state = qp_context.is_error();

            can_auto_ack = !is_error_state && psn == expected_psn;
            can_skip_report_header = psn == expected_psn;

            expected_psn_option = Some(expected_psn);

            let new_expected_psn = (psn >= expected_psn).then_some(psn + 1);
            let new_error_psn = (psn > expected_psn).then_some(psn);

            if let Some(new_expected_psn) = new_expected_psn {
                qp_context.set_expect_psn(new_expected_psn);
            }

            if let Some(new_error_psn) = new_error_psn {
                qp_context.set_error_psn(new_error_psn);
            }
        }

        let need_ack = can_auto_ack && msg.meta_data.common_meta().ack_req;
        if need_ack {
            let buf = generate_ack(msg, qp_context.unwrap().peer_qpn(), expected_psn_option.unwrap());
            let _ = self.udp_agent.get().unwrap().send_to(&buf, src);
        }

        let need_report_header = !can_skip_report_header;
        if need_report_header {
            let descriptor = message_to_bthreth(msg, expected_psn_option.unwrap_or(0u32), req_status, can_auto_ack);
            log::debug!("push meta report: {descriptor:?}");
            unsafe { self.meta_report_queue().push(descriptor) };
        }

        Ok(())
    }
}
