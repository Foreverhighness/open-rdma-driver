use super::common::Common;
use crate::dma::Client;
use crate::net::Agent;
use crate::queues::descriptor::HandleDescriptor;
use crate::queues::send::descriptors::{ScatterGatherElement, Seg0, Seg1, VariableLengthSge};
use crate::queues::send::operations::common::generate_segments_from_request;
use crate::third_party::queues::meta_report::ToHostWorkRbDescOpcode;
use crate::{DeviceInner, Result};

#[derive(Debug)]
pub struct ReadResponse {
    common: Common,
    #[expect(dead_code, reason = "todo")]
    last: bool,
    #[expect(dead_code, reason = "todo")]
    first: bool,
    sge: ScatterGatherElement,
}

impl AsRef<Common> for ReadResponse {
    fn as_ref(&self) -> &Common {
        &self.common
    }
}

impl<UA: Agent, DC: Client> HandleDescriptor<ReadResponse> for DeviceInner<UA, DC> {
    type Context = ();
    type Output = ();

    fn handle(&self, req: &ReadResponse, &mut (): &mut Self::Context) -> Result<Self::Output> {
        log::info!("handle read response op: {req:?}");

        let path_mtu = u32::from(&req.common.path_mtu_kind);
        let segments = generate_segments_from_request(req.sge.local_addr.0, req.sge.len, path_mtu);
        let key = req.sge.local_key;

        let mut remote_va = req.common.remote_addr.0;
        let mut psn = req.common.psn;
        match *segments.as_slice() {
            [ref only] => {
                self.send_write_message(
                    req,
                    ToHostWorkRbDescOpcode::RdmaReadResponseOnly,
                    psn,
                    true,
                    key,
                    remote_va,
                    only,
                );
            }
            [ref first, ref middles @ .., ref last] => {
                self.send_write_message(
                    req,
                    ToHostWorkRbDescOpcode::RdmaReadResponseFirst,
                    psn,
                    false,
                    key,
                    remote_va,
                    first,
                );

                remote_va += u64::from(first.len());
                psn = psn.wrapping_add(1);

                for middle in middles {
                    self.send_write_message(
                        req,
                        ToHostWorkRbDescOpcode::RdmaReadResponseMiddle,
                        psn,
                        false,
                        key,
                        remote_va,
                        middle,
                    );

                    remote_va += u64::from(middle.len());
                    psn = psn.wrapping_add(1);
                }

                self.send_write_message(
                    req,
                    ToHostWorkRbDescOpcode::RdmaReadResponseLast,
                    psn,
                    true,
                    key,
                    remote_va,
                    last,
                );
            }
            [] => todo!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
/// Write Builder
// TODO(fh): use strict state machine representation?
pub struct Builder(ReadResponse);

impl Builder {
    /// Initialize builder from valid seg0
    pub fn from_seg0(seg0: Seg0) -> Self {
        let first = seg0.header.first();
        let last = seg0.header.last();
        Self(ReadResponse {
            common: Common::from_seg0(seg0),
            last,
            first,
            sge: ScatterGatherElement {
                local_key: 0.into(),
                len: 0,
                local_addr: 0.into(),
            },
        })
    }

    /// Update valid seg1, assuming only seg0 is processed
    pub fn with_seg1(mut self, seg1: Seg1) -> Self {
        self.0.common.with_seg1(seg1);

        self
    }

    /// Update sge, assuming seg0 and seg1 are processed
    pub fn with_sge(mut self, sge: VariableLengthSge) -> ReadResponse {
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
