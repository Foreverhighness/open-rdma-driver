use bitflags::bitflags;
use num_enum::TryFromPrimitive;

/// page size is 2MB.
pub const PAGE_SIZE: usize = 1024 * 1024 * 2;
pub(crate) const PSN_MAX_WINDOW_SIZE: u32 = 1 << 23_i32;

/// `RKey` and `LKey`
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct Key(u32);
impl Key {
    /// Create a new `Key` with the given value.
    #[must_use]
    pub fn new(key: u32) -> Self {
        Self(key)
    }

    /// Get the value of `Key`.
    #[must_use]
    pub fn get(&self) -> u32 {
        self.0
    }

    /// Convert the value of `Key` to big endian.
    #[must_use]
    pub fn into_be(self) -> u32 {
        self.0.to_be()
    }

    /// Convert a big endian value to `Key`.
    #[must_use]
    pub fn from_be(val: u32) -> Self {
        // the val is already in big endian
        // So we need to convert it to little endian, use `to_be()`
        Self::new(val.to_be())
    }
}

impl From<u32> for Key {
    fn from(key: u32) -> Self {
        Self::new(key)
    }
}

/// Packet Sequence Number
pub type Psn = ThreeBytesStruct;

/// Queue Pair Number
pub type Qpn = ThreeBytesStruct;

/// In RDMA spec, some structs are defined as 24 bits.
/// For example : `PSN`, `QPN` etc.
///
/// This struct is used to represent these 24 bits.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct ThreeBytesStruct(u32);

impl ThreeBytesStruct {
    const BORDER: u32 = Self::MASK + 1;
    const MASK: u32 = u32::MAX >> (32 - Self::WIDTH);
    #[cfg(test)]
    pub(crate) const MAX_VALUE: u32 = Self::MASK;
    const WIDTH: usize = 24;

    /// Create a new `ThreeBytesStruct` with the given value.
    ///
    /// If the value is greater than 24 bits, the higher bits will be ignored.
    #[must_use]
    pub fn new(key: u32) -> Self {
        Self(key & Self::MASK)
    }

    /// Get the value of `ThreeBytesStruct`.
    #[must_use]
    pub fn get(&self) -> u32 {
        self.0
    }

    /// Convert the value of `ThreeBytesStruct` to big endian.
    #[must_use]
    pub fn into_be(self) -> u32 {
        // In little endian machine, to_le_bytes() is a no-op. Just get the layout.
        let key = self.0.to_le_bytes();
        // Then we reoder the bytes to big endian
        // Note that the last byte is exceed the 24 bits, any value in it will be ignored
        u32::from_le_bytes([key[2], key[1], key[0], 0])
    }

    /// Convert a big endian value to `ThreeBytesStruct`.
    #[must_use]
    pub fn from_be(val: u32) -> Self {
        // get the layout.
        let key = val.to_le_bytes();
        // from_le_bytes is also a no-op in little endian machine.
        // We just use it to convert from [u8;4] to `u32`.
        Self::new(u32::from_le_bytes([key[2], key[1], key[0], 0]))
    }

    /// wrapping add the current value with rhs
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)]
    pub fn wrapping_add(&self, rhs: u32) -> Self {
        // since (a+b) mod p  = (a + (b mod p)) mod p, we don't have to let rhs= rhs%p here
        Self((self.0 + rhs) % Self::BORDER)
    }

    /// wrapping sub the current value with rhs
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)]
    pub fn wrapping_sub(&self, rhs: u32) -> Self {
        let rhs = rhs % Self::BORDER;
        if self.0 >= rhs {
            Self(self.0 - rhs)
        } else {
            Self(Self::BORDER - rhs + self.0)
        }
    }

    /// The absolute difference between two PSN
    /// We assume that the bigger PSN should not exceed the
    /// smaller PSN by more than 2^23(that half of the range)
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)]
    pub fn wrapping_abs(&self, rhs: Psn) -> u32 {
        if self.0 >= rhs.0 {
            self.0 - rhs.get()
        } else {
            self.0 + Self::BORDER - rhs.0
        }
    }

    /// Check if the current PSN is larger or equal to the PSN in the argument
    pub(crate) fn larger_in_psn(&self, rhs: Psn) -> bool {
        let diff = self.wrapping_sub(rhs.get()).get();
        // if diff < 2^23, then self is larger or equal to rhs
        diff <= PSN_MAX_WINDOW_SIZE
    }
}

