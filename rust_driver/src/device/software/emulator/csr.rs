//! Control Status Register for emulator use
//! In contrast to the Driver layer definition, `repr(C)` is not required in Emulator implementation.

#[macro_use]
mod macros;

pub mod command_request;
pub mod command_response;
pub mod handler;
pub mod meta_report;
pub mod send;

// mod types;

use command_request::EmulatorRegistersCommandRequest;
use command_response::EmulatorRegistersCommandResponse;
use meta_report::EmulatorRegistersMetaReport;
use send::EmulatorRegistersSend;

use super::dma::Client;
use super::net::Agent;
use super::DeviceInner;

#[derive(Debug, Default)]
pub struct EmulatorCsrs {
    pub(crate) cmd_request: EmulatorRegistersCommandRequest,
    pub(crate) cmd_response: EmulatorRegistersCommandResponse,
    pub(crate) meta_report: EmulatorRegistersMetaReport,
    pub(crate) send: EmulatorRegistersSend,
}

#[derive(Debug)]
pub struct EmulatorCsrsHandler<'h, UA: Agent, DC: Client> {
    csrs: &'h EmulatorCsrs,
    dev: &'h DeviceInner<UA, DC>,
}
impl<'h, UA: Agent, DC: Client> EmulatorCsrsHandler<'h, UA, DC> {
    pub(super) fn new<'c, 'd>(csrs: &'c EmulatorCsrs, dev: &'d DeviceInner<UA, DC>) -> Self
    where
        'c: 'h,
        'd: 'h,
    {
        Self { csrs, dev }
    }
}
