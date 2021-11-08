//! # The IP-Address module.
//!
//! An IP Address identifies a single host within a network. This does not mean it tells how to
//! route a packet destined for this host, but merely allows a router to make a more informed
//! descision about what to do with a packet.
use std::{fmt, ops};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IpAddress(u32);

impl From<u32> for IpAddress {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<[u8; 4]> for IpAddress {
    fn from(octets: [u8; 4]) -> Self {
        Self::from(u32::from_be_bytes(octets))
    }
}

impl ops::Deref for IpAddress {
    /// An IP Address will dereference to its binary represetation.
    /// I found thinking of the address as one number to be more intuitive than considering it to be a series of bytes.
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IpAddress {
    /// Creates a new IP Address with the specified binary representation.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Gets an array of bytes representing this IP Address.
    pub fn octets(&self) -> [u8; 4] {
        u32::to_be_bytes(**self)
    }
}

impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d] = self.octets();
        write!(f, "{}.{}.{}.{}", a, b, c, d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        assert_eq!(IpAddress::from(0), IpAddress::from([0, 0, 0, 0]));
        assert_eq!(IpAddress::from(10), IpAddress::from([0, 0, 0, 10]));
        assert_eq!(IpAddress::from(16843009), IpAddress::from([1, 1, 1, 1]));
        assert_eq!(
            IpAddress::from(684196753),
            IpAddress::from([40, 200, 3, 145])
        );
        assert_eq!(IpAddress::from(65535), IpAddress::from([0, 0, 255, 255]));
        assert_eq!(
            IpAddress::from(u32::MAX),
            IpAddress::from([255, 255, 255, 255])
        );
    }

    #[test]
    fn octets() {
        assert_eq!([0, 0, 0, 0], IpAddress::from(0).octets());
        assert_eq!([0, 0, 0, 10], IpAddress::from(10).octets());
        assert_eq!([1, 1, 1, 1], IpAddress::from(16843009).octets());
        assert_eq!([40, 200, 3, 145], IpAddress::from(684196753).octets());
        assert_eq!([0, 0, 255, 255], IpAddress::from(65535).octets());
        assert_eq!([255, 255, 255, 255], IpAddress::from(u32::MAX).octets());
    }

    #[test]
    fn display() {
        assert_eq!("1.1.1.1", IpAddress::from([1, 1, 1, 1]).to_string());
        assert_eq!("25.25.25.25", IpAddress::from([25, 25, 25, 25]).to_string());
        assert_eq!(
            "255.255.255.255",
            IpAddress::from([255, 255, 255, 255]).to_string()
        );
        assert_eq!("40.200.3.145", IpAddress::from(684196753).to_string());
        assert_eq!("0.0.255.255", IpAddress::from(65535).to_string());
    }
}
