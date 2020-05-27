use std::fmt::{self, Display, Formatter};

mod peer;

mod rpc;

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
