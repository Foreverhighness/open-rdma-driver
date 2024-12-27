//! Blue Rdma Device API

pub mod csr;

pub trait ControlStatusRegisters {
    fn cmd_request(&self) -> impl csr::RegistersCommandRequest;
    fn cmd_response(&self) -> impl csr::RegistersCommandResponse;
    fn meta_report(&self) -> impl csr::RegistersMetaReport;
    fn send(&self) -> impl csr::RegistersSend;
    fn reset(&self) -> impl csr::RegisterReset;
}

pub trait RawDevice {
    fn csrs(&self) -> impl ControlStatusRegisters;
}
