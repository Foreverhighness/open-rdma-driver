use eui48::MacAddress;

use crate::device::software::emulator::queues::send::common::{
    PacketSequenceNumber, PmtuAndSendFlagAndQpTypeAndSgeCount, QueuePairNumber, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};

#[repr(C, align(32))]
pub(crate) struct Seg1 {
    pmtu_send_flag_qp_type_sge_cnt: PmtuAndSendFlagAndQpTypeAndSgeCount,
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
