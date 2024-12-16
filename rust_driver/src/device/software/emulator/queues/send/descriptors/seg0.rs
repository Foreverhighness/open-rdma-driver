use core::fmt;
use core::net::Ipv4Addr;

use crate::device::software::emulator::address::DmaAddress;
use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::queues::send::common::{Header, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};
use crate::device::software::emulator::queues::send::queue::Builder;
use crate::device::software::emulator::types::MemoryRegionKey;
use crate::device::software::emulator::{Emulator, Result};

#[repr(C, align(32))]
pub(crate) struct Seg0 {
    header: Header,
    remote_addr: DmaAddress,
    remote_key: MemoryRegionKey,
    dest_ip: Ipv4Addr,
    partition_key: u16,
    _reserved: [bool; 6],
}
type Descriptor = Seg0;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);

impl<UA: Agent> HandleDescriptor<Descriptor> for Emulator<UA> {
    type Context = Builder;
    type Output = ();

    fn handle(&self, request: &Descriptor, builder: &mut Builder) -> Result<Self::Output> {
        todo!();
    }
}

impl fmt::Debug for Seg0 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SendSeg0")
            .field("header", &self.header)
            .field("remote_addr", &self.remote_addr)
            .field("remote_key", &self.remote_key)
            .field("dest_ip", &self.dest_ip)
            .field("partition_key", &self.partition_key)
            .finish()
    }
}

impl Descriptor {
    pub fn from_bytes(raw: [u8; DESCRIPTOR_SIZE]) -> Self {
        let descriptor = unsafe { core::mem::transmute::<_, Self>(raw) };
        assert!((&raw const descriptor).is_aligned());
        descriptor
    }
}
