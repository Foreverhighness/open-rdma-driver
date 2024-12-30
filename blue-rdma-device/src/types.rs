// Common
pub type PathMtuKind = crate::third_party::rdma::Pmtu;
pub type ProtectDomainHandler = u32;
pub type MemoryAccessFlag = crate::third_party::rdma::MemAccessTypeFlag;
pub type PacketSequenceNumber = u32;
pub type MessageSequenceNumber = u16;

// Queue Pair
pub type MemoryRegionKey = crate::third_party::rdma::Key;

// Memory Region
pub type QueuePairType = crate::third_party::rdma::QpType;
pub type QueuePairNumber = u32;

// Send
pub type SendFlag = crate::third_party::rdma::WorkReqSendFlag;
