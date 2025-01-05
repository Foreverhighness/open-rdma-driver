use super::common::Common;
use crate::dma::Client;
use crate::net::Agent;
use crate::queues::descriptor::HandleDescriptor;
use crate::queues::send::descriptors::{ScatterGatherElement, Seg0, Seg1, VariableLengthSge};
use crate::{DeviceInner, Result};

#[derive(Debug)]
pub struct WriteWithImmediate {
    common: Common,
    #[expect(dead_code, reason = "todo")]
    last: bool,
    #[expect(dead_code, reason = "todo")]
    first: bool,
    immediate_data: u32,
    sge: ScatterGatherElement,
}

impl<UA: Agent, DC: Client> HandleDescriptor<WriteWithImmediate> for DeviceInner<UA, DC> {
    type Context = ();
    type Output = ();

    #[expect(unused, reason = "todo")]
    fn handle(&self, request: &WriteWithImmediate, cx: &mut Self::Context) -> Result<Self::Output> {
        todo!()
    }
}

#[derive(Debug)]
/// Write Builder
// TODO(fh): use strict state machine representation?
pub struct Builder(WriteWithImmediate);

impl Builder {
    /// Initialize builder from valid seg0
    pub fn from_seg0(seg0: Seg0) -> Self {
        let first = seg0.header.first();
        let last = seg0.header.last();
        Self(WriteWithImmediate {
            common: Common::from_seg0(seg0),
            last,
            first,
            immediate_data: 0,
            sge: ScatterGatherElement {
                local_key: 0.into(),
                len: 0,
                local_addr: 0.into(),
            },
        })
    }

    /// Update valid seg1, assuming only seg0 is processed
    pub fn with_seg1(mut self, seg1: Seg1) -> Self {
        self.0.immediate_data = seg1.immediate_data;

        self.0.common.with_seg1(seg1);

        self
    }

    /// Update sge, assuming seg0 and seg1 are processed
    pub fn with_sge(mut self, sge: VariableLengthSge) -> WriteWithImmediate {
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
