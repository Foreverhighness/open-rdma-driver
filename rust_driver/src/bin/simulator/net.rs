//! Network components

pub mod simulator;
pub mod socket;

pub type Result<T> = core::result::Result<T, Error>;

/// Assume UDP port is always 4791.
const RDMA_PROT: u16 = 4791;

/// The maximum size that `UdpAgent` can send.
#[expect(unused, reason = "MTU may not used")]
const MTU: usize = 4128;

/// refer to [std::net::UdpSocket]
pub trait UdpAgent {
    /// Sends data to the given address. On success, returns the number of bytes written.
    ///
    /// # Errors
    ///
    /// This will return an error if `buf.len()` excess the MTU.
    ///
    /// This will return an error when the IP version of the local socket
    /// does not match that returned from [`ToSocketAddrs`].
    fn send_to(&self, buf: &[u8], addr: core::net::IpAddr) -> Result<usize>;

    /// Receives a single datagram message. On success, returns the number of bytes read and the origin.
    ///
    /// The function must be called with valid byte array buf of sufficient size to hold the message bytes.
    /// If a message is too long to fit in the supplied buffer, excess bytes may be discarded.
    fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, core::net::IpAddr)>;
}

// TODO(fh): fill with `thiserror` crate
pub enum Error {}
