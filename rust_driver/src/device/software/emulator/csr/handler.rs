use super::command_request::EmulatorRegistersCommandRequestHandler;
use super::command_response::EmulatorRegistersCommandResponseHandler;
use super::meta_report::EmulatorRegistersMetaReportHandler;
use super::reset::EmulatorRegisterResetHandler;
use super::send::EmulatorRegistersSendHandler;
use super::EmulatorCsrsHandler;
use crate::device::software::emulator::device_api::{csr, ControlStatusRegisters};
use crate::device::software::emulator::dma::Client;
use crate::device::software::emulator::net::Agent;

impl<UA: Agent, DC: Client> ControlStatusRegisters for EmulatorCsrsHandler<'_, UA, DC> {
    fn cmd_request(&self) -> impl csr::RegistersCommandRequest {
        EmulatorRegistersCommandRequestHandler::new(&self.csrs.cmd_request, self.dev)
    }

    fn cmd_response(&self) -> impl csr::RegistersCommandResponse {
        EmulatorRegistersCommandResponseHandler::new(&self.csrs.cmd_response, self.dev)
    }

    fn meta_report(&self) -> impl csr::RegistersMetaReport {
        EmulatorRegistersMetaReportHandler::new(&self.csrs.meta_report, self.dev)
    }

    fn send(&self) -> impl csr::RegistersSend {
        EmulatorRegistersSendHandler::new(&self.csrs.send, self.dev)
    }

    fn reset(&self) -> impl csr::RegisterReset {
        EmulatorRegisterResetHandler::new(&self.csrs.reset, self.dev)
    }
}
