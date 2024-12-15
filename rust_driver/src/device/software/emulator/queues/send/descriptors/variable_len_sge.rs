use core::fmt;

use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::queues::send::common::{
    ScatterGatherElement, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};
use crate::device::software::emulator::queues::send::queue::State;
use crate::device::software::emulator::{Emulator, Result};

#[repr(C, align(32))]
pub(crate) struct VariableLengthSGE {
    sge2: ScatterGatherElement,
    sge1: ScatterGatherElement,
}
type Descriptor = VariableLengthSGE;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);

impl<UA: Agent> HandleDescriptor<Descriptor> for Emulator<UA> {
    type Context = State;
    type Output = State;

    fn handle(&self, request: &Descriptor, cx: Self::Context) -> Result<Self::Output> {
        todo!()
    }
}

impl fmt::Debug for VariableLengthSGE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VariableLengthSGE")
            .field("sge1", &self.sge1)
            .field("sge2", &self.sge2)
            .finish()
    }
}
