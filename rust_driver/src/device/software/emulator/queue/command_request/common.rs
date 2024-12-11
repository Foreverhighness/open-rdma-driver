use crate::device::layout::CmdQueueDescCommonHead;

pub const DESCRIPTOR_SIZE: usize = 32; // 256 bits
const DESCRIPTOR_ALIGN: usize = 32; // 256 bits

pub struct CommonHeader(CmdQueueDescCommonHead<u64>);

#[derive(Debug)]
struct Body([u8; 24]);

#[repr(C, align(32))]
pub struct Unknown {
    header: CommonHeader,
    body: Body,
}
const _: () = assert!(size_of::<Unknown>() == DESCRIPTOR_SIZE);

trait Common: AsRef<Unknown> {
    fn header(&self) -> &CommonHeader {
        assert_eq!(size_of_val(self), DESCRIPTOR_SIZE);
        assert_eq!(align_of_val(self), DESCRIPTOR_ALIGN);

        &self.as_ref().header
    }

    fn body(&self) -> &Body {
        assert_eq!(size_of_val(self), DESCRIPTOR_SIZE);
        assert_eq!(align_of_val(self), DESCRIPTOR_ALIGN);

        &self.as_ref().body
    }
}
