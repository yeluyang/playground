use std::{
    fmt::{self, Display, Formatter},
    sync::mpsc::Sender,
    thread,
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

impl From<&(&str, u16)> for EndPoint {
    fn from(host: &(&str, u16)) -> Self {
        Self::from((host.0.to_owned(), host.1))
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
    pub fn from_hosts(hosts: Vec<(String, u16)>) -> Vec<Self> {
        let mut endpoints: Vec<Self> = Vec::new();

        for host in hosts {
            endpoints.push(Self::from(host));
        }

        endpoints
    }
}

pub trait PeerClientRPC: Send + Clone + 'static {
    fn connect(host: &EndPoint) -> Self;

    fn heart_beat(&self);

    fn request_vote(&self, host: EndPoint, term: usize, log_seq: Option<LogSeq>) -> Vote;

    fn request_vote_async(
        &self,
        host: EndPoint,
        term: usize,
        log_seq: Option<LogSeq>,
        ch: Sender<Vote>,
    ) {
        let agent = self.clone();
        thread::spawn(move || {
            ch.send(agent.request_vote(host, term, log_seq)).unwrap();
        });
    }
}
