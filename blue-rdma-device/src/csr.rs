//! Control Status Register for emulator use
//! In contrast to the Driver layer definition, `repr(C)` is not required in Emulator implementation.

#[macro_use]
mod macros;

pub mod command_request;
pub mod command_response;
pub mod handler;
pub mod meta_report;
pub mod send;

pub mod reset;
// mod types;

use command_request::EmulatorRegistersCommandRequest;
use command_response::EmulatorRegistersCommandResponse;
use meta_report::EmulatorRegistersMetaReport;
use reset::EmulatorRegisterReset;
use send::EmulatorRegistersSend;

use super::DeviceInner;
use super::dma::Client;
use super::net::Agent;

#[derive(Debug, Default)]
pub struct EmulatorCsrs {
    pub(crate) cmd_request: EmulatorRegistersCommandRequest,
    pub(crate) cmd_response: EmulatorRegistersCommandResponse,
    pub(crate) meta_report: EmulatorRegistersMetaReport,
    pub(crate) send: EmulatorRegistersSend,
    pub(crate) reset: EmulatorRegisterReset,
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
