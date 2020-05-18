use std::collections::HashMap;

mod rpc;
use rpc::PeerClient;

#[derive(Clone)]
enum Entry {}

#[derive(Default, Clone)]
struct Logger {
    // persistent state
    term: usize,
    entries: Vec<Entry>,

    // volatile state
    committed: usize,
    applied: usize,
}

impl Logger {
    fn new(term: usize, entries: Vec<Entry>) -> Self {
        Self {
            term,
            entries,
            committed: 0usize,
            applied: 0usize,
        }
    }
}

#[derive(Default, Clone)]
struct EndPoint {
    ip: String,
    port: u16,
}

#[derive(Clone)]
struct FollowerState {
    next: usize,
    matched: usize,
}

#[derive(Clone)]
enum Role {
    Leader {
        followers: HashMap<EndPoint, FollowerState>,
    },
    Candidate,
    Follower {
        voted: Option<EndPoint>,
    },
}

impl Default for Role {
    fn default() -> Self {
        Self::Follower { voted: None }
    }
}

#[derive(Default, Clone)]
struct Peer {
    role: Role,
    logs: Logger,

    host: EndPoint,
    peers: Vec<PeerClient>,
}

impl Peer {
    fn new(logs: String, host: EndPoint, peers: Vec<EndPoint>) -> Self {
        unimplemented!()
    }

    fn run(&mut self) {
        unimplemented!()
    }
}
