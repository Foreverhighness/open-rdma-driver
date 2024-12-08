//! Mocked RPC agent

use core::cell::RefCell;

use super::agent::RpcAgent;
use super::{BarIoInfo, RpcNetIfcRxTxPayload};

pub struct MockAgent {
    frame: RefCell<u32>,
    fragment: RefCell<u32>,
}

impl MockAgent {
    pub const fn new() -> Self {
        Self {
            frame: RefCell::new(0),
            fragment: RefCell::new(0),
        }
    }
}

impl RpcAgent for MockAgent {
    unsafe fn c_createBRAM(&self, word_width: u32, memory_size: u64) -> u64 {
        todo!()
    }

    unsafe fn c_netIfcGetRxData(&self, result: *mut RpcNetIfcRxTxPayload, _client_id: u64, _is_read: u8) {
        let frame = *self.frame.borrow();
        let fragment = *self.fragment.borrow();

        let filename = &format!("fragment-{frame}-{fragment}.bin");
        *self.fragment.borrow_mut() += 1;

        let json = std::fs::read(filename).unwrap();

        let response = serde_json::from_slice::<RpcNetIfcRxTxPayload>(&json).unwrap();

        let last_fragment = response.is_last == 1;
        if last_fragment {
            *self.frame.borrow_mut() += 1;
            *self.fragment.borrow_mut() = 0;
        }

        unsafe { *result = response };
    }

    unsafe fn c_netIfcPutTxData(&self, client_id: u64, data_stream: *mut RpcNetIfcRxTxPayload) {
        todo!()
    }

    unsafe fn c_getPcieBarReadReq(&self, result: *mut BarIoInfo, client_id: u64) {
        todo!()
    }

    unsafe fn c_getPcieBarWriteReq(&self, result: *mut BarIoInfo, client_id: u64) {
        todo!()
    }

    unsafe fn c_putPcieBarReadResp(&self, client_id: u64, result: *mut BarIoInfo) {
        todo!()
    }

    unsafe fn c_putPcieBarWriteResp(&self, client_id: u64, result: *mut BarIoInfo) {
        todo!()
    }

    unsafe fn c_readBRAM(&self, result: *mut u32, client_id: u64, csr_addr: u64, word_width: u32) {
        todo!()
    }

    unsafe fn c_writeBRAM(&self, client_id: u64, csr_addr: u64, data: *mut u32, byte_en: *mut u32, word_width: u32) {
        todo!()
    }
}
