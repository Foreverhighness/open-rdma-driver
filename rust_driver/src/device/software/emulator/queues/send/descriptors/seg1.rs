use core::fmt;

use eui48::MacAddress;

use crate::device::software::emulator::queues::errors::ParseDescriptorError;
use crate::device::software::emulator::queues::send::common::{
    PMtuAndSendFlagAndQpTypeAndSgeCount, PacketSequenceNumber, QueuePairNumber, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};
use crate::device::software::emulator::types::{PacketMtuKind, QueuePairType, SendFlag};
use crate::device::software::emulator::Result;

#[repr(C, align(32))]
pub(crate) struct Seg1 {
    pmtu_send_flag_qp_type_sge_cnt: PMtuAndSendFlagAndQpTypeAndSgeCount,
    packet_sequence_number: PacketSequenceNumber,
    mac: MacAddress,
    _reserved0: [bool; 2],
    queue_pair_number: QueuePairNumber,
    immediate: u32,
    _reserved1: [bool; 8],
}
type Descriptor = Seg1;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);

impl Seg1 {
    fn packet_mtu_kind(&self) -> Result<PacketMtuKind> {
        let packet_mtu_kind = self.pmtu_send_flag_qp_type_sge_cnt.packet_mtu_kind();
        let packet_mtu_kind = packet_mtu_kind
            .try_into()
            .map_err(|_| ParseDescriptorError::InvalidPacketMTUKind(packet_mtu_kind))?;

        Ok(packet_mtu_kind)
    }

    const fn send_flag(&self) -> SendFlag {
        SendFlag::from_bits(self.pmtu_send_flag_qp_type_sge_cnt.send_flag()).unwrap()
    }

    fn queue_pair_type(&self) -> Result<QueuePairType> {
        let queue_pair_type = self.pmtu_send_flag_qp_type_sge_cnt.queue_pair_type();
        let queue_pair_type = queue_pair_type
            .try_into()
            .map_err(|_| ParseDescriptorError::InvalidQueuePairType(queue_pair_type))?;

        Ok(queue_pair_type)
    }

    fn sge_count(&self) -> u8 {
        self.pmtu_send_flag_qp_type_sge_cnt.sge_count()
    }
}

impl fmt::Debug for Seg1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SendSeg1")
            .field("packet_mtu_kind", &self.packet_mtu_kind().map_err(|_| fmt::Error))
            .field("send_flag", &self.send_flag())
            .field("queue_pair_type", &self.queue_pair_type().map_err(|_| fmt::Error))
            .field("sge_count", &self.sge_count())
            .field("packet_sequence_number", &self.packet_sequence_number)
            .field("mac", &self.mac)
            .field("queue_pair_number", &self.queue_pair_number)
            .field("immediate", &self.immediate)
            .finish()
    }
}
