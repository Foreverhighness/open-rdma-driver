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

#[repr(C)]
#[derive(Debug)]
pub struct RpcNetIfcRxTxPayload {
    pub data: [u8; 64],
    pub byte_en: [u8; 8],

    pub reserved: u8, // align to 32 bit
    pub is_fisrt: u8,
    pub is_last: u8,
    pub is_valid: u8,
}

impl RpcNetIfcRxTxPayload {
    pub const fn new() -> Self {
        Self {
            data: [0; 64],
            byte_en: [0; 8],
            reserved: 0,
            is_fisrt: 0,
            is_last: 0,
            is_valid: 0,
        }
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
