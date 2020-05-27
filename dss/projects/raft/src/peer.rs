use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::{rpc::PeerClient, EndPoint};

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

    fn load(dir: &str) -> Self {
        let path = Path::new(dir);
        assert!(path.is_dir());

        let mut paths = Vec::new();
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if let Some(ext) = path.extension() {
                if ext == "wal" {
                    paths.push(path);
                }
            }
        }

        if paths.is_empty() {
            Self::new(0, Vec::new())
        } else {
            unimplemented!()
        }
    }
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

#[derive(Default)]
struct PeerState {
    role: Role,
    logs: Logger,
}

impl PeerState {
    fn new(logs: &str) -> Self {
        Self {
            role: Role::Follower { voted: None },
            logs: Logger::load(logs),
        }
    }
}

#[derive(Default, Clone)]
pub struct Peer {
    state: Arc<Mutex<PeerState>>,
    host: EndPoint,
    peers: HashMap<EndPoint, PeerClient>,
}

impl Peer {
    pub fn new(logs: &str, host: EndPoint, peer_hosts: Vec<EndPoint>) -> Self {
        let mut peers = HashMap::new();
        for h in peer_hosts {
            let client = PeerClient::connect(&h);
            peers.insert(h, client);
        }

        Self {
            state: Arc::new(Mutex::new(PeerState::new(logs))),
            host,
            peers,
        }
    }

    pub fn run(&mut self) {
        unimplemented!()
    }
}
