//! Blue Rdma Emulator implementation

use super::csr::EmulatorCsrs;
use super::net;

#[derive(Debug)]
pub enum State {
    NotReady,
}

#[derive(Debug)]
pub struct Emulator<UA: net::Agent> {
    udp_agent: UA,

    csrs: EmulatorCsrs,

    state: State,
}

impl<UA: net::Agent> Emulator<UA> {
    pub fn new(udp_agent: UA) -> Self {
        Self {
            udp_agent,
            csrs: EmulatorCsrs::default(),
            state: State::NotReady,
        }
    }
}
