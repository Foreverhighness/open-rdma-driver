//! Control Status Register Proxy

use core::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread::JoinHandle;

use super::super::device_api::{self, ControlStatusRegisters};
use super::rpc::{self, BarIoInfo};

/// CSR Proxy is responsible for forwarding CSR read/write requests from the Python Server to the emulator
/// and forwarding the response to the Python Server
#[derive(Debug)]
pub struct Proxy<R: rpc::Client, Dev: device_api::RawDevice> {
    client_id: u64,

    rpc: R,
    dev: Arc<Dev>,

    stop: Arc<AtomicBool>,
}

impl<R: rpc::Client, Dev: device_api::RawDevice> Proxy<R, Dev> {
    pub fn new(client_id: u64, rpc: R, dev: Arc<Dev>) -> Self {
        Self {
            client_id,
            rpc,
            dev,
            stop: Arc::default(),
        }
    }

    fn recv_read_request(&self) -> Option<BarIoInfo> {
        let mut request = BarIoInfo::new();
        while !self.stop.load(core::sync::atomic::Ordering::Relaxed) {
            unsafe {
                self.rpc.c_getPcieBarReadReq(&raw mut request, self.client_id);
            }
            if request.valid == 1 {
                // log::debug!("recv csr read request {request:?}");
                return Some(request);
            }
            core::hint::spin_loop();
        }
        None
    }

    fn handle_read_request(&self, req: &BarIoInfo) {
        let &BarIoInfo { addr, pci_tag, .. } = req;
        let value = self.dev.csrs().read(addr);
        let mut response = BarIoInfo::new_read_response(pci_tag, value.into());
        unsafe {
            self.rpc.c_putPcieBarReadResp(self.client_id, &raw mut response);
        }
    }

    fn recv_write_request(&self) -> Option<BarIoInfo> {
        let mut request = BarIoInfo::new();
        while !self.stop.load(core::sync::atomic::Ordering::Relaxed) {
            unsafe {
                self.rpc.c_getPcieBarWriteReq(&raw mut request, self.client_id);
            }
            if request.valid == 1 {
                // log::debug!("recv csr write request {request:?}");
                return Some(request);
            }
            core::hint::spin_loop();
        }
        None
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
        let stop = Arc::clone(&self.stop);
        let proxy_read = Arc::new(self);
        let proxy_write = Arc::clone(&proxy_read);

        let handle_read = std::thread::spawn(move || {
            while let Some(req) = proxy_read.recv_read_request() {
                proxy_read.handle_read_request(&req);
            }
        });
        let handle_write = std::thread::spawn(move || {
            while let Some(req) = proxy_write.recv_write_request() {
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
    use super::super::super::csr::command_request::REGISTERS_COMMAND_REQUEST_BASE_ADDR;
    use super::super::super::csr::command_response::REGISTERS_COMMAND_RESPONSE_BASE_ADDR;
    use super::super::super::csr::meta_report::REGISTERS_META_REPORT_BASE_ADDR;
    use super::super::super::csr::reset::REGISTERS_SOFT_RESET;
    use super::super::super::csr::send::REGISTERS_SEND_BASE_ADDR;
    use super::device_api::csr::{RegisterOperation, RegistersQueue, RegistersQueueAddress};
    use super::device_api::ControlStatusRegisters;
    use super::ControlStatusRegistersExt;
    use crate::device::software::emulator::csr::reset::{HARDWARE_VERSION, REGISTERS_HW_VERSION};

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
                REGISTERS_SOFT_RESET => {
                    let csr = self.reset();
                    csr.read()
                }
                REGISTERS_HW_VERSION => HARDWARE_VERSION,
                _ => unimplemented!("read at addr {addr:#018x}"),
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
                REGISTERS_SOFT_RESET => {
                    let csr = self.reset();
                    csr.write(val);
                }
                _ => unimplemented!("write at addr {addr:#018x}, val: {val}"),
            };
        }
    }
}
