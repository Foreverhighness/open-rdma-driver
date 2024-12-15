use core::fmt;
use core::net::Ipv4Addr;

use crate::device::software::emulator::address::DmaAddress;
use crate::device::software::emulator::queues::send::common::{Header, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};
use crate::device::software::emulator::types::MemoryRegionKey;

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

impl fmt::Debug for Descriptor {
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
