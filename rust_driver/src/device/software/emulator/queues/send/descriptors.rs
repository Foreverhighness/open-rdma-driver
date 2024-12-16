mod seg0;
mod seg1;
mod variable_len_sge;

pub(super) use seg0::Seg0;
pub(super) use seg1::Seg1;
pub(super) use variable_len_sge::VariableLengthSge;

use super::common::DESCRIPTOR_SIZE;
use crate::device::software::emulator::Result;

#[non_exhaustive]
#[derive(Debug)]
pub(super) enum DescriptorRef<'d> {
    Seg0(&'d Seg0),
    Seg1(&'d Seg1),
    VariableLengthSGE(&'d VariableLengthSge),
}

impl<'d> DescriptorRef<'d> {
    pub(super) fn parse<'r>(raw: &'r [u8; DESCRIPTOR_SIZE]) -> Result<DescriptorRef<'d>>
    where
        'r: 'd,
    {
        todo!()
    }
}
