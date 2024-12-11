use super::super::Result;
use crate::device::software::emulator::device_api::RawDevice;

pub(super) trait Request {
    type Response;

    fn handle<Dev: RawDevice>(&self, dev: &Dev) -> Result<Self::Response>;
}
