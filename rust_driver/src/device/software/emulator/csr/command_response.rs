use log::trace;

use crate::device::software::emulator::device_api::csr::RegisterOperation;

register_queue_csr!(0x0000, Emulator, CommandResponse, COMMAND_RESPONSE);

impl<UA: Agent, DC: Client> RegisterOperation for EmulatorRegistersCommandResponseAddressHighHandler<'_, UA, DC> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!("Read command_response address high part {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        trace!("Write command_response address high part {val:#010X}",);
        self.reg.write(val);
    }
}

impl<UA: Agent, DC: Client> RegisterOperation for EmulatorRegistersCommandResponseAddressLowHandler<'_, UA, DC> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!("Read command_response address low part {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        trace!("Write command_response address low part {val:#010X}",);
        self.reg.write(val);
    }
}

impl<UA: Agent, DC: Client> RegisterOperation for EmulatorRegistersCommandResponseHeadHandler<'_, UA, DC> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!("Read command_response head {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        let old = self.reg.read();
        self.reg.write(val);

        trace!("Write command_response head {old:010X} -> {val:010X}");
    }
}

impl<UA: Agent, DC: Client> RegisterOperation for EmulatorRegistersCommandResponseTailHandler<'_, UA, DC> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!("Read command_response tail {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        let old = self.reg.read();
        self.reg.write(val);

        trace!("Write command_response tail {old:010X} -> {val:010X}",);
    }
}
