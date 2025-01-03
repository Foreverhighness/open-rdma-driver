use log::trace;

use crate::device::software::emulator::device_api::csr::RegisterOperation;

register_queue_csr!(0x1000, Emulator, MetaReport, META_REPORT);

impl<UA: Agent, DC: Client> RegisterOperation for EmulatorRegistersMetaReportAddressHighHandler<'_, UA, DC> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!("Read meta_report address high part {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        trace!("Write meta_report address high part {val:#010X}",);
        self.reg.write(val);
    }
}

impl<UA: Agent, DC: Client> RegisterOperation for EmulatorRegistersMetaReportAddressLowHandler<'_, UA, DC> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!("Read meta_report address low part {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        trace!("Write meta_report address low part {val:#010X}",);
        self.reg.write(val);
    }
}

impl<UA: Agent, DC: Client> RegisterOperation for EmulatorRegistersMetaReportHeadHandler<'_, UA, DC> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!("Read meta_report head {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        let old = self.reg.read();
        self.reg.write(val);

        trace!("Write meta_report head {old:010X} -> {val:010X}");
    }
}

impl<UA: Agent, DC: Client> RegisterOperation for EmulatorRegistersMetaReportTailHandler<'_, UA, DC> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        let val = self.reg.read();
        trace!("Read meta_report tail {val:010X}",);
        val
    }

    fn write(&self, val: Self::Output) {
        let old = self.reg.read();
        self.reg.write(val);

        trace!("Write meta_report tail {old:010X} -> {val:010X}",);
    }
}
