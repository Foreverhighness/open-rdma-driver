macro_rules! declare_and_impl_basic_register {
    ($prefix:ident, $csr_type:ident, $part:ident) => {
        paste::paste! {
            #[derive(Debug, Default)]
            pub(super) struct [<$prefix $csr_type $part>](core::sync::atomic::AtomicU32);
            impl [<$prefix $csr_type $part>] {
                fn read(&self) -> u32 {
                    self.0.load(core::sync::atomic::Ordering::Relaxed)
                }
                fn write(&self, val: u32) {
                    self.0.store(val, core::sync::atomic::Ordering::Relaxed)
                }
            }

            #[derive(Debug)]
            pub struct [<$prefix $csr_type $part Handler>]<'h, UA: Agent> {
                reg: &'h [<$prefix $csr_type $part>],
                dev: &'h Emulator<UA>,
            }
            impl<'h, UA: Agent> [<$prefix $csr_type $part Handler>]<'h, UA> {
                pub(super) fn new<'r, 'd>(reg: &'r [<$prefix $csr_type $part>], dev: &'d Emulator<UA>) -> Self
                where
                    'r: 'h,
                    'd: 'h,
                {
                    Self { reg, dev }
                }
            }
            use $crate::device::software::emulator::device_api::csr::[<RegisterQueue $part>];
            impl<UA: Agent> [<RegisterQueue $part>] for [<$prefix $csr_type $part Handler>]<'_, UA> {}
        }
    };
}

macro_rules! declare_and_impl_address {
    ($prefix:ident, $csr_type:ident) => {
        paste::paste! {
            #[derive(Debug, Default)]
            pub(super) struct [<$prefix $csr_type Address>] {
                low: [<$prefix $csr_type AddressLow>],
                high: [<$prefix $csr_type AddressHigh>]
            }
            impl [<$prefix $csr_type Address>] {
                fn read(&self) -> u64 {
                    let low: u64 = self.low.read().into();
                    let high: u64 = self.high.read().into();

                    (high << 32) | low
                }
                fn write(&self, val: u64) {
                    let low: u32 = val as u32;
                    let high: u32 = (val >> 32) as u32;
                    self.low.write(low);
                    self.high.write(high);
                }
            }

            #[derive(Debug)]
            pub struct [<$prefix $csr_type AddressHandler>]<'h, UA: Agent> {
                addr: &'h [<$prefix $csr_type Address>],
                dev: &'h Emulator<UA>,
            }
            impl<'h, UA: Agent> [<$prefix $csr_type AddressHandler>]<'h, UA> {
                pub(super) fn new<'r, 'd>(addr: &'r [<$prefix $csr_type Address>], dev: &'d Emulator<UA>) -> Self
                where
                    'r: 'h,
                    'd: 'h,
                {
                    Self { addr, dev }
                }
            }
            use $crate::device::software::emulator::device_api::csr::RegistersQueueAddress;
            impl<UA: Agent> RegistersQueueAddress for [<$prefix $csr_type AddressHandler>]<'_, UA> {
                type High<'a>
                    = [<$prefix $csr_type AddressHighHandler>]<'a, UA>
                where
                    Self: 'a;
                type Low<'a>
                    = [<$prefix $csr_type AddressLowHandler>]<'a, UA>
                where
                    Self: 'a;

                fn high(&self) -> Self::High<'_> {
                    [<$prefix $csr_type AddressHighHandler>]::new(&self.addr.high, self.dev)
                }
                fn low(&self) -> Self::Low<'_> {
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
            pub(super) struct [<$prefix $csr_type>] {
                addr: [<$prefix $csr_type Address>],
                head: [<$prefix $csr_type Head>],
                tail: [<$prefix $csr_type Tail>],
            }

            #[derive(Debug)]
            pub struct [<$prefix $csr_type Handler>]<'h, UA: Agent> {
                regs: &'h [<$prefix $csr_type>],
                dev: &'h Emulator<UA>,
            }
            impl<'h, UA: Agent> [<$prefix $csr_type Handler>]<'h, UA> {
                pub(super) fn new<'r, 'd>(regs: &'r [<$prefix $csr_type>], dev: &'d Emulator<UA>) -> Self
                where
                    'r: 'h,
                    'd: 'h,
                {
                    Self { regs, dev }
                }
            }
            use $crate::device::software::emulator::device_api::csr::$csr_type;
            use $crate::device::software::emulator::device_api::csr::RegistersQueue;
            impl<UA: Agent> $csr_type for [<$prefix $csr_type Handler>]<'_, UA> {}
            impl<UA: Agent> RegistersQueue for [<$prefix $csr_type Handler>]<'_, UA> {
                type Address<'a>
                    = [<$prefix $csr_type AddressHandler>]<'a, UA>
                where
                    Self: 'a;
                type Head<'a>
                    = [<$prefix $csr_type HeadHandler>]<'a, UA>
                where
                    Self: 'a;
                type Tail<'a>
                    = [<$prefix $csr_type TailHandler>]<'a, UA>
                where
                    Self: 'a;

                fn addr(&self) -> Self::Address<'_> {
                    [<$prefix $csr_type AddressHandler>]::new(&self.regs.addr, self.dev)
                }

                fn head(&self) -> Self::Head<'_> {
                    [<$prefix $csr_type HeadHandler>]::new(&self.regs.head, self.dev)
                }

                fn tail(&self) -> Self::Tail<'_> {
                    [<$prefix $csr_type TailHandler>]::new(&self.regs.tail, self.dev)
                }
            }
        }
    };
}

macro_rules! register_queue_csr {
    ($base_addr:literal, $prefix:ident, $csr_type:ident, $csr_type_upper:ident) => {
        use $crate::device::software::emulator::net::Agent;
        use $crate::device::software::emulator::Emulator;

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
