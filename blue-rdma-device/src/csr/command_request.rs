use log::trace;

use crate::device_api::csr::RegisterOperation;

// Register common part
register_queue_csr!(0x8000, Emulator, CommandRequest, COMMAND_REQUEST);

impl<UA: Agent, DC: Client> RegisterOperation for EmulatorRegistersCommandRequestAddressHighHandler<'_, UA, DC> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!("Read command_request address high part {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        trace!("Write command_request address high part {val:#010X}",);
        self.reg.write(val);
    }
}

impl<UA: Agent, DC: Client> RegisterOperation for EmulatorRegistersCommandRequestAddressLowHandler<'_, UA, DC> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!("Read command_request address low part {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        trace!("Write command_request address low part {val:#010X}",);
        self.reg.write(val);
    }
}

impl<UA: Agent, DC: Client> RegisterOperation for EmulatorRegistersCommandRequestHeadHandler<'_, UA, DC> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!("Read command_request head {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        let old = self.reg.read();
        self.reg.write(val);

        trace!("Write command_request head {old:010X} -> {val:010X}");

        self.dev.command_request_queue().doorbell(val);
    }
}

impl<UA: Agent, DC: Client> RegisterOperation for EmulatorRegistersCommandRequestTailHandler<'_, UA, DC> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!("Read command_request tail {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        let old = self.reg.read();
        self.reg.write(val);

        trace!("Write command_request tail {old:010X} -> {val:010X}",);
    }
}
