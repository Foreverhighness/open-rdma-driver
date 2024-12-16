use core::fmt;

use crate::device::software::emulator::address::DmaAddress;
use crate::device::software::emulator::queues::errors::ParseDescriptorError;
use crate::device::software::emulator::queues::send::operations::Opcode;
use crate::device::software::emulator::types::MemoryRegionKey;
use crate::device::software::emulator::Result;

#[bitfield_struct::bitfield(u64, debug = false)]
pub(crate) struct Header {
    pub valid: bool,
    is_success_or_need_signal_cplt: bool,
    pub first: bool,
    pub last: bool,
    #[bits(4)]
    opcode_inner: u8,
    #[bits(4)]
    pub extra_segment_cnt: u8,
    #[bits(20)]
    __: (),
    pub total_len: u32,
}

impl Header {
    pub const fn is_success(&self) -> bool {
        self.is_success_or_need_signal_cplt()
    }

    pub const fn need_signal_cplt(&self) -> bool {
        self.is_success_or_need_signal_cplt()
    }

    pub fn opcode(&self) -> Result<Opcode> {
        let opcode = self
            .opcode_inner()
            .try_into()
            .map_err(|_| ParseDescriptorError::SendUnknownOpcode(self.opcode_inner()))?;
        Ok(opcode)
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandRequestCommonHeader")
            .field("valid", &self.valid())
            .field("is_success_or_need_signal_cplt", &self.is_success())
            .field("first", &self.first())
            .field("last", &self.last())
            .field("opcode", &self.opcode().map_err(|_| fmt::Error)?)
            .field("extra_segment_cnt", &self.extra_segment_cnt())
            .field("total_len", &self.total_len())
            .finish()
    }
}

#[bitfield_struct::bitfield(u32)]
pub struct PMtuAndSendFlagAndQpTypeAndSgeCount {
    #[bits(3)]
    pub packet_mtu_kind: u8,
    #[bits(5)]
    __: (),
    #[bits(5)]
    pub send_flag: u8,
    #[bits(3)]
    __: (),
    #[bits(4)]
    pub queue_pair_type: u8,
    #[bits(4)]
    __: (),
    #[bits(3)]
    pub sge_count: u8,
    #[bits(5)]
    __: (),
}

#[bitfield_struct::bitfield(u32)]
pub struct PacketSequenceNumber {
    #[bits(24)]
    pub packet_sequence_number: u32,
    #[bits(8)]
    __: (),
}

#[bitfield_struct::bitfield(u32)]
pub struct QueuePairNumber {
    #[bits(24)]
    pub queue_pair_number: u32,
    #[bits(8)]
    __: (),
}

#[repr(C)]
pub struct ScatterGatherElement {
    pub local_key: MemoryRegionKey,
    pub len: u32,
    pub local_addr: DmaAddress,
}

impl fmt::Debug for ScatterGatherElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScatterGatherElement")
            .field("local_addr", &self.local_addr)
            .field("len", &self.len)
            .field("local_key", &self.local_key)
            .finish()
    }
}
