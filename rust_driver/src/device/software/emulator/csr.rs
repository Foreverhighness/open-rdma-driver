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
pub struct EmulatorCsrs {
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

// TODO(fh): Remove pub?
pub(super) const REGISTERS_COMMAND_REQUEST_BASE_ADDR: u64 = 0x8000;
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

// TODO(fh): Remove pub?
pub(super) const REGISTERS_COMMAND_RESPONSE_BASE_ADDR: u64 = 0x0000;
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

// TODO(fh): Remove pub?
pub(super) const REGISTERS_META_REPORT_BASE_ADDR: u64 = 0x1000;
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

// TODO(fh): Remove pub?
pub(super) const REGISTERS_SEND_BASE_ADDR: u64 = 0x9000;
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
pub struct EmulatorQueueAddressLow<const BASE_ADDR: u64>(AtomicU32);
#[derive(Debug, Default)]
pub struct EmulatorQueueAddressHigh<const BASE_ADDR: u64>(AtomicU32);
#[derive(Debug, Default)]
pub struct EmulatorQueueHead<const BASE_ADDR: u64>(AtomicU32);
#[derive(Debug, Default)]
pub struct EmulatorQueueTail<const BASE_ADDR: u64>(AtomicU32);
#[derive(Debug, Default)]
pub struct EmulatorQueueAddress<const BASE_ADDR: u64>(
    EmulatorQueueAddressHigh<BASE_ADDR>,
    EmulatorQueueAddressLow<BASE_ADDR>,
);
#[derive(Debug, Default)]
pub struct EmulatorRegistersQueue<const BASE_ADDR: u64> {
    addr: EmulatorQueueAddress<BASE_ADDR>,
    head: EmulatorQueueHead<BASE_ADDR>,
    tail: EmulatorQueueTail<BASE_ADDR>,
}

impl<const BASE_ADDR: u64> RegisterQueueAddressLow for EmulatorQueueAddressLow<BASE_ADDR> {}
impl<const BASE_ADDR: u64> RegisterOperation for EmulatorQueueAddressLow<BASE_ADDR> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        self.0.read()
    }

    fn write(&self, val: Self::Output) {
        self.0.write(val)
    }
}
impl<const BASE_ADDR: u64> RegisterQueueAddressHigh for EmulatorQueueAddressHigh<BASE_ADDR> {}
impl<const BASE_ADDR: u64> RegisterOperation for EmulatorQueueAddressHigh<BASE_ADDR> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        self.0.read()
    }

    fn write(&self, val: Self::Output) {
        self.0.write(val)
    }
}
impl<const BASE_ADDR: u64> RegisterQueueHead for EmulatorQueueHead<BASE_ADDR> {}
impl<const BASE_ADDR: u64> RegisterOperation for EmulatorQueueHead<BASE_ADDR> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        self.0.read()
    }

    fn write(&self, val: Self::Output) {
        self.0.write(val)
    }
}
impl<const BASE_ADDR: u64> RegisterQueueTail for EmulatorQueueTail<BASE_ADDR> {}
impl<const BASE_ADDR: u64> RegisterOperation for EmulatorQueueTail<BASE_ADDR> {
    type Output = u32;

    fn read(&self) -> Self::Output {
        self.0.read()
    }

    fn write(&self, val: Self::Output) {
        self.0.write(val)
    }
}
impl<const BASE_ADDR: u64> RegistersQueueAddress for EmulatorQueueAddress<BASE_ADDR> {
    type High = EmulatorQueueAddressHigh<BASE_ADDR>;
    type Low = EmulatorQueueAddressLow<BASE_ADDR>;

    fn low(&self) -> &Self::Low {
        &self.1
    }

    fn high(&self) -> &Self::High {
        &self.0
    }
}
impl<const BASE_ADDR: u64> RegistersQueue for EmulatorRegistersQueue<BASE_ADDR> {
    type Address = EmulatorQueueAddress<BASE_ADDR>;
    type Head = EmulatorQueueHead<BASE_ADDR>;
    type Tail = EmulatorQueueTail<BASE_ADDR>;

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
