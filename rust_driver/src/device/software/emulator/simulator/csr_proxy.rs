//! Control Status Register Proxy

use core::sync::atomic::AtomicBool;
use core::time::Duration;
use std::sync::Arc;
use std::thread::JoinHandle;

use log::debug;

use super::rpc::{self, BarIoInfo};

/// CSR Proxy is responsible for forwarding CSR read/write requests from the Python Server to the emulator
/// and forwarding the response to the Python Server
#[derive(Debug)]
pub struct Proxy<R: rpc::Client> {
    client_id: u64,

    rpc: R,
}

impl<R: rpc::Client> Proxy<R> {
    pub fn new(client_id: u64, rpc: R) -> Self {
        Self { client_id, rpc }
    }

    fn recv_read_request(&self) -> BarIoInfo {
        let mut request = BarIoInfo::new();
        loop {
            unsafe {
                self.rpc.c_getPcieBarReadReq(&raw mut request, self.client_id);
            }
            if request.valid == 1 {
                debug!("recv csr read request {request:?}");
                return request;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    fn handle_read_request(&self, req: &BarIoInfo) {
        let &BarIoInfo { addr, pci_tag, .. } = req;
        let value = 0;
        let mut response = BarIoInfo::new_read_response(pci_tag, value);
        unsafe {
            self.rpc.c_putPcieBarReadResp(self.client_id, &raw mut response);
        }
    }

    fn recv_write_request(&self) -> BarIoInfo {
        let mut request = BarIoInfo::new();
        loop {
            unsafe {
                self.rpc.c_getPcieBarWriteReq(&raw mut request, self.client_id);
            }
            if request.valid == 1 {
                debug!("recv csr write request {request:?}");
                return request;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    fn handle_write_request(&self, req: &BarIoInfo) {
        let &BarIoInfo {
            value, addr, pci_tag, ..
        } = req;
        // write
        let mut response = BarIoInfo::new_write_response(pci_tag, true);
        unsafe {
            self.rpc.c_putPcieBarWriteResp(self.client_id, &raw mut response);
        }
    }
}

impl<R: rpc::Client + Send + Sync + 'static> Proxy<R> {
    pub fn run(self) -> (JoinHandle<()>, JoinHandle<()>, Arc<AtomicBool>) {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_read = Arc::clone(&stop);
        let stop_write = Arc::clone(&stop);
        let proxy_read = Arc::new(self);
        let proxy_write = Arc::clone(&proxy_read);

        let handle_read = std::thread::spawn(move || {
            while !stop_read.load(core::sync::atomic::Ordering::Relaxed) {
                let req = proxy_read.recv_read_request();
                proxy_read.handle_read_request(&req);
            }
        });
        let handle_write = std::thread::spawn(move || {
            while !stop_write.load(core::sync::atomic::Ordering::Relaxed) {
                let req = proxy_write.recv_write_request();
                proxy_write.handle_write_request(&req);
            }
        });

        (handle_read, handle_write, stop)
    }
}
