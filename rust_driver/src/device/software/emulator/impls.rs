//! Blue Rdma Emulator implementation

use core::sync::atomic::AtomicBool;
use std::sync::Arc;

use super::csr::{EmulatorCsrs, EmulatorCsrsHandler};
use super::device_api::RawDevice;
use super::net;

#[derive(Debug)]
pub enum State {
    NotReady,
}

#[derive(Debug)]
pub struct Emulator<UA: net::Agent> {
    csrs: EmulatorCsrs,

    state: State,

    udp_agent: UA,
    stop: AtomicBool,
}

impl<UA: net::Agent> Emulator<UA> {
    pub fn new(udp_agent: UA) -> Self {
        Self {
            udp_agent,
            csrs: EmulatorCsrs::default(),
            state: State::NotReady,
            stop: AtomicBool::default(),
        }
    }

    pub(super) fn start_net(self: &Arc<Self>)
    where
        UA: Send + Sync + 'static,
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

impl<UA: net::Agent> Drop for Emulator<UA> {
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
