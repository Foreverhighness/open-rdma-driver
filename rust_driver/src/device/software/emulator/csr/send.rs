use log::trace;

use crate::device::software::emulator::device_api::csr::RegisterOperation;

register_queue_csr!(0x9000, Emulator, Send, SEND);

impl<UA: Agent> RegisterOperation for EmulatorRegistersSendAddressHighHandler<'_, UA> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!(
            "Read {} address high part {val:010X}",
            std::path::Path::new(file!()).file_stem().unwrap().to_str().unwrap()
        );
        val
    }

    fn write(&self, val: Self::Output) {
        trace!(
            "Write {} address high part {val:#010X}",
            std::path::Path::new(file!()).file_stem().unwrap().to_str().unwrap()
        );
        self.reg.write(val);
    }
}

impl<UA: Agent> RegisterOperation for EmulatorRegistersSendAddressLowHandler<'_, UA> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!(
            "Read {} address low part {val:010X}",
            std::path::Path::new(file!()).file_stem().unwrap().to_str().unwrap()
        );
        val
    }

    fn write(&self, val: Self::Output) {
        trace!(
            "Write {} address low part {val:#010X}",
            std::path::Path::new(file!()).file_stem().unwrap().to_str().unwrap()
        );
        self.reg.write(val);
    }
}

impl<UA: Agent> RegisterOperation for EmulatorRegistersSendHeadHandler<'_, UA> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!(
            "Read {} head {val:010X}",
            std::path::Path::new(file!()).file_stem().unwrap().to_str().unwrap()
        );
        val
    }

    fn write(&self, val: Self::Output) {
        todo!()
    }
}

impl<UA: Agent> RegisterOperation for EmulatorRegistersSendTailHandler<'_, UA> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!(
            "Read {} tail {val:010X}",
            std::path::Path::new(file!()).file_stem().unwrap().to_str().unwrap()
        );
        val
    }

    fn write(&self, val: Self::Output) {
        todo!()
    }
}
