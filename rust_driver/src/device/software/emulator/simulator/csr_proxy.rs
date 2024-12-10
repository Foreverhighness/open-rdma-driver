//! Control Status Register Proxy

use core::sync::atomic::AtomicBool;
use core::time::Duration;
use std::sync::Arc;
use std::thread::JoinHandle;

use log::debug;

use super::super::device_api::{self, ControlStatusRegisters};
use super::rpc::{self, BarIoInfo};

/// CSR Proxy is responsible for forwarding CSR read/write requests from the Python Server to the emulator
/// and forwarding the response to the Python Server
#[derive(Debug)]
pub struct Proxy<R: rpc::Client, Dev: device_api::RawDevice> {
    client_id: u64,

    rpc: R,
    dev: Arc<Dev>,
}

impl<R: rpc::Client, Dev: device_api::RawDevice> Proxy<R, Dev> {
    pub fn new(client_id: u64, rpc: R, dev: Arc<Dev>) -> Self {
        Self { client_id, rpc, dev }
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
        let value = self.dev.csrs().read(addr);
        let mut response = BarIoInfo::new_read_response(pci_tag, value.into());
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
        self.dev.csrs().write(addr, value as u32);
        let mut response = BarIoInfo::new_write_response(pci_tag, true);
        unsafe {
            self.rpc.c_putPcieBarWriteResp(self.client_id, &raw mut response);
        }
    }
}

impl<R, Dev> Proxy<R, Dev>
where
    R: rpc::Client + Send + Sync + 'static,
    Dev: device_api::RawDevice + Send + Sync + 'static,
{
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

// May move into `device_api`
trait ControlStatusRegistersExt: ControlStatusRegisters {
    fn read(&self, addr: u64) -> u32;
    fn write(&self, addr: u64, val: u32);
}

mod csr {
    use super::super::super::csr::{
        REGISTERS_COMMAND_REQUEST_BASE_ADDR, REGISTERS_COMMAND_RESPONSE_BASE_ADDR, REGISTERS_META_REPORT_BASE_ADDR,
        REGISTERS_SEND_BASE_ADDR,
    };
    use super::device_api::csr::{RegisterOperation, RegistersQueue, RegistersQueueAddress};
    use super::device_api::ControlStatusRegisters;
    use super::ControlStatusRegistersExt;

    const REGISTERS_SEND_BASE_ADDR_END: u64 = REGISTERS_SEND_BASE_ADDR + 16;
    const REGISTERS_META_REPORT_BASE_ADDR_END: u64 = REGISTERS_META_REPORT_BASE_ADDR + 16;
    const REGISTERS_COMMAND_RESPONSE_BASE_ADDR_END: u64 = REGISTERS_COMMAND_RESPONSE_BASE_ADDR + 16;
    const REGISTERS_COMMAND_REQUEST_BASE_ADDR_END: u64 = REGISTERS_COMMAND_REQUEST_BASE_ADDR + 16;

    impl<T: ControlStatusRegisters> ControlStatusRegistersExt for T {
        fn read(&self, addr: u64) -> u32 {
            match addr {
                REGISTERS_COMMAND_REQUEST_BASE_ADDR..REGISTERS_COMMAND_REQUEST_BASE_ADDR_END => {
                    let csr = self.cmd_request();
                    match addr & 0xF {
                        0 => csr.addr().low().read(),
                        4 => csr.addr().high().read(),
                        8 => csr.head().read(),
                        12 => csr.tail().read(),
                        _ => unreachable!(),
                    }
                }
                REGISTERS_COMMAND_RESPONSE_BASE_ADDR..REGISTERS_COMMAND_RESPONSE_BASE_ADDR_END => {
                    let csr = self.cmd_response();
                    match addr & 0xF {
                        0 => csr.addr().low().read(),
                        4 => csr.addr().high().read(),
                        8 => csr.head().read(),
                        12 => csr.tail().read(),
                        _ => unreachable!(),
                    }
                }
                REGISTERS_META_REPORT_BASE_ADDR..REGISTERS_META_REPORT_BASE_ADDR_END => {
                    let csr = self.meta_report();
                    match addr & 0xF {
                        0 => csr.addr().low().read(),
                        4 => csr.addr().high().read(),
                        8 => csr.head().read(),
                        12 => csr.tail().read(),
                        _ => unreachable!(),
                    }
                }
                REGISTERS_SEND_BASE_ADDR..REGISTERS_SEND_BASE_ADDR_END => {
                    let csr = self.send();
                    match addr & 0xF {
                        0 => csr.addr().low().read(),
                        4 => csr.addr().high().read(),
                        8 => csr.head().read(),
                        12 => csr.tail().read(),
                        _ => unreachable!(),
                    }
                }
                _ => unimplemented!(),
            }
        }

        fn write(&self, addr: u64, val: u32) {
            match addr {
                REGISTERS_COMMAND_REQUEST_BASE_ADDR..REGISTERS_COMMAND_REQUEST_BASE_ADDR_END => {
                    let csr = self.cmd_request();
                    match addr & 0xF {
                        0 => csr.addr().low().write(val),
                        4 => csr.addr().high().write(val),
                        8 => csr.head().write(val),
                        12 => csr.tail().write(val),
                        _ => unreachable!(),
                    }
                }
                REGISTERS_COMMAND_RESPONSE_BASE_ADDR..REGISTERS_COMMAND_RESPONSE_BASE_ADDR_END => {
                    let csr = self.cmd_response();
                    match addr & 0xF {
                        0 => csr.addr().low().write(val),
                        4 => csr.addr().high().write(val),
                        8 => csr.head().write(val),
                        12 => csr.tail().write(val),
                        _ => unreachable!(),
                    }
                }
                REGISTERS_META_REPORT_BASE_ADDR..REGISTERS_META_REPORT_BASE_ADDR_END => {
                    let csr = self.meta_report();
                    match addr & 0xF {
                        0 => csr.addr().low().write(val),
                        4 => csr.addr().high().write(val),
                        8 => csr.head().write(val),
                        12 => csr.tail().write(val),
                        _ => unreachable!(),
                    }
                }
                REGISTERS_SEND_BASE_ADDR..REGISTERS_SEND_BASE_ADDR_END => {
                    let csr = self.send();
                    match addr & 0xF {
                        0 => csr.addr().low().write(val),
                        4 => csr.addr().high().write(val),
                        8 => csr.head().write(val),
                        12 => csr.tail().write(val),
                        _ => unreachable!(),
                    }
                }
                _ => unimplemented!(),
            };
        }
    }
}