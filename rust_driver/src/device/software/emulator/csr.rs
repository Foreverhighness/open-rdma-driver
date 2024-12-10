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

use super::net::Agent;
use super::Emulator;

#[derive(Debug, Default)]
pub(super) struct EmulatorCsrs {
    cmd_request: EmulatorRegistersCommandRequest,
    cmd_response: EmulatorRegistersCommandResponse,
    meta_report: EmulatorRegistersMetaReport,
    send: EmulatorRegistersSend,
}

#[derive(Debug)]
pub struct EmulatorCsrsHandler<'h, UA: Agent> {
    csrs: &'h EmulatorCsrs,
    dev: &'h Emulator<UA>,
}
impl<'h, UA: Agent> EmulatorCsrsHandler<'h, UA> {
    pub(super) fn new<'c, 'd>(csrs: &'c EmulatorCsrs, dev: &'d Emulator<UA>) -> Self
    where
        'c: 'h,
        'd: 'h,
    {
        Self { csrs, dev }
    }
}
