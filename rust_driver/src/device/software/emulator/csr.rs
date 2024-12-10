//! Control Status Register for emulator use
//! In contrast to the Driver layer definition, `repr(C)` is not required in Emulator implementation.

use core::sync::atomic::AtomicU32;

use super::device_api::csr::{
    RegisterOperation, RegisterQueueAddressHigh, RegisterQueueAddressLow, RegisterQueueHead, RegisterQueueTail,
    RegistersCommandRequest, RegistersCommandResponse, RegistersMetaReport, RegistersQueue, RegistersQueueAddress,
    RegistersSend,
};
use super::device_api::ControlStatusRegisters;

// TODO(fh): AtomicU32 is not a register conceptually, so I need remove this impl
impl RegisterOperation for AtomicU32 {
    type Output = u32;

    fn read(&self) -> Self::Output {
        self.load(core::sync::atomic::Ordering::Relaxed)
    }

    fn write(&self, val: Self::Output) {
        self.store(val, core::sync::atomic::Ordering::Relaxed)
    }
}

#[derive(Debug, Default)]
pub(super) struct EmulatorCsrs {
    cmd_request: EmulatorRegistersCommandRequest,
    cmd_response: EmulatorRegistersCommandResponse,
    meta_report: EmulatorRegistersMetaReport,
    send: EmulatorRegistersSend,
}

impl ControlStatusRegisters for EmulatorCsrs {
    type CmdRequest = EmulatorRegistersCommandRequest;
    type CmdResponse = EmulatorRegistersCommandResponse;
    type MetaReport = EmulatorRegistersMetaReport;
    type Send = EmulatorRegistersSend;

    fn cmd_request(&self) -> &Self::CmdRequest {
        &self.cmd_request
    }

    fn cmd_response(&self) -> &Self::CmdResponse {
        &self.cmd_response
    }

    fn send(&self) -> &Self::Send {
        &self.send
    }

    fn meta_report(&self) -> &Self::MetaReport {
        &self.meta_report
    }
}

const REGISTERS_COMMAND_REQUEST_BASE_ADDR: u64 = 0x8000;
/// Struct that holds registers related to one command request queue.
#[derive(Debug, Default)]
pub struct EmulatorRegistersCommandRequest(EmulatorRegistersQueue<REGISTERS_COMMAND_REQUEST_BASE_ADDR>);
impl RegistersCommandRequest for EmulatorRegistersCommandRequest {}
impl RegistersQueue for EmulatorRegistersCommandRequest {
    type Address = EmulatorQueueAddress<REGISTERS_COMMAND_REQUEST_BASE_ADDR>;
    type Head = EmulatorQueueHead<REGISTERS_COMMAND_REQUEST_BASE_ADDR>;
    type Tail = EmulatorQueueTail<REGISTERS_COMMAND_REQUEST_BASE_ADDR>;

    fn addr(&self) -> &Self::Address {
        self.0.addr()
    }

    fn head(&self) -> &Self::Head {
        self.0.head()
    }

    fn tail(&self) -> &Self::Tail {
        self.0.tail()
    }
}

const REGISTERS_COMMAND_RESPONSE_BASE_ADDR: u64 = 0x0000;
/// Struct that holds registers related to one command response queue.
#[derive(Debug, Default)]
pub struct EmulatorRegistersCommandResponse(EmulatorRegistersQueue<REGISTERS_COMMAND_RESPONSE_BASE_ADDR>);
impl RegistersCommandResponse for EmulatorRegistersCommandResponse {}
impl RegistersQueue for EmulatorRegistersCommandResponse {
    type Address = EmulatorQueueAddress<REGISTERS_COMMAND_RESPONSE_BASE_ADDR>;
    type Head = EmulatorQueueHead<REGISTERS_COMMAND_RESPONSE_BASE_ADDR>;
    type Tail = EmulatorQueueTail<REGISTERS_COMMAND_RESPONSE_BASE_ADDR>;

    fn addr(&self) -> &Self::Address {
        self.0.addr()
    }

    fn head(&self) -> &Self::Head {
        self.0.head()
    }

    fn tail(&self) -> &Self::Tail {
        self.0.tail()
    }
}

const REGISTERS_META_REPORT_BASE_ADDR: u64 = 0x1000;
/// Struct that holds registers related to one meta report queue.
#[derive(Debug, Default)]
pub struct EmulatorRegistersMetaReport(EmulatorRegistersQueue<REGISTERS_META_REPORT_BASE_ADDR>);
impl RegistersMetaReport for EmulatorRegistersMetaReport {}
impl RegistersQueue for EmulatorRegistersMetaReport {
    type Address = EmulatorQueueAddress<REGISTERS_META_REPORT_BASE_ADDR>;
    type Head = EmulatorQueueHead<REGISTERS_META_REPORT_BASE_ADDR>;
    type Tail = EmulatorQueueTail<REGISTERS_META_REPORT_BASE_ADDR>;

