use core::fmt;

use crate::device::layout::MetaReportQueueDescFragAETH;
use crate::device::software::emulator::address::VirtualAddress;
use crate::device::software::emulator::types::{
    MemoryRegionKey, MessageSequenceNumber, PacketSequenceNumber, QueuePairNumber,
};
use crate::device::ToHostWorkRbDescAethCode;

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

impl fmt::Debug for BaseTransportHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BaseTransportHeader")
            .field(&self.0)
            .field(&self.1)
            .finish()
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

impl fmt::Debug for RdmaExtendedTransportHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RdmaExtendedTransportHeader")
            .field("local_virtual_addr", &self.local_va())
            .field("local_key", &self.local_key)
            .field("len", &self.len)
            .finish()
    }
}

impl RdmaExtendedTransportHeader {
    pub const fn new(local_va: VirtualAddress, local_key: MemoryRegionKey, len: u32) -> Self {
        Self {
            local_virtual_addr: local_va.0.to_ne_bytes(),
            local_key,
            len,
        }
    }

    pub const fn local_va(&self) -> VirtualAddress {
        VirtualAddress(u64::from_ne_bytes(self.local_virtual_addr))
    }
}

#[repr(transparent)]
pub(crate) struct AckExtendedTransportHeader(MetaReportQueueDescFragAETH<[u8; 8]>);

impl AckExtendedTransportHeader {
    pub fn new(
        psn: PacketSequenceNumber,
        msn: MessageSequenceNumber,
        value: u8,
        code: ToHostWorkRbDescAethCode,
    ) -> Self {
        let mut aeth = MetaReportQueueDescFragAETH([0u8; 8]);
        aeth.set_psn(psn);
        aeth.set_msn(msn as u32);
        aeth.set_aeth_value(value as u32);
        aeth.set_aeth_code(u8::from(code) as u32);

        Self(aeth)
    }

    pub fn packet_sequence_number(&self) -> PacketSequenceNumber {
        self.0.get_psn().into()
    }

    pub fn message_sequence_number(&self) -> MessageSequenceNumber {
        self.0.get_msn() as _
    }

    pub fn value(&self) -> u8 {
        self.0.get_aeth_value() as _
    }

    pub fn code(&self) -> ToHostWorkRbDescAethCode {
        (self.0.get_aeth_code() as u8).try_into().unwrap()
    }
}

impl fmt::Debug for AckExtendedTransportHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AckExtendedTransportHeader")
            .field("psn", &self.packet_sequence_number())
            .field("msn", &self.message_sequence_number())
            .field("value", &self.value())
            .field("code", &self.code())
            .finish()
    }
}

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
