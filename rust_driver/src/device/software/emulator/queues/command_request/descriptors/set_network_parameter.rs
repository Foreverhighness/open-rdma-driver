use core::fmt;

use crate::device::layout::CmdQueueReqDescSetNetworkParam;
use crate::device::software::emulator::queues::command_request::common::{Unknown, DESCRIPTOR_ALIGN, DESCRIPTOR_SIZE};

#[repr(C, align(32))]
pub struct SetNetworkParameter(CmdQueueReqDescSetNetworkParam<[u8; DESCRIPTOR_SIZE]>);
const _: () = assert!(size_of::<SetNetworkParameter>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<SetNetworkParameter>() == DESCRIPTOR_ALIGN);

impl fmt::Debug for SetNetworkParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl AsRef<Unknown> for SetNetworkParameter {
    fn as_ref(&self) -> &Unknown {
        unsafe { core::mem::transmute(self) }
    }
}

impl AsRef<SetNetworkParameter> for Unknown {
    fn as_ref(&self) -> &SetNetworkParameter {
        unsafe { core::mem::transmute(self) }
    }
}
