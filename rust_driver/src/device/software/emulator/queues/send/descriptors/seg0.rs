use core::fmt;
use core::net::Ipv4Addr;

use super::common::Header;
use super::{DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};
use crate::device::software::emulator::address::DmaAddress;
use crate::device::software::emulator::types::MemoryRegionKey;

#[repr(C, align(32))]
pub(crate) struct Seg0 {
    pub header: Header,
    pub remote_addr: DmaAddress,
    pub remote_key: MemoryRegionKey,
    dest_ip: [u8; 4],
    pub partition_key: u16,
    _reserved: [bool; 6],
}
type Descriptor = Seg0;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);

impl Seg0 {
    pub fn dest_ip(&self) -> Ipv4Addr {
        Ipv4Addr::new(self.dest_ip[3], self.dest_ip[2], self.dest_ip[1], self.dest_ip[0])
    }
}

impl fmt::Debug for Seg0 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SendSeg0")
            .field("header", &self.header)
            .field("remote_addr", &self.remote_addr)
            .field("remote_key", &self.remote_key)
            .field("dest_ip", &self.dest_ip())
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
