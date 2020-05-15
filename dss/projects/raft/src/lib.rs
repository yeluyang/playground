use std::collections::HashMap;

mod rpc;
use rpc::PeerRPC;

enum Entry {}

struct Logger {
    // persistent state
    term: usize,
    entries: Vec<Entry>,

    // volatile state
    committed: usize,
    applied: usize,
}

struct EndPoint {
    ip: String,
    port: usize,
}

struct FollowerState {
    next: usize,
    matched: usize,
}

enum Role {
    Leader {
        followers: HashMap<EndPoint, FollowerState>,
    },
    Candidate,
    Follower {
        voted: Option<EndPoint>,
    },
}

struct Peer {
    role: Role,
    logs: Logger,
}
