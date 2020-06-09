use std::{
    fmt::{self, Display, Formatter},
    sync::mpsc::Sender,
};

use crate::{logger::LogSeq, peer::Vote};

#[derive(Default, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EndPoint {
    pub ip: String,
    pub port: u16,
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
    pub fn from_hosts(hosts: &[(String, u16)]) -> Vec<Self> {
        let mut endpoints: Vec<Self> = Vec::new();

        for host in hosts {
            endpoints.push(Self::from(host.clone()));
        }

        endpoints
    }
}

pub trait PeerClientRPC: Clone {
    fn connect(host: &EndPoint) -> Self;
    fn heart_beat(&self);

    fn request_vote_async(
        &self,
        host: EndPoint,
        term: usize,
        log_seq: Option<LogSeq>,
        ch: Sender<Vote>,
    );
}
