//! Blue Rdma Device API

pub mod csr;

pub trait ControlStatusRegisters {
    type CmdRequest<'a>: csr::RegistersCommandRequest
    where
        Self: 'a;
    type CmdResponse<'a>: csr::RegistersCommandResponse
    where
        Self: 'a;
    type MetaReport<'a>: csr::RegistersMetaReport
    where
        Self: 'a;
    type Send<'a>: csr::RegistersSend
    where
        Self: 'a;

    fn cmd_request(&self) -> Self::CmdRequest<'_>;
    fn cmd_response(&self) -> Self::CmdResponse<'_>;
    fn meta_report(&self) -> Self::MetaReport<'_>;
    fn send(&self) -> Self::Send<'_>;
}

pub trait RawDevice {
    type Csrs<'c>: ControlStatusRegisters
    where
        Self: 'c;

    fn csrs(&self) -> Self::Csrs<'_>;
}
