mod queue_pair_management;
mod set_network_parameter;
mod set_raw_packet_receive_meta;
mod update_error_psn_recover_point;
mod update_mr_table;
mod update_page_table;

pub use queue_pair_management::QueuePairManagement;
pub use set_network_parameter::SetNetworkParameter;
pub use set_raw_packet_receive_meta::SetRawPacketReceiveMeta;
pub use update_error_psn_recover_point::UpdateErrorPacketSequenceNumberRecoverPoint;
pub use update_mr_table::UpdateMemoryRegionTable;
pub use update_page_table::UpdatePageTable;

use super::common::Unknown;
use super::opcode::Opcode;
use crate::device::software::emulator::queue::command_request::common::Header;
use crate::device::software::emulator::Result;

#[non_exhaustive]
#[derive(Debug)]
pub enum DescriptorRef<'d> {
    UpdateMemoryRegionTable(&'d UpdateMemoryRegionTable),
    UpdatePageTable(&'d UpdatePageTable),
    QueuePairManagement(&'d QueuePairManagement),
    SetNetworkParameter(&'d SetNetworkParameter),
    SetRawPacketReceiveMeta(&'d SetRawPacketReceiveMeta),
    UpdateErrorPacketSequenceNumberRecoverPoint(&'d UpdateErrorPacketSequenceNumberRecoverPoint),
    // Unknown(&'d Unknown),
}

impl<'d> DescriptorRef<'d> {
    fn parse<'r>(raw: &'r Unknown) -> Result<DescriptorRef<'d>>
    where
        'r: 'd,
    {
        let opcode = raw.header().opcode()?;
        let descriptor = match opcode {
            Opcode::UpdateMrTable => Self::UpdateMemoryRegionTable(raw.as_ref()),
            Opcode::UpdatePageTable => Self::UpdatePageTable(raw.as_ref()),
            Opcode::QpManagement => Self::QueuePairManagement(raw.as_ref()),
            Opcode::SetNetworkParam => Self::SetNetworkParameter(raw.as_ref()),
            Opcode::SetRawPacketReceiveMeta => Self::SetRawPacketReceiveMeta(raw.as_ref()),
            Opcode::UpdateErrorPsnRecoverPoint => Self::UpdateErrorPacketSequenceNumberRecoverPoint(raw.as_ref()),
        };
        Ok(descriptor)
    }
}
