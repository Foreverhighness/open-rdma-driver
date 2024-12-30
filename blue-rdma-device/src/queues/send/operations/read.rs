use core::net::Ipv4Addr;

use super::common::Common;
use crate::dma::Client;
use crate::net::Agent;
use crate::net::util::generate_payload_from_msg;
use crate::queues::descriptor::HandleDescriptor;
use crate::queues::send::descriptors::{ScatterGatherElement, Seg0, Seg1, VariableLengthSge};
use crate::third_party::net::{
    Key, Metadata, PKey, PayloadInfo, Qpn, RdmaGeneralMeta, RdmaMessage, RdmaMessageMetaCommon, RethHeader,
};
use crate::third_party::queues::meta_report::ToHostWorkRbDescOpcode;
use crate::third_party::rdma::Psn;
use crate::types::SendFlag;
use crate::{DeviceInner, Result};

#[derive(Debug)]
pub struct Read {
    common: Common,
    sge: ScatterGatherElement,
}

impl<UA: Agent, DC: Client> HandleDescriptor<Read> for DeviceInner<UA, DC> {
    type Context = ();
    type Output = ();

    fn handle(&self, req: &Read, (): &mut Self::Context) -> Result<Self::Output> {
        log::info!("handle read op: {req:#?}");

        // Question here: When to send ack?
        let ack_req = req.common.send_flag == SendFlag::IbvSendSignaled;

        let common_meta = {
            let tran_type = req.common.qp_type.into();
            let pkey = PKey::new(req.common.msn);
            let dqpn = Qpn::new(req.common.dest_qpn);
            move |opcode, psn, ack_req| {
                RdmaMessageMetaCommon {
                    tran_type,
                    opcode,
                    solicited: false,
                    // We use the pkey to store msn
                    pkey,
                    dqpn,
                    ack_req,
                    psn: Psn::new(psn),
                }
            }
        };

        let remote_va = req.common.remote_addr.0;
        let psn = req.common.psn;
        let read_msg = RdmaMessage {
            meta_data: Metadata::General(RdmaGeneralMeta {
                common_meta: common_meta(ToHostWorkRbDescOpcode::RdmaReadRequest, psn, ack_req),
                reth: RethHeader {
                    va: remote_va,
                    rkey: Key::new(req.common.remote_key.get()),
                    len: req.common.total_len,
                },
                imm: None,
                secondary_reth: Some(RethHeader {
                    va: req.sge.local_addr.into(),
                    rkey: Key::new(req.sge.local_key.get()),
                    len: req.sge.len,
                }),
            }),
            payload: PayloadInfo::new(),
        };

        // FIXME(fh): hardcode for calculate Invariant CRC, should remove
        let src = Ipv4Addr::new(192, 168, 0, 2);
        let dst = req.common.dest_ip;
        let payload = generate_payload_from_msg(&read_msg, src, dst);
        let _ = self
            .udp_agent
            .get()
            .unwrap()
            .send_to(&payload, dst.into())
            .expect("send error");

        Ok(())
    }
}

#[derive(Debug)]
/// Write Builder
// TODO(fh): use strict state machine representation?
pub struct Builder(Read);

impl Builder {
    /// Initialize builder from valid seg0
    pub fn from_seg0(seg0: Seg0) -> Self {
        Self(Read {
            common: Common::from_seg0(&seg0),
            sge: ScatterGatherElement {
                local_key: 0.into(),
                len: 0,
                local_addr: 0.into(),
            },
        })
    }

    /// Update valid seg1, assuming only seg0 is processed
    pub fn with_seg1(mut self, seg1: Seg1) -> Self {
        self.0.common.with_seg1(&seg1);

        self
    }

    /// Update sge, assuming seg0 and seg1 are processed
    pub fn with_sge(mut self, sge: VariableLengthSge) -> Read {
        let sge0 = sge.sge1;
        let sge1 = sge.sge2;
        assert!(
            sge1.local_addr == 0.into() && sge1.len == 0 && sge1.local_key == 0.into(),
            "support only one sge for now"
        );

        self.0.sge = sge0;

        self.0
    }
}
