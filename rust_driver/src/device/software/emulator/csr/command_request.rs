use log::trace;

use crate::device::software::emulator::device_api::csr::RegisterOperation;
use crate::device::software::emulator::queues::command_request::CommandRequestQueueAbility;

// Register common part
register_queue_csr!(0x8000, Emulator, CommandRequest, COMMAND_REQUEST);

impl<UA: Agent> RegisterOperation for EmulatorRegistersCommandRequestAddressHighHandler<'_, UA> {
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

impl<UA: Agent> RegisterOperation for EmulatorRegistersCommandRequestAddressLowHandler<'_, UA> {
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

impl<UA: Agent> RegisterOperation for EmulatorRegistersCommandRequestHeadHandler<'_, UA> {
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

        let _ = self.reg.write_cnt.fetch_add(1, core::sync::atomic::Ordering::Relaxed);

        trace!(
            "Write {} tail {old:010X} -> {val:010X}",
            std::path::Path::new(file!()).file_stem().unwrap().to_str().unwrap()
        );

        self.dev.doorbell(val);
    }
}

impl<UA: Agent> RegisterOperation for EmulatorRegistersCommandRequestTailHandler<'_, UA> {
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

        let _ = self.reg.write_cnt.fetch_add(1, core::sync::atomic::Ordering::Relaxed);

        trace!(
            "Write {} tail {old:010X} -> {val:010X}",
            std::path::Path::new(file!()).file_stem().unwrap().to_str().unwrap()
        );
    }
}
