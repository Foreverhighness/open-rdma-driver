//! Blue Rdma Emulator implementation

use core::sync::atomic::AtomicBool;
use std::sync::Arc;

use super::csr::{EmulatorCsrs, EmulatorCsrsHandler};
use super::device_api::RawDevice;
use super::simulator::dma_client::DmaClient;
use super::simulator::udp_agent::UdpAgent;
use super::{dma, net};

#[derive(Debug)]
pub enum State {
    NotReady,
}

#[derive(Debug)]
pub struct Emulator<UA: net::Agent = UdpAgent, DC: dma::Client = DmaClient> {
    /// Control and Status Registers
    csrs: EmulatorCsrs,

    /// Udp agent
    udp_agent: UA,

    /// DMA Client
    pub(crate) dma_client: DC,

    /// Thread Stop signal, may move out of this structure if I change this structure into `EmulatorInner`
    stop: AtomicBool,

    /// Emulator State
    state: State,
}

impl<UA: net::Agent, DC: dma::Client> Emulator<UA, DC> {
    pub fn new(udp_agent: UA, dma_client: DC) -> Self {
        Self {
            udp_agent,
            dma_client,
            csrs: EmulatorCsrs::default(),
            state: State::NotReady,
            stop: AtomicBool::default(),
        }
    }

    pub(super) fn start_net(self: &Arc<Self>)
    where
        UA: Send + Sync + 'static,
        DC: Send + Sync + 'static,
    {
        let dev = Arc::clone(self);

        // TODO(fh): Store this handler properly
        let _handle = std::thread::spawn(move || {
            let mut buf = vec![0u8; 8192];
            while !dev.stop.load(core::sync::atomic::Ordering::Relaxed) {
                let (len, src) = dev.udp_agent.recv_from(&mut buf).expect("recv error");
                log::debug!("receive data {:?} from {src:?}", &buf[..len]);
            }
        });
    }
}

impl<UA: net::Agent, DC: dma::Client> Drop for Emulator<UA, DC> {
    fn drop(&mut self) {
        self.stop.store(true, core::sync::atomic::Ordering::Relaxed);
    }
}

impl<UA: net::Agent> RawDevice for Emulator<UA> {
    type Csrs<'a>
        = EmulatorCsrsHandler<'a, UA>
    where
        Self: 'a;

    fn csrs(&self) -> Self::Csrs<'_> {
        Self::Csrs::new(&self.csrs, self)
    }
}
