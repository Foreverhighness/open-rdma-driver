use core::sync::atomic::AtomicU32;

use crate::device::software::emulator::device_api::csr::{
    RegisterOperation, RegisterQueueAddressHigh, RegisterQueueAddressLow, RegisterQueueHead, RegisterQueueTail,
    RegistersQueue, RegistersQueueAddress,
};

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
