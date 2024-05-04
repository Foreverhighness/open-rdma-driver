use std::{
    alloc::{alloc, dealloc, Layout},
    ops::{Deref, DerefMut},
    slice::from_raw_parts_mut,
};

use crate::{
    types::{Pmtu, PAGE_SIZE},
    Error,
};

/// Get the length of the first packet.
///
/// A buffer will be divided into multiple packets if any slice is crossed the boundary of pmtu
/// For example, if pmtu = 256 and va = 254, then the first packet can be at most 2 bytes.
/// If pmtu = 256 and va = 256, then the first packet can be at most 256 bytes.
#[inline]
#[allow(clippy::arithmetic_side_effects)]
pub(crate) fn get_first_packet_max_length(va: u64, pmtu: u32) -> u32 {
    // The offset is smaller than pmtu, which is smaller than 4096 currently.
    #[allow(clippy::cast_possible_truncation)]
    let offset = (va.wrapping_rem(u64::from(pmtu))) as u32;

    pmtu - offset
}

#[allow(clippy::arithmetic_side_effects)] // total_len must be greater or equal than first_pkt_len
pub(crate) fn calculate_packet_cnt(pmtu: Pmtu, raddr: u64, total_len: u32) -> u32 {
    let first_pkt_max_len = get_first_packet_max_length(raddr, u32::from(&pmtu));
    let first_pkt_len = total_len.min(first_pkt_max_len);

    1 + (total_len - first_pkt_len).div_ceil(u32::from(&pmtu))
}

#[allow(clippy::arithmetic_side_effects)]
pub(crate) fn u8_slice_to_u64(slice: &[u8]) -> u64 {
    // this operation convert a [u8;8] to a u64. So it's safe to left shift
    slice.iter().fold(0, |a, b| (a << 8_i32) + u64::from(*b))
}

pub(crate) fn allocate_aligned_memory(size: usize) -> Result<&'static mut [u8], Error> {
    let layout = Layout::from_size_align(size, PAGE_SIZE)
        .map_err(|_| Error::ResourceNoAvailable(format!("size is too large,which is {size:?}")))?;
    let ptr = unsafe { alloc(layout) };
    Ok(unsafe { from_raw_parts_mut(ptr, size) })
}

pub(crate) fn deallocate_aligned_memory(buf: &mut [u8], size: usize) -> Result<(), Error> {
    let layout = Layout::from_size_align(size, PAGE_SIZE)
        .map_err(|_| Error::ResourceNoAvailable(format!("size is too large,which is {size:?}")))?;
    unsafe {
        dealloc(buf.as_mut_ptr(), layout);
    }
    Ok(())
}

/// An aligned memory buffer.
#[derive(Debug)]
pub struct AlignedMemory<'a>(&'a mut [u8]);

impl AlignedMemory<'_> {
    /// # Errors
    /// Return an error if the size is too large.
    pub fn new(size: usize) -> Result<Self, Error> {
        Ok(AlignedMemory(allocate_aligned_memory(size)?))
    }
}

impl Drop for AlignedMemory<'_> {
    fn drop(&mut self) {
        if let Err(e) = deallocate_aligned_memory(self.0, self.0.len()){
            log::error!("Failed to deallocate aligned memory: {:?}", e);
        }
    }
}

impl Deref for AlignedMemory<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl DerefMut for AlignedMemory<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Pmtu;

    #[test]
    fn test_calculate_packet_cnt() {
        let raddr = 0;
        let total_len = 4096;
        let packet_cnt = super::calculate_packet_cnt(Pmtu::Mtu1024, raddr, total_len);
        assert_eq!(packet_cnt, 4);

        for raddr in 1..1023 {
            let packet_cnt = super::calculate_packet_cnt(Pmtu::Mtu1024, raddr, total_len);
            assert_eq!(packet_cnt, 5);
        }
    }
}
