use log::trace;

use crate::device::software::emulator::device_api::csr::RegisterOperation;

register_queue_csr!(0x1000, Emulator, MetaReport, META_REPORT);

impl<UA: Agent> RegisterOperation for EmulatorRegistersMetaReportAddressHighHandler<'_, UA> {
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

impl<UA: Agent> RegisterOperation for EmulatorRegistersMetaReportAddressLowHandler<'_, UA> {
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

impl<UA: Agent> RegisterOperation for EmulatorRegistersMetaReportHeadHandler<'_, UA> {
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
        let old = self.reg.read();
        self.reg.write(val);

        trace!(
            "Write {} tail {old:010X} -> {val:010X}",
            std::path::Path::new(file!()).file_stem().unwrap().to_str().unwrap()
        );
    }
}

impl<UA: Agent> RegisterOperation for EmulatorRegistersMetaReportTailHandler<'_, UA> {
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
        let old = self.reg.read();
        self.reg.write(val);

        trace!(
            "Write {} tail {old:010X} -> {val:010X}",
            std::path::Path::new(file!()).file_stem().unwrap().to_str().unwrap()
        );
    }
}
