// region:csr_proxy

use std::sync::Arc;

use crate::device::ringbuf::{CsrReaderAdaptor, CsrWriterAdaptor};
use crate::device_api::csr::{RegisterOperation, RegistersQueue};
use crate::device_api::{ControlStatusRegisters, RawDevice};
use crate::Emulator;
use crate::device::DeviceError;

#[derive(Debug)]
pub(crate) struct CsrProxy {
    dev: Arc<Emulator>,
}

#[derive(Debug)]
pub(crate) struct CommandRequest(pub Arc<Emulator>);

impl CsrWriterAdaptor for CommandRequest {
    fn write_head(&self, data: u32) -> Result<(), DeviceError> {
        Ok(self.0.csrs().cmd_request().head().write(data))
    }

    fn read_tail(&self) -> Result<u32, DeviceError> {
        Ok(self.0.csrs().cmd_request().tail().read())
    }
}

#[derive(Debug)]
pub(crate) struct CommandResponse(pub Arc<Emulator>);

impl CsrReaderAdaptor for CommandResponse {
    fn write_tail(&self, data: u32) -> Result<(), DeviceError> {
        Ok(self.0.csrs().cmd_response().tail().write(data))
    }

    fn read_head(&self) -> Result<u32, DeviceError> {
        Ok(self.0.csrs().cmd_response().head().read())
    }
}

#[derive(Debug)]
pub(crate) struct Send(pub Arc<Emulator>);

impl CsrWriterAdaptor for Send {
    fn write_head(&self, data: u32) -> Result<(), DeviceError> {
        Ok(self.0.csrs().send().head().write(data))
    }

    fn read_tail(&self) -> Result<u32, DeviceError> {
        Ok(self.0.csrs().send().tail().read())
    }
}

#[derive(Debug)]
pub(crate) struct MetaReport(pub Arc<Emulator>);

impl CsrReaderAdaptor for MetaReport {
    fn write_tail(&self, data: u32) -> Result<(), DeviceError> {
        Ok(self.0.csrs().meta_report().tail().write(data))
    }

    fn read_head(&self) -> Result<u32, DeviceError> {
        Ok(self.0.csrs().meta_report().head().read())
    }
}

// endregion:csr_proxy
