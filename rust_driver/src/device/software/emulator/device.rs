use core::net::IpAddr;
use std::sync::Arc;

use blue_rdma_device::device_api::csr::{RegistersQueue, RegistersQueueAddress};
use blue_rdma_device::device_api::{ControlStatusRegisters, RawDevice};
use blue_rdma_device::Emulator;
use core_affinity::CoreId;
use parking_lot::Mutex;

use super::csr_proxy::{CommandRequest, CommandResponse, MetaReport, Send};
use crate::device::ringbuf::Ringbuf;
use crate::device::software::DescriptorScheduler;
use crate::device::{
    constants, DeviceAdaptor, DeviceError, ToCardCtrlRbDesc, ToCardRb, ToHostCtrlRbDesc, ToHostRb, ToHostWorkRbDesc,
    ToHostWorkRbDescError,
};
use crate::utils::Buffer;
use crate::{AlignedMemory, SchedulerStrategy};

type ToCardCtrlRb = Ringbuf<
    CommandRequest,
    AlignedMemory,
    { constants::RINGBUF_DEPTH },
    { constants::RINGBUF_ELEM_SIZE },
    { constants::RINGBUF_PAGE_SIZE },
>;

type ToHostCtrlRb = Ringbuf<
    CommandResponse,
    AlignedMemory,
    { constants::RINGBUF_DEPTH },
    { constants::RINGBUF_ELEM_SIZE },
    { constants::RINGBUF_PAGE_SIZE },
>;

type ToCardWorkRb = Ringbuf<
    Send,
    Buffer,
    { constants::RINGBUF_DEPTH },
    { constants::RINGBUF_ELEM_SIZE },
    { constants::RINGBUF_PAGE_SIZE },
>;

type ToHostWorkRb = Ringbuf<
    MetaReport,
    AlignedMemory,
    { constants::RINGBUF_DEPTH },
    { constants::RINGBUF_ELEM_SIZE },
    { constants::RINGBUF_PAGE_SIZE },
>;

#[derive(Debug, Clone)]
pub(crate) struct EmulatorDevice<S: SchedulerStrategy>(Arc<EmulatorDeviceInner<S>>);

#[allow(clippy::struct_field_names)]
#[derive(Debug)]
pub(crate) struct EmulatorDeviceInner<S: SchedulerStrategy> {
    command_request: Arc<Mutex<ToCardCtrlRb>>,
    command_response: Arc<Mutex<ToHostCtrlRb>>,
    meta_report: Arc<Mutex<ToHostWorkRb>>,
    send: Arc<DescriptorScheduler<S>>,

    dev: Arc<Emulator>,
}

impl<S: SchedulerStrategy> EmulatorDevice<S> {
    pub(crate) fn new(
        strategy: S,
        core_id: Option<CoreId>,
        scheduler_size: u32,
        tun_ip: IpAddr,
    ) -> Result<Self, DeviceError> {
        let dev = Emulator::new_emulator(tun_ip);

        let buffer = AlignedMemory::new(constants::RINGBUF_PAGE_SIZE)?;
        dev.csrs()
            .cmd_request()
            .addr()
            .write(buffer.as_ref().as_ptr() as usize as _);
        let command_request = ToCardCtrlRb::new(CommandRequest(Arc::clone(&dev)), buffer);

        let buffer = AlignedMemory::new(constants::RINGBUF_PAGE_SIZE)?;
        dev.csrs()
            .cmd_response()
            .addr()
            .write(buffer.as_ref().as_ptr() as usize as _);
        let command_response = ToHostCtrlRb::new(CommandResponse(Arc::clone(&dev)), buffer);

        let buffer = AlignedMemory::new(constants::RINGBUF_PAGE_SIZE)?;
        dev.csrs().send().addr().write(buffer.as_ref().as_ptr() as usize as _);
        let send = ToCardWorkRb::new(Send(Arc::clone(&dev)), Buffer::AlignedMemory(buffer));

        let buffer = AlignedMemory::new(constants::RINGBUF_PAGE_SIZE)?;
        dev.csrs()
            .meta_report()
            .addr()
            .write(buffer.as_ref().as_ptr() as usize as _);
        let meta_report = ToHostWorkRb::new(MetaReport(Arc::clone(&dev)), buffer);

        let send = Arc::new(DescriptorScheduler::new(
            strategy,
            Mutex::new(send),
            core_id,
            scheduler_size,
        ));

        let dev = Self(Arc::new(EmulatorDeviceInner {
            command_request: Arc::new(Mutex::new(command_request)),
            command_response: Arc::new(Mutex::new(command_response)),
            meta_report: Arc::new(Mutex::new(meta_report)),
            send,
            dev,
        }));

        Ok(dev)
    }
}

impl<S: SchedulerStrategy> DeviceAdaptor for EmulatorDevice<S> {
    fn to_card_ctrl_rb(&self) -> Arc<dyn ToCardRb<ToCardCtrlRbDesc>> {
        Arc::clone(&self.0.command_request) as _
    }

    fn to_host_ctrl_rb(&self) -> Arc<dyn ToHostRb<ToHostCtrlRbDesc>> {
        Arc::clone(&self.0.command_response) as _
    }

    fn to_card_work_rb(&self) -> Arc<dyn ToCardRb<Box<crate::device::ToCardWorkRbDesc>>> {
        Arc::clone(&self.0.send) as _
    }

    fn to_host_work_rb(&self) -> Arc<dyn ToHostRb<ToHostWorkRbDesc>> {
        Arc::clone(&self.0.meta_report) as _
    }

    fn read_csr(&self, addr: usize) -> Result<u32, DeviceError> {
        todo!()
    }

    fn write_csr(&self, addr: usize, data: u32) -> Result<(), DeviceError> {
        todo!()
    }

    fn get_phys_addr(&self, virt_addr: usize) -> Result<usize, DeviceError> {
        Ok(virt_addr)
    }

    fn use_hugepage(&self) -> bool {
        false
    }
}

#[allow(clippy::unwrap_used, clippy::unwrap_in_result)]
impl ToCardRb<ToCardCtrlRbDesc> for Mutex<ToCardCtrlRb> {
    fn push(&self, desc: ToCardCtrlRbDesc) -> Result<(), DeviceError> {
        let mut guard = self.lock();
        let mut writer = guard.write();

        let mem = writer.next().unwrap(); // If blockly write desc fail, it should panic
        log::debug!("{:?}", &desc);
        desc.write(mem);

        Ok(())
    }
}

impl ToHostRb<ToHostCtrlRbDesc> for Mutex<ToHostCtrlRb> {
    fn pop(&self) -> Result<ToHostCtrlRbDesc, DeviceError> {
        let mut guard = self.lock();
        let mut reader = guard.read();
        let mem = reader.next()?;
        let desc = ToHostCtrlRbDesc::read(mem)?;
        log::debug!("{:?}", &desc);
        Ok(desc)
    }
}

impl ToHostRb<ToHostWorkRbDesc> for Mutex<ToHostWorkRb> {
    fn pop(&self) -> Result<ToHostWorkRbDesc, DeviceError> {
        let mut guard = self.lock();
        let mut reader = guard.read();

        let mem = reader.next()?;
        let mut read_res = ToHostWorkRbDesc::read(mem);

        loop {
            match read_res {
                Ok(desc) => break Ok(desc),
                Err(ToHostWorkRbDescError::DeviceError(e)) => {
                    return Err(e);
                }
                Err(ToHostWorkRbDescError::Incomplete(incomplete_desc)) => {
                    let next_mem = reader.next()?;
                    read_res = incomplete_desc.read(next_mem);
                }
            }
        }
    }
}
