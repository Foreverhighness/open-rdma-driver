macro_rules! declare_and_impl_basic_register {
    ($prefix:ident, $csr_type:ident, $part:ident) => {
        paste::paste! {
            #[derive(Debug, Default)]
            pub(crate) struct [<$prefix $csr_type $part>] {
                val: core::sync::atomic::AtomicU32,
            }
            impl [<$prefix $csr_type $part>] {
                pub fn read(&self) -> u32 {
                    self.val.load(core::sync::atomic::Ordering::Relaxed)
                }
                pub fn write(&self, val: u32) {
                    self.val.store(val, core::sync::atomic::Ordering::Relaxed)
                }
            }

            #[derive(Debug)]
            pub struct [<$prefix $csr_type $part Handler>]<'h, UA: Agent, DC: Client> {
                reg: &'h [<$prefix $csr_type $part>],
                dev: &'h DeviceInner<UA, DC>,
            }
            impl<'h, UA: Agent, DC: Client> [<$prefix $csr_type $part Handler>]<'h, UA, DC> {
                pub(crate) fn new<'r, 'd>(reg: &'r [<$prefix $csr_type $part>], dev: &'d DeviceInner<UA, DC>) -> Self
                where
                    'r: 'h,
                    'd: 'h,
                {
                    Self { reg, dev }
                }
            }
            impl<UA: Agent, DC: Client> csr::[<RegisterQueue $part>] for [<$prefix $csr_type $part Handler>]<'_, UA, DC> {}
        }
    };
}

macro_rules! declare_and_impl_address {
    ($prefix:ident, $csr_type:ident) => {
        paste::paste! {
            #[derive(Debug, Default)]
            pub(crate) struct [<$prefix $csr_type Address>] {
                pub(crate) low: [<$prefix $csr_type AddressLow>],
                pub(crate) high: [<$prefix $csr_type AddressHigh>]
            }
            impl [<$prefix $csr_type Address>] {
                pub(crate) fn read(&self) -> u64 {
                    let low: u64 = self.low.read().into();
                    let high: u64 = self.high.read().into();

                    (high << 32) | low
                }
                pub(super) fn write(&self, val: u64) {
                    let low: u32 = val as u32;
                    let high: u32 = (val >> 32) as u32;
                    self.low.write(low);
                    self.high.write(high);
                }
            }

            #[derive(Debug)]
            pub struct [<$prefix $csr_type AddressHandler>]<'h, UA: Agent, DC: Client> {
                addr: &'h [<$prefix $csr_type Address>],
                dev: &'h DeviceInner<UA, DC>,
            }
            impl<'h, UA: Agent, DC: Client> [<$prefix $csr_type AddressHandler>]<'h, UA, DC> {
                pub(crate) fn new<'r, 'd>(addr: &'r [<$prefix $csr_type Address>], dev: &'d DeviceInner<UA, DC>) -> Self
                where
                    'r: 'h,
                    'd: 'h,
                {
                    Self { addr, dev }
                }
            }
            impl<UA: Agent, DC: Client> csr::RegistersQueueAddress for [<$prefix $csr_type AddressHandler>]<'_, UA, DC> {
                fn high(&self) -> impl csr::RegisterQueueAddressHigh {
                    [<$prefix $csr_type AddressHighHandler>]::new(&self.addr.high, self.dev)
                }
                fn low(&self) -> impl csr::RegisterQueueAddressLow {
                    [<$prefix $csr_type AddressLowHandler>]::new(&self.addr.low, self.dev)
                }
            }
        }
    };
}

macro_rules! declare_and_impl_queue {
    ($prefix:ident, $csr_type:ident) => {
        paste::paste! {
            #[derive(Debug, Default)]
            pub(crate) struct [<$prefix $csr_type>] {
                pub(crate) addr: [<$prefix $csr_type Address>],
                pub(crate) head: [<$prefix $csr_type Head>],
                pub(crate) tail: [<$prefix $csr_type Tail>],
            }

            #[derive(Debug)]
            pub struct [<$prefix $csr_type Handler>]<'h, UA: Agent, DC: Client> {
                regs: &'h [<$prefix $csr_type>],
                dev: &'h DeviceInner<UA, DC>,
            }
            impl<'h, UA: Agent, DC: Client> [<$prefix $csr_type Handler>]<'h, UA, DC> {
                pub(crate) fn new<'r, 'd>(regs: &'r [<$prefix $csr_type>], dev: &'d DeviceInner<UA, DC>) -> Self
                where
                    'r: 'h,
                    'd: 'h,
                {
                    Self { regs, dev }
                }
            }
            impl<UA: Agent, DC: Client> csr::$csr_type for [<$prefix $csr_type Handler>]<'_, UA, DC> {}
            impl<UA: Agent, DC: Client> csr::RegistersQueue for [<$prefix $csr_type Handler>]<'_, UA, DC> {
                fn addr(&self) -> impl csr::RegistersQueueAddress {
                    [<$prefix $csr_type AddressHandler>]::new(&self.regs.addr, self.dev)
                }

                fn head(&self) -> impl csr::RegisterQueueHead {
                    [<$prefix $csr_type HeadHandler>]::new(&self.regs.head, self.dev)
                }

                fn tail(&self) -> impl csr::RegisterQueueTail {
                    [<$prefix $csr_type TailHandler>]::new(&self.regs.tail, self.dev)
                }
            }
        }
    };
}

macro_rules! register_queue_csr {
    ($base_addr:literal, $prefix:ident, $csr_type:ident, $csr_type_upper:ident) => {
        use $crate::device::software::emulator::device_api::csr;
        use $crate::device::software::emulator::dma::Client;
        use $crate::device::software::emulator::net::Agent;
        use $crate::device::software::emulator::DeviceInner;

        paste::paste! {
            pub const [<REGISTERS_ $csr_type_upper _BASE_ADDR>]: u64 = $base_addr;
            const _: () = assert!($base_addr % 4096 == 0); // Base address should align to 4K

            declare_and_impl_basic_register!($prefix, [<Registers $csr_type>], AddressHigh);
            declare_and_impl_basic_register!($prefix, [<Registers $csr_type>], AddressLow);
            declare_and_impl_basic_register!($prefix, [<Registers $csr_type>], Head);
            declare_and_impl_basic_register!($prefix, [<Registers $csr_type>], Tail);

            declare_and_impl_address!($prefix, [<Registers $csr_type>]);

            declare_and_impl_queue!($prefix, [<Registers $csr_type>]);
        }
    };
}
