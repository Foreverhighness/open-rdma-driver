use super::command_request::EmulatorRegistersCommandRequestHandler;
use super::command_response::EmulatorRegistersCommandResponseHandler;
use super::meta_report::EmulatorRegistersMetaReportHandler;
use super::send::EmulatorRegistersSendHandler;
use super::EmulatorCsrsHandler;
use crate::device::software::emulator::device_api::ControlStatusRegisters;
use crate::device::software::emulator::net::Agent;

impl<UA: Agent> ControlStatusRegisters for EmulatorCsrsHandler<'_, UA> {
    type CmdRequest<'a>
        = EmulatorRegistersCommandRequestHandler<'a, UA>
    where
        Self: 'a;
    type CmdResponse<'a>
        = EmulatorRegistersCommandResponseHandler<'a, UA>
    where
        Self: 'a;
    type MetaReport<'a>
        = EmulatorRegistersMetaReportHandler<'a, UA>
    where
        Self: 'a;
    type Send<'a>
        = EmulatorRegistersSendHandler<'a, UA>
    where
        Self: 'a;

    fn cmd_request(&self) -> Self::CmdRequest<'_> {
        Self::CmdRequest::new(&self.csrs.cmd_request, self.dev)
    }

    fn cmd_response(&self) -> Self::CmdResponse<'_> {
        Self::CmdResponse::new(&self.csrs.cmd_response, self.dev)
    }

    fn meta_report(&self) -> Self::MetaReport<'_> {
        Self::MetaReport::new(&self.csrs.meta_report, self.dev)
    }

    fn send(&self) -> Self::Send<'_> {
        Self::Send::new(&self.csrs.send, self.dev)
    }
}
