use std::collections::HashMap;

mod rpc;

#[derive(Clone)]
enum Entry {}

#[derive(Clone)]
struct Logger {
    // persistent state
    term: usize,
    entries: Vec<Entry>,

    // volatile state
    committed: usize,
    applied: usize,
}

#[derive(Clone)]
struct EndPoint {
    ip: String,
    port: usize,
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

#[derive(Clone)]
struct Peer {
    role: Role,
    logs: Logger,
}
