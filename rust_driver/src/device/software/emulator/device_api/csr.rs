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
    type High: RegisterQueueAddressHigh;
    type Low: RegisterQueueAddressLow;

    fn high(&self) -> &Self::High;
    fn low(&self) -> &Self::Low;

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
    type Address: RegistersQueueAddress;
    type Head: RegisterQueueHead;
    type Tail: RegisterQueueTail;

    // Do I need expose base address to the driver?
    // const BASE_ADDR: u64;

    fn addr(&self) -> &Self::Address;
    fn head(&self) -> &Self::Head;
    fn tail(&self) -> &Self::Tail;
}

pub trait RegistersCommandRequest: RegistersQueue {}
pub trait RegistersCommandResponse: RegistersQueue {}
pub trait RegistersSend: RegistersQueue {}
pub trait RegistersMetaReport: RegistersQueue {}
