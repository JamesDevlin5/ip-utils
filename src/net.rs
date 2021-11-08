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

    /// Gets the number of individual hosts that reside within this network.
    pub fn num_hosts(&self) -> u64 {
        2_u64.pow(self.num_host_bits().into())
    }

    /// The supernet of some IP network is one bit less-specific than its subnets. This means that
    /// the address space is one bit more ambiguous, and offers a power of two more addresses
    /// within the network set.
    pub fn supernet(self) -> Option<Self> {
        match self.num_network_bits() {
            0 => None,
            n => Self::new(self.base, n - 1),
        }
    }

    /// Gets the two children of this network. The point of contention for these two networks will
    /// be the immediate new bit in the prefix. This bit may be a `1` or a `0`, where before this
    /// number was irrelevant. Each of these children networks will contain exactly half of the
    /// supernet.
    pub fn subnets(self) -> Option<(Self, Self)> {
        if let Some(lower_net) = Self::new(self.base, self.num_network_bits() + 1) {
            let mut upper_net = lower_net.clone();
            upper_net.base = (*lower_net.base | (1 << lower_net.num_host_bits())).into();
            Some((upper_net, lower_net))
        } else {
            None
        }
    }

    /// Creates the mask associated with this network, in IP Address form.
    pub fn get_mask(&self) -> IpAddress {
        match self.num_network_bits() {
            // Overflow check
            32 => IpAddress::from(u32::MAX),
            n => IpAddress::from(!(u32::MAX >> n)),
        }
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

    #[test]
    fn supernet() {
        assert!(IpNetwork::new(IpAddress::from(0), 0)
            .unwrap()
            .supernet()
            .is_none());
        assert_eq!(
            IpNetwork::new(IpAddress::from(0), 1)
                .unwrap()
                .supernet()
                .unwrap(),
            IpNetwork::new(IpAddress::from(0), 0).unwrap()
        );
        assert_eq!(
            IpNetwork::new(IpAddress::from(4290772992), 10)
                .unwrap()
                .supernet()
                .unwrap(),
            IpNetwork::new(IpAddress::from(4290772992), 9).unwrap()
        );
        assert_eq!(
            IpNetwork::new(IpAddress::from(4292870144), 14)
                .unwrap()
                .supernet()
                .unwrap(),
            IpNetwork::new(IpAddress::from(4292870144), 13).unwrap()
        );
        assert_eq!(
            IpNetwork::new(IpAddress::from(3578789888), 30)
                .unwrap()
                .supernet()
                .unwrap(),
            IpNetwork::new(IpAddress::from(3578789888), 29).unwrap()
        );
        assert_eq!(
            IpNetwork::new(IpAddress::from(u32::MAX - 3), 32)
                .unwrap()
                .supernet()
                .unwrap(),
            IpNetwork::new(IpAddress::from(u32::MAX - 3), 31).unwrap()
        );
    }

    #[test]
    fn mask_gen() {
        assert_eq!(
            IpAddress::from(0),
            IpNetwork::new(IpAddress::from(u32::MAX), 0)
                .unwrap()
                .get_mask()
        );
        assert_eq!(
            IpAddress::from(0),
            IpNetwork::new(IpAddress::from(21010), 0)
                .unwrap()
                .get_mask()
        );
        assert_eq!(
            IpAddress::from([255, 255, 0, 0]),
            IpNetwork::new(IpAddress::from(0), 16).unwrap().get_mask()
        );
        assert_eq!(
            IpAddress::from([255, 255, 248, 0]),
            IpNetwork::new(IpAddress::from(0), 21).unwrap().get_mask()
        );
        assert_eq!(
            IpAddress::from([255, 255, 252, 0]),
            IpNetwork::new(IpAddress::from(0), 22).unwrap().get_mask()
        );
        assert_eq!(
            IpAddress::from([255, 255, 254, 0]),
            IpNetwork::new(IpAddress::from(0), 23).unwrap().get_mask()
        );
        assert_eq!(
            IpAddress::from([255, 255, 255, 0]),
            IpNetwork::new(IpAddress::from(0), 24).unwrap().get_mask()
        );
        assert_eq!(
            IpAddress::from([255, 255, 255, 128]),
            IpNetwork::new(IpAddress::from(0), 25).unwrap().get_mask()
        );
        assert_eq!(
            IpAddress::from([255, 255, 255, 255]),
            IpNetwork::new(IpAddress::from(0), 32).unwrap().get_mask()
        );
    }

    #[test]
    fn num_hosts() {
        assert_eq!(
            1,
            IpNetwork::new(IpAddress::from(0), 32).unwrap().num_hosts()
        );
        assert_eq!(
            2,
            IpNetwork::new(IpAddress::from(0), 31).unwrap().num_hosts()
        );
        assert_eq!(
            4096,
            IpNetwork::new(IpAddress::from(0), 20).unwrap().num_hosts()
        );
        assert_eq!(
            4194304,
            IpNetwork::new(IpAddress::from(0), 10).unwrap().num_hosts()
        );
        assert_eq!(
            2147483648,
            IpNetwork::new(IpAddress::from(0), 1).unwrap().num_hosts()
        );
        assert_eq!(
            4294967296,
            IpNetwork::new(IpAddress::from(0), 0).unwrap().num_hosts()
        );
    }
}
