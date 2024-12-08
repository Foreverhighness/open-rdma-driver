//! Python Server RPC call agent

use core::mem::transmute;

#[expect(non_snake_case, reason = "C library interface")]
pub trait RpcAgent {
    unsafe fn c_createBRAM(&self, word_width: u32, memory_size: u64) -> u64;
    unsafe fn c_netIfcGetRxData(&self, result: *mut RpcNetIfcRxTxPayload, client_id: u64, is_read: u8);
    unsafe fn c_netIfcPutTxData(&self, client_id: u64, data_stream: *mut RpcNetIfcRxTxPayload);
    unsafe fn c_getPcieBarReadReq(&self, result: *mut BarIoInfo, client_id: u64);
    unsafe fn c_getPcieBarWriteReq(&self, result: *mut BarIoInfo, client_id: u64);
    unsafe fn c_putPcieBarReadResp(&self, client_id: u64, result: *mut BarIoInfo);
    unsafe fn c_putPcieBarWriteResp(&self, client_id: u64, result: *mut BarIoInfo);
    unsafe fn c_readBRAM(&self, result: *mut u32, client_id: u64, csr_addr: u64, word_width: u32);
    unsafe fn c_writeBRAM(&self, client_id: u64, csr_addr: u64, data: *mut u32, byte_en: *mut u32, word_width: u32);
}

#[repr(C)]
#[derive(Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct RpcNetIfcRxTxPayload {
    // change type u8 -> u32, so it can be serialize and remain 32 bit align
    pub data: [u32; 16],
    pub byte_en: [u8; 8],

    pub reserved: u8, // align to 32 bit

    // `is_first` field is unused, according to <https://github.com/datenlord/blue-rdma/blob/next/src/third_party/MockHost.bsv#L204>
    pub _is_first: u8,
    pub is_last: u8,
    pub is_valid: u8,
}

impl RpcNetIfcRxTxPayload {
    pub const fn new() -> Self {
        Self {
            data: [0; 16],
            byte_en: [0; 8],
            reserved: 0,
            _is_first: 0,
            is_last: 0,
            is_valid: 0,
        }
    }

    /// new request, return generated request and remained buffer
    pub fn request(buf: &[u8]) -> (Self, &[u8]) {
        assert!(!buf.is_empty());

        let len = buf.len().min(64);
        let remainder = &buf[len..];
        let is_last = if remainder.is_empty() { 1 } else { 0 };

        let mut data = [0; 16];

        // Safety: 32 * 16 == 8 * 64
        let data_u8 = unsafe { transmute::<_, &mut [u8; 64]>(&mut data) };
        assert_eq!(core::mem::size_of_val(data_u8), core::mem::size_of_val(&data));

        data_u8[..len].copy_from_slice(&buf[..len]);

        let byte_en = if len == 64 {
            [u8::MAX; 8]
        } else {
            let byte_en: u64 = (1 << len) - 1;
            byte_en.to_ne_bytes()
        };

        let request = Self {
            data,
            byte_en,
            reserved: 0,
            _is_first: 170,
            is_last,
            is_valid: 1,
        };

        (request, remainder)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct BarIoInfo {
    pub value: u64,
    pub addr: u64,
    pub valid: u64,
    pub pci_tag: u64,
}

impl BarIoInfo {
    pub const fn new() -> Self {
        Self {
            value: 0,
            addr: 0,
            valid: 0,
            pci_tag: 0,
        }
    }
}

// gcc -O3 -D_FILE_OFFSET_BITS=64 -fPIC -shared -o /usr/lib/libMockHost.so /blue-rdma/src/third_party/MockHost.c
#[link(name = "MockHost")]
extern "C" {
    pub fn c_createBRAM(word_width: u32, memory_size: u64) -> u64;
    pub fn c_netIfcGetRxData(result: *mut RpcNetIfcRxTxPayload, client_id: u64, is_read: u8);
    pub fn c_netIfcPutTxData(client_id: u64, data_stream: *mut RpcNetIfcRxTxPayload);
    pub fn c_getPcieBarReadReq(result: *mut BarIoInfo, client_id: u64);
    pub fn c_getPcieBarWriteReq(result: *mut BarIoInfo, client_id: u64);
    pub fn c_putPcieBarReadResp(client_id: u64, result: *mut BarIoInfo);
    pub fn c_putPcieBarWriteResp(client_id: u64, result: *mut BarIoInfo);
    pub fn c_readBRAM(result: *mut u32, client_id: u64, csr_addr: u64, word_width: u32);
    pub fn c_writeBRAM(client_id: u64, csr_addr: u64, data: *mut u32, byte_en: *mut u32, word_width: u32);
}

pub struct Agent;

impl RpcAgent for Agent {
    unsafe fn c_createBRAM(&self, word_width: u32, memory_size: u64) -> u64 {
        unsafe { c_createBRAM(word_width, memory_size) }
    }

    unsafe fn c_netIfcGetRxData(&self, result: *mut RpcNetIfcRxTxPayload, client_id: u64, is_read: u8) {
        unsafe { c_netIfcGetRxData(result, client_id, is_read) }
    }

    unsafe fn c_netIfcPutTxData(&self, client_id: u64, data_stream: *mut RpcNetIfcRxTxPayload) {
        unsafe { c_netIfcPutTxData(client_id, data_stream) }
    }

    unsafe fn c_getPcieBarReadReq(&self, result: *mut BarIoInfo, client_id: u64) {
        unsafe { c_getPcieBarReadReq(result, client_id) }
    }

    unsafe fn c_getPcieBarWriteReq(&self, result: *mut BarIoInfo, client_id: u64) {
        unsafe { c_getPcieBarWriteReq(result, client_id) }
    }

    unsafe fn c_putPcieBarReadResp(&self, client_id: u64, result: *mut BarIoInfo) {
        unsafe { c_putPcieBarReadResp(client_id, result) }
    }

    unsafe fn c_putPcieBarWriteResp(&self, client_id: u64, result: *mut BarIoInfo) {
        unsafe { c_putPcieBarWriteResp(client_id, result) }
    }

    unsafe fn c_readBRAM(&self, result: *mut u32, client_id: u64, csr_addr: u64, word_width: u32) {
        unsafe { c_readBRAM(result, client_id, csr_addr, word_width) }
    }

    unsafe fn c_writeBRAM(&self, client_id: u64, csr_addr: u64, data: *mut u32, byte_en: *mut u32, word_width: u32) {
        unsafe { c_writeBRAM(client_id, csr_addr, data, byte_en, word_width) }
    }
}
