use crate::device::software::emulator::address::DmaAddress;
use crate::device::software::emulator::types::MemoryRegionKey;

pub(super) const DESCRIPTOR_SIZE: usize = 32; // 256 bits
pub(super) const DESCRIPTOR_ALIGN: usize = 32; // 256 bits

#[bitfield_struct::bitfield(u64)]
pub(super) struct Header {
    valid: bool,
    is_success_or_need_signal_cplt: bool,
    first: bool,
    last: bool,
    #[bits(4)]
    opcode_inner: u8,
    #[bits(4)]
    extra_segment_cnt: u8,
    #[bits(20)]
    __: (),
    total_len: u32,
}

#[bitfield_struct::bitfield(u32)]
pub(super) struct PmtuAndSendFlagAndQpTypeAndSgeCount {
    #[bits(3)]
    packet_mtu_kind: u8,
    #[bits(5)]
    __: (),
    #[bits(5)]
    send_flag: u8,
    #[bits(3)]
    __: (),
    #[bits(4)]
    queue_pair_type: u8,
    #[bits(4)]
    __: (),
    #[bits(3)]
    sge_count: u8,
    #[bits(5)]
    __: (),
}

#[bitfield_struct::bitfield(u32)]
pub(super) struct PacketSequenceNumber {
    #[bits(24)]
    packet_sequence_number: u32,
    #[bits(8)]
    __: (),
}

#[bitfield_struct::bitfield(u32)]
pub(super) struct QueuePairNumber {
    #[bits(24)]
    queue_pair_number: u32,
    #[bits(8)]
    __: (),
}

#[repr(C)]
pub(super) struct ScatterGatherElement {
    local_key: MemoryRegionKey,
    len: u32,
    local_addr: DmaAddress,
}
