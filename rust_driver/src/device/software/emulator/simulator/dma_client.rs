//! Direct Memory Access Client

use super::config::WORD_WIDTH;
use super::rpc;

#[derive(Debug)]
pub struct DmaClient<R: rpc::Client> {
    client_id: u64,

    rpc: R,
}

impl<R: rpc::Client> DmaClient<R> {
    pub const fn new(client_id: u64, rpc: R) -> Self {
        Self { client_id, rpc }
    }

    fn read_at_64(&self, addr: u64) -> [u8; 64] {
        let mut result = [0u8; 64];
        unsafe {
            self.rpc
                .c_readBRAM((&raw mut result).cast(), self.client_id, addr, WORD_WIDTH);
        }
        result
    }

    fn write_at_64_with_en(&self, addr: u64, data: &[u8; 64], byte_en: &[u8; 8]) {
        unsafe {
            self.rpc.c_writeBRAM(
                self.client_id,
                addr,
                data.as_ptr().cast::<u32>().cast_mut(),
                byte_en.as_ptr().cast::<u32>().cast_mut(),
                WORD_WIDTH,
            );
        }
    }
}
