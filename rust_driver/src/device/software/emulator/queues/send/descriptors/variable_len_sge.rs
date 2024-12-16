use core::fmt;

use crate::device::software::emulator::net::Agent;
use crate::device::software::emulator::queues::descriptor::HandleDescriptor;
use crate::device::software::emulator::queues::send::common::{
    ScatterGatherElement, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE,
};
use crate::device::software::emulator::queues::send::queue::Builder;
use crate::device::software::emulator::{Emulator, Result};

#[repr(C, align(32))]
pub(crate) struct VariableLengthSge {
    sge2: ScatterGatherElement,
    sge1: ScatterGatherElement,
}
type Descriptor = VariableLengthSge;
const _: () = assert!(size_of::<Descriptor>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Descriptor>() == DESCRIPTOR_ALIGN);

impl<UA: Agent> HandleDescriptor<Descriptor> for Emulator<UA> {
    type Context = Builder;
    type Output = ();

    fn handle(&self, request: &Descriptor, builder: &mut Builder) -> Result<Self::Output> {
        todo!()
    }
}

impl fmt::Debug for VariableLengthSge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VariableLengthSGE")
            .field("sge1", &self.sge1)
            .field("sge2", &self.sge2)
            .finish()
    }
}

impl Descriptor {
    pub fn from_bytes(raw: [u8; DESCRIPTOR_SIZE]) -> Self {
        let descriptor = unsafe { core::mem::transmute::<_, Self>(raw) };
        assert!((&raw const descriptor).is_aligned());
        descriptor
    }
}
