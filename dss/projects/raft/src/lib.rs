use std::collections::HashMap;

mod rpc;

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

impl Default for Role {
    fn default() -> Self {
        Self::Follower { voted: None }
    }
}

#[derive(Default, Clone)]
struct Peer {
    role: Role,
    logs: Logger,
}
