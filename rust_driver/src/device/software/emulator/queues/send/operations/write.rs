use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::queues::send::descriptors::{Seg0, Seg1, VariableLengthSge};
use crate::device::software::emulator::{Emulator, Result};

#[derive(Debug)]
pub struct Write {}

impl<UA: Agent> HandleDescriptor<Write> for Emulator<UA> {
    type Context = ();
    type Output = ();

    fn handle(&self, req: &Write, (): &mut ()) -> Result<Self::Output> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Builder {}

impl Builder {
    pub fn from_seg0(seg0: Seg0) -> Self {
        todo!()
    }

    pub fn with_seg1(self, seg1: Seg1) -> Self {
        todo!()
    }

    pub fn with_sge(self, sge: VariableLengthSge) -> Write {
        todo!()
    }
}
