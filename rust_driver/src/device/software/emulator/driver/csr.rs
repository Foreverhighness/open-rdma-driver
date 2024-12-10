//! Control Status Register for driver use

/// The layout in memory of blue rdma command request queue registers.
#[repr(C)]
pub struct BlueRdmaCommandRequestRegisters {
    pub registers: RegistersCommandRequest, // 0x8000 - 0x800F
    _padding: [u8; 4080],                   // 0x8010 - 0x8FFF
}
const _: () = assert!(size_of::<BlueRdmaCommandRequestRegisters>() == 4096);

/// The layout in memory of blue rdma command response queue registers.
#[repr(C)]
pub struct BlueRdmaCommandResponseRegisters {
    pub registers: RegistersCommandResponse, // 0x0000 - 0x000F
    _padding: [u8; 4080],                    // 0x0010 - 0x0FFF
}
const _: () = assert!(size_of::<BlueRdmaCommandResponseRegisters>() == 4096);

/// The layout in memory of blue rdma send queue registers.
#[repr(C)]
pub struct BlueRdmaSendRegisters {
    pub registers: RegistersSend, // 0x9000 - 0x900F
    _padding: [u8; 4080],         // 0x9010 - 0x9FFF
}
const _: () = assert!(size_of::<BlueRdmaSendRegisters>() == 4096);

/// The layout in memory of blue rdma meta report queue registers.
#[repr(C)]
pub struct BlueRdmaMetaReportRegisters {
    pub registers: RegistersMetaReport, // 0x1000 - 0x100F
    _padding: [u8; 4080],               // 0x1010 - 0x1FFF
}
const _: () = assert!(size_of::<BlueRdmaMetaReportRegisters>() == 4096);

/// Struct that holds registers related to one command request queue.
#[derive(Debug, Default)]
#[repr(C)]
pub struct RegistersCommandRequest {
    pub ring_buffer_base_address_low: u32,  // 0x8000
    pub ring_buffer_base_address_high: u32, // 0x8004
    pub ring_buffer_head: u32,              // 0x8008
    pub ring_buffer_tail: u32,              // 0x800C
}

/// Struct that holds registers related to one command response queue.
#[derive(Debug, Default)]
#[repr(C)]
pub struct RegistersCommandResponse {
    pub ring_buffer_base_address_low: u32,  // 0x0000
    pub ring_buffer_base_address_high: u32, // 0x0004
    pub ring_buffer_head: u32,              // 0x0008
    pub ring_buffer_tail: u32,              // 0x000C
}

/// Struct that holds registers related to one send queue.
#[derive(Debug, Default)]
#[repr(C)]
pub struct RegistersSend {
    pub ring_buffer_base_address_low: u32,  // 0x9000
    pub ring_buffer_base_address_high: u32, // 0x9004
    pub ring_buffer_head: u32,              // 0x9008
    pub ring_buffer_tail: u32,              // 0x900C
}

/// Struct that holds registers related to one meta report queue.
#[derive(Debug, Default)]
#[repr(C)]
pub struct RegistersMetaReport {
    pub ring_buffer_base_address_low: u32,  // 0x1000
    pub ring_buffer_base_address_high: u32, // 0x1004
    pub ring_buffer_head: u32,              // 0x1008
    pub ring_buffer_tail: u32,              // 0x100C
}
