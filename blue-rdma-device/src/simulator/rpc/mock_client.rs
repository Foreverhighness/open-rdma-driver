//! Mocked RPC client

use core::cell::RefCell;

use super::{Client, RpcNetIfcRxTxPayload};

#[derive(Debug, Clone)]
pub struct MockRpcClient {
    frame: RefCell<u32>,
    fragment: RefCell<u32>,
}

impl MockRpcClient {
    #[allow(unused, reason = "used in test")]
    pub const fn new() -> Self {
        Self {
            frame: RefCell::new(0),
            fragment: RefCell::new(0),
        }
    }
}

impl Client for MockRpcClient {
    unsafe fn c_netIfcGetRxData(&self, result: *mut RpcNetIfcRxTxPayload, _client_id: u64, _is_read: u8) {
        let frame = *self.frame.borrow();
        let fragment = *self.fragment.borrow();

        let filename = &format!(".cache/captures/fragment-{frame}-{fragment}.bin");
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
}
