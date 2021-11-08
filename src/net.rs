//! # The IP-Network Module
//!
//! An IP Network is a grouping of hosts, which create a communication mesh. Depending
//! on the context, the hosts within a network may have a special relationship. Just as the
//! address is only an identifier of a host, a network is only an identifier of a set of hosts.
use super::addr::IpAddress;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IpNetwork {
    base: IpAddress,
    prefix_len: u8,
}

impl IpNetwork {
    /// Creates a new IP Network struct with the specified base ip-address, and prefix length (in
    /// *bits*).
    ///
    /// The prefix length **must** be between 0 and 32, inclusive. If it were not, then we would be
    /// left with a prefix length longer than the address. This is undefined behavior.
    pub fn new(base: IpAddress, prefix_len: u8) -> Option<Self> {
        if (0..=32).contains(&prefix_len) {
            Some(Self { base, prefix_len })
        } else {
            None
        }
    }

    /// The number of bits that compose the network prefix.
    ///
    /// This will be the number of leading bits that are required to be **identical** to the
    /// network's base address, in order to be considered included within that network.
    pub fn num_network_bits(&self) -> u8 {
        self.prefix_len
    }

    /// The number of bits that compose the network suffix.
    ///
    /// This will be the number of bits that differentiate each host within the network.
    pub fn num_host_bits(&self) -> u8 {
        32 - self.num_network_bits()
    }
}

impl fmt::Display for IpNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.base, self.num_network_bits())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            "0.0.0.0/0",
            IpNetwork::new(IpAddress::from(0), 0).unwrap().to_string()
        );
        assert_eq!(
            "255.255.255.255/32",
            IpNetwork::new(IpAddress::from([255, 255, 255, 255]), 32)
                .unwrap()
                .to_string()
        );
    }

    #[test]
    fn bad_prefix_len() {
        for i in 0..=32 {
            assert!(IpNetwork::new(IpAddress::from(0), i).is_some());
        }
        assert!(IpNetwork::new(IpAddress::from(0), 33).is_none());
        assert!(IpNetwork::new(IpAddress::from(0), 200).is_none());
        assert!(IpNetwork::new(IpAddress::from(0), 255).is_none());
    }
}
