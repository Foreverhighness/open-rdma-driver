//! Blue Rdma Device API

pub mod csr;

pub trait ControlStatusRegisters {
    type CmdRequest: csr::RegistersCommandRequest;
    type CmdResponse: csr::RegistersCommandResponse;
    type MetaReport: csr::RegistersMetaReport;
    type Send: csr::RegistersSend;

    fn cmd_request(&self) -> &Self::CmdRequest;
    fn cmd_response(&self) -> &Self::CmdResponse;
    fn meta_report(&self) -> &Self::MetaReport;
    fn send(&self) -> &Self::Send;
}

pub trait RawDevice {
    type Csrs: ControlStatusRegisters;

    fn csrs(&self) -> &Self::Csrs;
}
