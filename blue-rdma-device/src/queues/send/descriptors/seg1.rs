use core::fmt;

use eui48::MacAddress;

use super::{DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE, common};
use crate::Result;
use crate::queues::errors::ParseDescriptorError;
use crate::types::{PacketSequenceNumber, PathMtuKind, QueuePairNumber, QueuePairType, SendFlag};

#[repr(C, align(32))]
pub struct Seg1 {
    pmtu_send_flag_qp_type_sge_cnt: common::PMtuAndSendFlagAndQpTypeAndSgeCount,
    psn_inner: common::PacketSequenceNumber,
    mac: [u8; 6],
    _reserved0: core::mem::MaybeUninit<[u8; 2]>,
    dest_qpn_inner: common::QueuePairNumber,
    pub immediate_data: u32,
    _reserved1: core::mem::MaybeUninit<[u8; 8]>,
}
type Descriptor = Seg1;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);

impl Seg1 {
    pub fn path_mtu_kind(&self) -> Result<PathMtuKind> {
        let path_mtu_kind = self.pmtu_send_flag_qp_type_sge_cnt.path_mtu_kind();
        let path_mtu_kind = path_mtu_kind
            .try_into()
            .map_err(|_| ParseDescriptorError::InvalidPathMTUKind(path_mtu_kind))?;

        Ok(path_mtu_kind)
    }

    pub const fn send_flag(&self) -> SendFlag {
        SendFlag::from_bits(self.pmtu_send_flag_qp_type_sge_cnt.send_flag()).unwrap()
    }

    pub const fn mac(&self) -> MacAddress {
        MacAddress::new([
            self.mac[5],
            self.mac[4],
            self.mac[3],
            self.mac[2],
            self.mac[1],
            self.mac[0],
        ])
    }

    pub fn queue_pair_type(&self) -> Result<QueuePairType> {
        let queue_pair_type = self.pmtu_send_flag_qp_type_sge_cnt.queue_pair_type();
        let queue_pair_type = queue_pair_type
            .try_into()
            .map_err(|_| ParseDescriptorError::InvalidQueuePairType(queue_pair_type))?;

        Ok(queue_pair_type)
    }

    pub const fn sge_count(&self) -> u8 {
        self.pmtu_send_flag_qp_type_sge_cnt.sge_count()
    }

    #[expect(clippy::useless_conversion, reason = "PacketSequenceNumber should change later")]
    pub fn packet_sequence_number(&self) -> PacketSequenceNumber {
        self.psn_inner.packet_sequence_number().into()
    }

    #[expect(clippy::useless_conversion, reason = "QueuePairNumber should change later")]
    pub fn dest_queue_pair_number(&self) -> QueuePairNumber {
        self.dest_qpn_inner.queue_pair_number().into()
    }
}

impl fmt::Debug for Seg1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SendSeg1")
            .field("path_mtu_kind", &self.path_mtu_kind().map_err(|_| fmt::Error))
            .field("send_flag", &self.send_flag())
            .field("queue_pair_type", &self.queue_pair_type().map_err(|_| fmt::Error))
            .field("sge_count", &self.sge_count())
            .field("packet_sequence_number", &self.psn_inner)
            .field("mac", &self.mac)
            .field("queue_pair_number", &self.dest_qpn_inner)
            .field("immediate_data", &self.immediate_data)
            .finish_non_exhaustive()
    }
}

impl Descriptor {
    // TODO(fh): validate here
    pub fn from_bytes(raw: [u8; DESCRIPTOR_SIZE]) -> Self {
        let descriptor = unsafe { core::mem::transmute::<[u8; 32], Self>(raw) };
        assert!((&raw const descriptor).is_aligned());
        descriptor
    }
}