impl From<u32> for ThreeBytesStruct {
    fn from(key: u32) -> Self {
        Self::new(key)
    }
}

bitflags! {
    /// Memory access bit flags
    #[derive(Debug,Clone, Copy, PartialEq)]
    pub struct MemAccessTypeFlag: u8 {
        /// No access flag
        const IbvAccessNoFlags = 0;      // Not defined in rdma-core

        /// Local write
        const IbvAccessLocalWrite = 1;   // (1 << 0)

        /// Remote write
        const IbvAccessRemoteWrite = 2;  // (1 << 1)

        /// Remote read
        const IbvAccessRemoteRead = 4;   // (1 << 2)

        /// Remote atomic
        const IbvAccessRemoteAtomic = 8; // (1 << 3)

        /// Mw bind
        const IbvAccessMwBind = 16;      // (1 << 4)

        /// Zero based
        const IbvAccessZeroBased = 32;   // (1 << 5)

        /// On demand
        const IbvAccessOnDemand = 64;    // (1 << 6)

        /// Hugetlb
        const IbvAccessHugetlb = 128;    // (1 << 7)

        // IbvAccessRelaxedOrdering   = IBV_ACCESS_OPTIONAL_FIRST,
    }
}

bitflags! {
    /// Work Request Send Flag
    #[derive(Debug,Clone,Copy,Default,PartialEq, Eq)]
    pub struct WorkReqSendFlag: u8 {
        /// No flags
        const IbvSendNoFlags  = 0; // Not defined in rdma-core
        /// Send fence
        const IbvSendFence     = 1;
        /// Send signaled
        const IbvSendSignaled  = 2;
        /// Send solicited
        const IbvSendSolicited = 4;
        /// Send inline
        const IbvSendInline    = 8;
        /// Send IP checksum
        const IbvSendChecksum   = 16;
    }
}

/// Queue Pair Type for software/hardware
#[non_exhaustive]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum QpType {
    /// Reliable Connection
    Rc = 2,

    /// Unreliable Connection
    Uc = 3,

    /// Unreliable Datagram
    Ud = 4,

    /// Raw Packet
    RawPacket = 8,

    /// XRC Send
    XrcSend = 9,

    /// XRC Receive
    XrcRecv = 10,
}

/// Packet MTU
#[non_exhaustive]
#[derive(Default, Debug, Clone, Copy, TryFromPrimitive, PartialEq)]
#[repr(u8)]
pub enum Pmtu {
    /// 256 bytes
    #[default]
    Mtu256 = 1,

    /// 512 bytes
    Mtu512 = 2,

    /// 1024 bytes
    Mtu1024 = 3,

    /// 2048 bytes
    Mtu2048 = 4,

    /// 4096 bytes
    Mtu4096 = 5,
}

impl From<&Pmtu> for u64 {
    fn from(pmtu: &Pmtu) -> u64 {
        match pmtu {
            Pmtu::Mtu256 => 256,
            Pmtu::Mtu512 => 512,
            Pmtu::Mtu1024 => 1024,
            Pmtu::Mtu2048 => 2048,
            Pmtu::Mtu4096 => 4096,
        }
    }
}

impl From<&Pmtu> for u32 {
    fn from(pmtu: &Pmtu) -> u32 {
        match pmtu {
            Pmtu::Mtu256 => 256,
            Pmtu::Mtu512 => 512,
            Pmtu::Mtu1024 => 1024,
            Pmtu::Mtu2048 => 2048,
            Pmtu::Mtu4096 => 4096,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Msn(u16);
impl Msn {
    /// Create a new `Msn` with the given value.
    #[must_use]
    pub fn new(msn: u16) -> Self {
        Self(msn)
    }

    /// Get the value of `Msn`.
    #[must_use]
    pub fn get(&self) -> u16 {
        self.0
    }

    /// Convert the value of `Msn` to big endian.
    #[must_use]
    pub fn into_be(self) -> u16 {
        self.0.to_be()
    }
}

impl From<u16> for Msn {
    fn from(msn: u16) -> Self {
        Self::new(msn)
    }
}

impl Default for Msn {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Scatter Gather Element
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct Sge {
    /// Address(physical address)
    pub addr: u64,
    /// Length
    pub len: u32,
    /// LKey
    pub key: Key,
}

impl Sge {
    /// Create a new `Sge`
    #[must_use]
    pub fn new(addr: u64, len: u32, key: Key) -> Self {
        Self { addr, len, key }
    }
}
