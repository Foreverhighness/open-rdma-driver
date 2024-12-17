use crate::device::layout::MetaReportQueueDescFragAETH;
use crate::device::software::emulator::address::VirtualAddress;
use crate::device::software::emulator::types::{MemoryRegionKey, PacketSequenceNumber, QueuePairNumber};

#[repr(C)]
pub(crate) struct BaseTransportHeader(BTHPart0, BTHPart1);

impl BaseTransportHeader {
    pub const fn new(
        trans_type: u8,
        opcode: u8,
        qpn: QueuePairNumber,
        psn: PacketSequenceNumber,
        solicited: bool,
        is_ack_req: bool,
        pad_cnt: u8,
    ) -> Self {
        Self(
            BTHPart0::new()
                .with_trans_type(trans_type)
                .with_opcode(opcode)
                .with_queue_pair_number(qpn),
            BTHPart1::new()
                .with_packet_sequence_number(psn)
                .with_solicited(solicited)
                .with_is_ack_req(is_ack_req)
                .with_pad_count(pad_cnt),
        )
    }
}

#[bitfield_struct::bitfield(u32)]
struct BTHPart0 {
    #[bits(3)]
    pub trans_type: u8,

    #[bits(5)]
    pub opcode: u8,

    #[bits(24)]
    pub queue_pair_number: u32,
}

#[bitfield_struct::bitfield(u32)]
struct BTHPart1 {
    #[bits(24)]
    pub packet_sequence_number: u32,

    pub solicited: bool,

    pub is_ack_req: bool,

    #[bits(2)]
    pub pad_count: u8,

    #[bits(4)]
    __: (),
}

#[repr(C)]
pub(crate) struct RdmaExtendedTransportHeader {
    local_virtual_addr: [u8; 8],
    local_key: MemoryRegionKey,
    len: u32,
}

impl RdmaExtendedTransportHeader {
    pub const fn new(local_va: VirtualAddress, local_key: MemoryRegionKey, len: u32) -> Self {
        Self {
            local_virtual_addr: local_va.0.to_ne_bytes(),
            local_key,
            len,
        }
    }
}

#[repr(transparent)]
pub(super) struct AckExtendedTransportHeader(MetaReportQueueDescFragAETH<[u8; 8]>);

#[derive(Debug)]
#[repr(C)]
pub(super) struct SecondaryRdmaExtendedTransportHeader {
    addr: VirtualAddress,
    local_key: MemoryRegionKey,
}

#[derive(Debug)]
#[repr(C)]
pub(super) struct ImmediateExtendedTransportHeader {
    data: u32,
}

#[bitfield_struct::bitfield(u32)]
pub(super) struct PsnAndReqStatus {
    #[bits(24)]
    pub expected_psn: u32,
    #[bits(8)]
    pub req_status: u8,
}

#[bitfield_struct::bitfield(u32)]
pub(super) struct MessageSequenceNumberAndCanAutoAck {
    #[bits(24)]
    pub message_sequence_number: u32,
    #[bits(7)]
    __: (),

    pub can_auto_ack: bool,
}