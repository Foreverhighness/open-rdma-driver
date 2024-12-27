use super::Agent;
use crate::device::software::emulator::device_api::csr::{RegisterOperation, RegisterReset};
use crate::device::software::emulator::dma::Client;
use crate::device::software::emulator::DeviceInner;

pub const REGISTERS_SOFT_RESET: u64 = 0x0002_1000;
pub const REGISTERS_HW_VERSION: u64 = 0x0002_0000;
pub const HARDWARE_VERSION: u32 = 2024042901;

#[derive(Debug, Default)]
pub(crate) struct EmulatorRegisterReset {
    val: core::sync::atomic::AtomicU32,
}

impl EmulatorRegisterReset {
    pub fn read(&self) -> u32 {
        self.val.load(core::sync::atomic::Ordering::Relaxed)
    }

    pub fn write(&self, val: u32) {
        self.val.store(val, core::sync::atomic::Ordering::Relaxed)
    }
}

pub(crate) struct EmulatorRegisterResetHandler<'h, UA: Agent, DC: Client> {
    reg: &'h EmulatorRegisterReset,
    dev: &'h DeviceInner<UA, DC>,
}

impl<'h, UA: Agent, DC: Client> EmulatorRegisterResetHandler<'h, UA, DC> {
    pub(crate) fn new<'r, 'd>(reg: &'r EmulatorRegisterReset, dev: &'d DeviceInner<UA, DC>) -> Self
    where
        'r: 'h,
        'd: 'h,
    {
        Self { reg, dev }
    }
}

impl<UA: Agent, DC: Client> RegisterReset for EmulatorRegisterResetHandler<'_, UA, DC> {}
impl<UA: Agent, DC: Client> RegisterOperation for EmulatorRegisterResetHandler<'_, UA, DC> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        log::trace!("Read reset {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        log::trace!("Write reset {val:#010X}",);
        match val {
            0 => (),
            1 => {
                self.reg.write(1);
                self.dev.reset();
                self.reg.write(0);
            }
            _ => unimplemented!("reset with value {val}"),
        }
    }
}

impl<UA: Agent, DC: Client> DeviceInner<UA, DC> {
    pub(crate) fn reset(&self) {
        assert!(self.udp_agent.get().is_none());

        self.csrs.cmd_request.addr.write(0);
        self.csrs.cmd_request.head.write(0);
        self.csrs.cmd_request.tail.write(0);

        self.csrs.cmd_response.addr.write(0);
        self.csrs.cmd_response.head.write(0);
        self.csrs.cmd_response.tail.write(0);

        self.csrs.send.addr.write(0);
        self.csrs.send.head.write(0);
        self.csrs.send.tail.write(0);

        self.csrs.meta_report.addr.write(0);
        self.csrs.meta_report.head.write(0);
        self.csrs.meta_report.tail.write(0);
    }
}
