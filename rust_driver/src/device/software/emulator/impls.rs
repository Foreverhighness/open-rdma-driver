//! Blue Rdma Emulator implementation

use super::csr::{RegistersCommandRequest, RegistersCommandResponse, RegistersMetaReport, RegistersSend};
use super::net;

#[derive(Debug)]
pub enum State {
    NotReady,
}

#[derive(Debug)]
pub struct Emulator<UA: net::Agent> {
    udp_agent: UA,

    regs_cmd_request: RegistersCommandRequest,
    regs_cmd_response: RegistersCommandResponse,
    regs_send: RegistersSend,
    regs_meta_report: RegistersMetaReport,

    state: State,
}

impl<UA: net::Agent> Emulator<UA> {
    pub fn new(udp_agent: UA) -> Self {
        Self {
            udp_agent,
            regs_cmd_request: RegistersCommandRequest::default(),
            regs_cmd_response: RegistersCommandResponse::default(),
            regs_send: RegistersSend::default(),
            regs_meta_report: RegistersMetaReport::default(),
            state: State::NotReady,
        }
    }
}
