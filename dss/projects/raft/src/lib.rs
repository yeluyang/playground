use std::fmt::{self, Display, Formatter};

#[macro_use]
extern crate log;

mod peer;

mod rpc;
pub use rpc::{Config, PeerClient, PeerServer};

#[derive(Default, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EndPoint {
    ip: String,
    port: u16,
}

impl Display for EndPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

impl From<(String, u16)> for EndPoint {
    fn from(host: (String, u16)) -> Self {
        Self {
            ip: host.0,
            port: host.1,
        }
    }
}

impl EndPoint {
    fn from_hosts(hosts: &[(String, u16)]) -> Vec<Self> {
        let mut endpoints: Vec<Self> = Vec::new();

        for host in hosts {
            endpoints.push(Self::from(host.clone()));
        }

        endpoints
    }
}
