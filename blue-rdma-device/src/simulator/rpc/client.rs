//! Python Server RPC call Client

use serde::de::SeqAccess;
use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[expect(non_snake_case, reason = "C library interface")]
pub trait Client: Clone + Send + 'static {
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

    unsafe fn c_readBRAM(&self, result: *mut u32, client_id: u64, addr: u64, word_width: u32) {
        unsafe { c_readBRAM(result, client_id, addr, word_width) }
    }

    unsafe fn c_writeBRAM(&self, client_id: u64, addr: u64, data: *mut u32, byte_en: *mut u32, word_width: u32) {
        unsafe { c_writeBRAM(client_id, addr, data, byte_en, word_width) }
    }
}

#[repr(C)]
#[derive(Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct RpcNetIfcRxTxPayload {
    pub data: Payload,
    pub byte_en: [u8; 8],

    pub reserved: u8, // align to 32 bit

    // `is_first` field is unused, according to <https://github.com/datenlord/blue-rdma/blob/next/src/third_party/MockHost.bsv#L204>
    pub _is_first: u8,
    pub is_last: u8,
    pub is_valid: u8,
}

#[expect(missing_copy_implementations, reason = "This type should not be clone or copy")]
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct Payload(pub [u8; 64]);

impl Payload {
    const fn new() -> Self {
        Self([0u8; 64])
    }
}

impl Serialize for Payload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(self.0.len())?;
        for elem in &self.0 {
            seq.serialize_element(elem)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Payload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Payload;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str(concat!("an array of length ", 64))
            }

            #[inline]
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let array = core::array::from_fn(|_| seq.next_element().unwrap().unwrap());
                Ok(Payload(array))
            }
        }
        deserializer.deserialize_tuple(64, Visitor)
    }
}

impl Default for RpcNetIfcRxTxPayload {
    fn default() -> Self {
        Self::new()
    }
}

impl RpcNetIfcRxTxPayload {
    pub const fn new() -> Self {
        Self {
            data: Payload::new(),
            byte_en: [0; 8],
            reserved: 0,
            _is_first: 0,
            is_last: 0,
            is_valid: 0,
        }
    }

    /// new request, return generated request and used bytes.
    pub fn new_request(buf: &[u8]) -> (Self, usize) {
        assert!(!buf.is_empty());

        let len = buf.len().min(64);
        let remainder = &buf[len..];
        let is_last = if remainder.is_empty() { 1 } else { 0 };

        let mut data = [0u8; 64];

        data[..len].copy_from_slice(&buf[..len]);

        let byte_en = if len == 64 {
            [u8::MAX; 8]
        } else {
            let byte_en: u64 = (1 << len) - 1;
            byte_en.to_ne_bytes()
        };

        let request = Self {
            data: Payload(data),
            byte_en,
            reserved: 0,
            // Hard code value, not used
            _is_first: 170,
            is_last,
            is_valid: 1,
        };

        (request, len)
    }
}

#[expect(missing_copy_implementations, reason = "This type should not be clone or copy")]
#[repr(C)]
#[derive(Debug)]
pub struct BarIoInfo {
    pub value: u64,
    pub addr: u64,
    pub valid: u64,
    pub pci_tag: u64,
}

impl Default for BarIoInfo {
    fn default() -> Self {
        Self::new()
    }
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

    pub const fn new_read_response(pci_tag: u64, value: u64) -> Self {
        Self {
            value,
            addr: 0,
            valid: 1,
            pci_tag,
        }
    }

    pub const fn new_write_response(pci_tag: u64, valid: bool) -> Self {
        Self {
            value: 0,
            addr: 0,
            valid: if valid { 1 } else { 0 },
            pci_tag,
        }
    }
}

// gcc -O3 -D_FILE_OFFSET_BITS=64 -fPIC -shared -o /usr/lib/libMockHost.so /blue-rdma/src/third_party/MockHost.c
#[link(name = "MockHost")]
unsafe extern "C" {
    pub fn c_createBRAM(word_width: u32, memory_size: u64) -> u64;
    pub fn c_netIfcGetRxData(result: *mut RpcNetIfcRxTxPayload, client_id: u64, is_read: u8);
    pub fn c_netIfcPutTxData(client_id: u64, data_stream: *mut RpcNetIfcRxTxPayload);
    pub fn c_getPcieBarReadReq(result: *mut BarIoInfo, client_id: u64);
    pub fn c_getPcieBarWriteReq(result: *mut BarIoInfo, client_id: u64);
    pub fn c_putPcieBarReadResp(client_id: u64, result: *mut BarIoInfo);
    pub fn c_putPcieBarWriteResp(client_id: u64, result: *mut BarIoInfo);
    pub fn c_readBRAM(result: *mut u32, client_id: u64, addr: u64, word_width: u32);
    pub fn c_writeBRAM(client_id: u64, addr: u64, data: *mut u32, byte_en: *mut u32, word_width: u32);
}

#[derive(Debug, Clone)]
pub struct RpcClient;

impl Client for RpcClient {}
