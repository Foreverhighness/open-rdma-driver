use core::fmt;

use crate::device::layout::CmdQueueDescCommonHead;
use crate::device::software::emulator::queues::errors::ParseDescriptorError;
use crate::device::software::emulator::Result;

pub(super) const DESCRIPTOR_SIZE: usize = 32; // 256 bits
pub(super) const DESCRIPTOR_ALIGN: usize = 32; // 256 bits

pub(super) type Opcode = crate::device::types::CtrlRbDescOpcode;

#[repr(transparent)]
pub struct CommonHeader(CmdQueueDescCommonHead<[u8; 8]>);

impl CommonHeader {
    pub fn new(opcode: Opcode, success: bool, user_data: u32) -> Self {
        let mut header = CmdQueueDescCommonHead([0; 8]);
        header.set_valid(true);
        header.set_is_success_or_need_signal_cplt(success);
        header.set_op_code(u8::from(opcode).into());
        header.set_user_data(user_data);

        Self(header)
    }

    pub fn valid(&self) -> bool {
        self.0.get_valid() as _
    }

    pub fn is_success(&self) -> bool {
        self.0.get_is_success_or_need_signal_cplt() as _
    }

    pub fn need_signal_cplt(&self) -> bool {
        self.0.get_is_success_or_need_signal_cplt() as _
    }

    pub fn opcode(&self) -> Result<Opcode> {
        let opcode: u8 = self.0.get_op_code().try_into().unwrap();
        let opcode = opcode
            .try_into()
            .map_err(|_| ParseDescriptorError::CommandRequestUnknownOpcode(opcode))?;

        Ok(opcode)
    }

    pub fn user_data(&self) -> u32 {
        self.0.get_user_data()
    }
}

impl fmt::Debug for CommonHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandRequestCommonHeader")
            .field("valid", &self.valid())
            .field("is_success_or_need_signal_cplt", &self.is_success())
            .field("opcode", &self.opcode().map_err(|_| fmt::Error)?)
            .field("extra_segment_cnt", &self.0.get_extra_segment_cnt())
            .field("user_data", &self.0.get_user_data())
            .finish()
    }
}

#[repr(transparent)]
struct Body([u8; 24]);

#[repr(C, align(32))]
pub struct Unknown {
    header: CommonHeader,
    body: Body,
}
const _: () = assert!(size_of::<Unknown>() == DESCRIPTOR_SIZE);
const _: () = assert!(align_of::<Unknown>() == DESCRIPTOR_ALIGN);

impl fmt::Debug for Unknown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandRequestUnknown")
            .field("header", &self.header)
            .field("body", &self.body.0)
            .finish()
    }
}

impl AsRef<Unknown> for Unknown {
    fn as_ref(&self) -> &Unknown {
        self
    }
}

pub trait Header {
    fn header(&self) -> &CommonHeader;
}

impl<T: AsRef<Unknown>> Header for T {
    fn header(&self) -> &CommonHeader {
        const { assert!(size_of::<T>() == DESCRIPTOR_SIZE) };

        &self.as_ref().header
    }
}
