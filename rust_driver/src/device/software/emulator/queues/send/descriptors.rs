mod seg0;
mod seg1;
mod variable_len_sge;

use seg0::Seg0;
use seg1::Seg1;
use variable_len_sge::VariableLengthSGE;

use super::common::DESCRIPTOR_SIZE;
use crate::device::software::emulator::Result;

#[non_exhaustive]
// #[derive(Debug)]
pub(super) enum DescriptorRef<'d> {
    Seg0(&'d Seg0),
    Seg1(&'d Seg1),
    VariableLengthSGE(&'d VariableLengthSGE),
}

impl<'d> DescriptorRef<'d> {
    pub(super) fn parse<'r>(raw: &'r [u8; DESCRIPTOR_SIZE]) -> Result<DescriptorRef<'d>>
    where
        'r: 'd,
    {
        todo!()
    }
}
