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
#[derive(Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct RpcNetIfcRxTxPayload {
    // change type u8 -> u32, so it can be serialize and remain 32 bit align
    pub data: [u32; 16],
    pub byte_en: [u32; 2],

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
            byte_en: [0; 2],
            reserved: 0,
            _is_first: 0,
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