    fn addr(&self) -> &Self::Address {
        self.0.addr()
    }

    fn head(&self) -> &Self::Head {
        self.0.head()
    }

    fn tail(&self) -> &Self::Tail {
        self.0.tail()
    }
}

const REGISTERS_SEND_BASE_ADDR: u64 = 0x9000;
/// Struct that holds registers related to one send queue.
#[derive(Debug, Default)]
pub struct EmulatorRegistersSend(EmulatorRegistersQueue<REGISTERS_SEND_BASE_ADDR>);
impl RegistersSend for EmulatorRegistersSend {}
impl RegistersQueue for EmulatorRegistersSend {
    type Address = EmulatorQueueAddress<REGISTERS_SEND_BASE_ADDR>;
    type Head = EmulatorQueueHead<REGISTERS_SEND_BASE_ADDR>;
    type Tail = EmulatorQueueTail<REGISTERS_SEND_BASE_ADDR>;

    fn addr(&self) -> &Self::Address {
        self.0.addr()
    }

    fn head(&self) -> &Self::Head {
        self.0.head()
    }

    fn tail(&self) -> &Self::Tail {
        self.0.tail()
    }
}

#[derive(Debug, Default)]
pub struct EmulatorQueueAddressLow<const BaseAddr: u64>(AtomicU32);
#[derive(Debug, Default)]
pub struct EmulatorQueueAddressHigh<const BaseAddr: u64>(AtomicU32);
#[derive(Debug, Default)]
pub struct EmulatorQueueHead<const BaseAddr: u64>(AtomicU32);
#[derive(Debug, Default)]
pub struct EmulatorQueueTail<const BaseAddr: u64>(AtomicU32);
#[derive(Debug, Default)]
pub struct EmulatorQueueAddress<const BaseAddr: u64>(
    EmulatorQueueAddressHigh<BaseAddr>,
    EmulatorQueueAddressLow<BaseAddr>,
);
#[derive(Debug, Default)]
pub struct EmulatorRegistersQueue<const BaseAddr: u64> {
    addr: EmulatorQueueAddress<BaseAddr>,
    head: EmulatorQueueHead<BaseAddr>,
    tail: EmulatorQueueTail<BaseAddr>,
}

impl<const BaseAddr: u64> RegisterQueueAddressLow for EmulatorQueueAddressLow<BaseAddr> {}
impl<const BaseAddr: u64> RegisterOperation for EmulatorQueueAddressLow<BaseAddr> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        self.0.read()
    }

    fn write(&self, val: Self::Output) {
        self.0.write(val)
    }
}
impl<const BaseAddr: u64> RegisterQueueAddressHigh for EmulatorQueueAddressHigh<BaseAddr> {}
impl<const BaseAddr: u64> RegisterOperation for EmulatorQueueAddressHigh<BaseAddr> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        self.0.read()
    }

    fn write(&self, val: Self::Output) {
        self.0.write(val)
    }
}
impl<const BaseAddr: u64> RegisterQueueHead for EmulatorQueueHead<BaseAddr> {}
impl<const BaseAddr: u64> RegisterOperation for EmulatorQueueHead<BaseAddr> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        self.0.read()
    }

    fn write(&self, val: Self::Output) {
        self.0.write(val)
    }
}
impl<const BaseAddr: u64> RegisterQueueTail for EmulatorQueueTail<BaseAddr> {}
impl<const BaseAddr: u64> RegisterOperation for EmulatorQueueTail<BaseAddr> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        self.0.read()
    }

    fn write(&self, val: Self::Output) {
        self.0.write(val)
    }
}
impl<const BaseAddr: u64> RegistersQueueAddress for EmulatorQueueAddress<BaseAddr> {
    type High = EmulatorQueueAddressHigh<BaseAddr>;
    type Low = EmulatorQueueAddressLow<BaseAddr>;

    fn low(&self) -> &Self::Low {
        &self.1
    }

    fn high(&self) -> &Self::High {
        &self.0
    }
}
impl<const BaseAddr: u64> RegistersQueue for EmulatorRegistersQueue<BaseAddr> {
    type Address = EmulatorQueueAddress<BaseAddr>;
    type Head = EmulatorQueueHead<BaseAddr>;
    type Tail = EmulatorQueueTail<BaseAddr>;

    fn addr(&self) -> &Self::Address {
        &self.addr
    }

    fn head(&self) -> &Self::Head {
        &self.head
    }

    fn tail(&self) -> &Self::Tail {
        &self.tail
    }
}
