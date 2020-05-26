use std::fmt::{self, Display, Formatter};

mod rpc;
pub use rpc::{NodeClient, NodeServer};

#[derive(Debug)]
pub struct EndPoint {
    ip: String,
    port: u16,
}

impl Display for EndPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}
