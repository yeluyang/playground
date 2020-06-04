use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

extern crate rand;
use rand::Rng;

use crate::{rpc::PeerClient, EndPoint};

#[derive(Clone)]
pub(crate) struct LogSeq {
    pub term: usize,
    pub index: usize,
}

#[derive(Clone)]
struct Entry {
    seq: LogSeq,
}

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
        leader_alive: bool,
    },
}

impl Default for Role {
    fn default() -> Self {
        Self::Follower {
            voted: None,
            leader_alive: false,
        }
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
            role: Role::Follower {
                voted: None,
                leader_alive: false,
            },
            logs: Logger::load(logs),
        }
    }
}

#[derive(Default, Clone)]
pub struct Peer {
    state: Arc<Mutex<PeerState>>,
    sleep_time: Duration,
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
            sleep_time: Duration::from_millis(get_follower_deadline_rand()),
            host,
            peers,
        }
    }

    pub fn run(&mut self) {
        loop {
            thread::sleep(self.sleep_time);
            {
                let mut s = self.state.lock().unwrap();
                match s.role {
                    Role::Leader { .. } => {
                        for (_, p) in &self.peers {
                            p.heart_beat();
                        }
                    }
                    Role::Candidate => {
                        s.logs.term += 1;

                        let (vote_sender, vote_recver) = mpsc::channel();
                        let (tick_sender, tick_recver) = mpsc::channel::<()>();

                        thread::spawn(move || {
                            thread::sleep(Duration::from_millis(get_follower_deadline_rand()));
                            tick_sender.send(()).unwrap();
                        });

                        for (_, peer) in &self.peers {
                            peer.request_vote_async(
                                self.host.clone(),
                                s.logs.term,
                                s.logs.entries[s.logs.applied].seq.clone(),
                                vote_sender.clone(),
                            );
                        }

                        let mut votes = 1usize;
                        while votes < (self.peers.len() / 2) + 1 {
                            if let Ok((granted, term)) = vote_recver.try_recv() {
                                if granted {
                                    votes += 1;
                                } else if term > s.logs.term {
                                    s.role = Role::Follower {
                                        voted: None,
                                        leader_alive: false,
                                    };
                                    self.sleep_time =
                                        Duration::from_millis(get_follower_deadline_rand());
                                    break;
                                }
                            } else if let Ok(_) = tick_recver.try_recv() {
                                break;
                            } else {
                                thread::sleep(Duration::from_millis(10));
                            }
                        }
                        if votes > (self.peers.len() / 2) + 1 {
                            // become leader
                            unimplemented!()
                        }
                    }
                    Role::Follower { leader_alive, .. } => {
                        if !leader_alive {
                            s.role = Role::Candidate;
                            self.sleep_time = Duration::from_millis(0);
                        }
                    }
                }
            }
        }
    }
}

fn get_follower_deadline_rand() -> u64 {
    rand::thread_rng().gen_range(100, 500 + 1)
}
