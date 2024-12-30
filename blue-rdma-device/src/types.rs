// Common
pub(crate) type PathMtuKind = crate::third_party::rdma::Pmtu;
pub(crate) type ProtectDomainHandler = u32;
pub(crate) type MemoryAccessFlag = crate::third_party::rdma::MemAccessTypeFlag;
pub(crate) type PacketSequenceNumber = u32;
pub(crate) type MessageSequenceNumber = u16;

// Queue Pair
pub(crate) type MemoryRegionKey = crate::third_party::rdma::Key;

// Memory Region
pub(crate) type QueuePairType = crate::third_party::rdma::QpType;
pub(crate) type QueuePairNumber = u32;

// Send
pub(crate) type SendFlag = crate::third_party::rdma::WorkReqSendFlag;
