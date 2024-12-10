pub trait RegisterOperation {
    type Output;
    fn read(&self) -> Self::Output;
    fn write(&self, val: Self::Output);
}
pub trait RegisterQueueAddressLow: RegisterOperation<Output = u32> {}
pub trait RegisterQueueAddressHigh: RegisterOperation<Output = u32> {}
pub trait RegisterQueueHead: RegisterOperation<Output = u32> {}
pub trait RegisterQueueTail: RegisterOperation<Output = u32> {}
pub trait RegistersQueueAddress {
    type High<'a>: RegisterQueueAddressHigh
    where
        Self: 'a;
    type Low<'a>: RegisterQueueAddressLow
    where
        Self: 'a;

    fn high(&self) -> Self::High<'_>;
    fn low(&self) -> Self::Low<'_>;

    fn read(&self) -> u64 {
        let low: u64 = self.low().read().into();
        let high: u64 = self.high().read().into();

        (high << 32) | low
    }
    fn write(&self, val: u64) {
        let low: u32 = val as u32;
        let high: u32 = (val >> 32) as u32;
        self.low().write(low);
        self.high().write(high);
    }
}
pub trait RegistersQueue {
    type Address<'a>: RegistersQueueAddress
    where
        Self: 'a;
    type Head<'a>: RegisterQueueHead
    where
        Self: 'a;
    type Tail<'a>: RegisterQueueTail
    where
        Self: 'a;

    // Do I need expose base address to the driver?
    // const BASE_ADDR: u64;

    fn addr(&self) -> Self::Address<'_>;
    fn head(&self) -> Self::Head<'_>;
    fn tail(&self) -> Self::Tail<'_>;
}

pub trait RegistersCommandRequest: RegistersQueue {}
pub trait RegistersCommandResponse: RegistersQueue {}
pub trait RegistersSend: RegistersQueue {}
pub trait RegistersMetaReport: RegistersQueue {}
