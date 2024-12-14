use crate::device::layout::MetaReportQueueDescFragAETH;
use crate::device::software::emulator::address::VirtualAddress;
use crate::device::software::emulator::types::MemoryRegionKey;

pub(super) const DESCRIPTOR_SIZE: usize = 32; // 256 bits
pub(super) const DESCRIPTOR_ALIGN: usize = 32; // 256 bits

#[repr(C)]
pub(super) struct BaseTransportHeader(BTHPart0, BTHPart1);

#[bitfield_struct::bitfield(u32)]
struct BTHPart0 {
    #[bits(3)]
    trans_type_inner: u8,

    #[bits(5)]
    opcode_inner: u8,

    #[bits(24)]
    queue_pair_number_inner: u32,
}

#[bitfield_struct::bitfield(u32)]
struct BTHPart1 {
    #[bits(24)]
    packet_sequence_number_inner: u32,

    solicited: bool,

    is_ack_req: bool,

    #[bits(2)]
    pad_count_inner: u8,

    #[bits(4)]
    __: (),
}

#[repr(C)]
#[derive(Debug)]
pub(super) struct RdmaExtendedTransportHeader {
    addr_inner: [u8; 8],
    remote_key: MemoryRegionKey,
    len: u32,
}

#[repr(transparent)]
pub(super) struct AckExtendedTransportHeader(MetaReportQueueDescFragAETH<[u8; 8]>);

#[derive(Debug)]
#[repr(C)]
pub(super) struct SecondaryRdmaExtendedTransportHeader {
    addr: VirtualAddress,
    remote_key: MemoryRegionKey,
}

#[derive(Debug)]
#[repr(C)]
pub(super) struct ImmediateExtendedTransportHeader {
    data: u32,
}

#[bitfield_struct::bitfield(u32)]
pub(super) struct PsnAndReqStatus {
    #[bits(24)]
    expected_psn_inner: u32,
    #[bits(8)]
    req_status: u8,
}

#[bitfield_struct::bitfield(u32)]
pub(super) struct MessageSequenceNumberAndCanAutoAck {
    #[bits(24)]
    message_sequence_number_inner: u32,
    #[bits(7)]
    __: (),
    can_auto_ack: bool,
}

#[bitfield_struct::bitfield(u32, repr = u32)]
struct Test {
    inner: u32,
}
