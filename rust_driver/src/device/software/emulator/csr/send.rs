use log::trace;

use crate::device::software::emulator::device_api::csr::RegisterOperation;

register_queue_csr!(0x9000, Emulator, Send, SEND);

impl<UA: Agent> RegisterOperation for EmulatorRegistersSendAddressHighHandler<'_, UA> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!("Read send address high part {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        trace!("Write send address high part {val:#010X}",);
        self.reg.write(val);
    }
}

impl<UA: Agent> RegisterOperation for EmulatorRegistersSendAddressLowHandler<'_, UA> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!("Read send address low part {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        trace!("Write send address low part {val:#010X}",);
        self.reg.write(val);
    }
}

impl<UA: Agent> RegisterOperation for EmulatorRegistersSendHeadHandler<'_, UA> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!("Read send head {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        let old = self.reg.read();
        self.reg.write(val);

        trace!("Write send head {old:010X} -> {val:010X}");

        self.dev.send_queue().doorbell(val);
    }
}

impl<UA: Agent> RegisterOperation for EmulatorRegistersSendTailHandler<'_, UA> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!("Read send tail {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        let old = self.reg.read();
        self.reg.write(val);

        trace!("Write send tail {old:010X} -> {val:010X}",);
    }
}
