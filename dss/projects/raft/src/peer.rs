use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs,
    path::Path,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

extern crate rand;
use rand::Rng;

use crate::{rpc::PeerClientRPC, EndPoint};

/// FIXME: bugs occur when self.term > other.term but self.index < other.index under derived `PartialOrd`
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct LogSeq {
    pub term: usize,
    pub index: usize,
}

impl Display for LogSeq {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{ term={}, index={} }}", self.term, self.index)
    }
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
    fn new() -> Self {
        Self::default()
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

        debug!("loading logs from {} files: {:?}", paths.len(), paths);
        if paths.is_empty() {
            Self::new()
        } else {
            unimplemented!()
        }
    }

    fn get_last_seq(&self) -> Option<LogSeq> {
        if let Some(entry) = self.entries.get(self.applied) {
            Some(entry.seq.clone())
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Vote {
    pub granted: bool,
    pub term: usize,
    pub log_seq: Option<LogSeq>,
}

impl Display for Vote {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(log_seq) = &self.log_seq {
            write!(
                f,
                "{{ granted?={}, term={}, log_seq={} }}",
                self.granted, self.term, log_seq
            )
        } else {
            write!(
                f,
                "{{ granted?={}, term={}, log_seq=None }}",
                self.granted, self.term,
            )
        }
    }
}

#[derive(Clone)]
struct FollowerState {
    next: usize,
    matched: Option<LogSeq>,
}

impl FollowerState {
    fn new(logs: usize) -> Self {
        Self {
            next: logs,
            matched: Default::default(),
        }
    }
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
pub struct Peer<C: PeerClientRPC> {
    state: Arc<Mutex<PeerState>>,
    sleep_time: Duration,
    host: EndPoint,
    peers: HashMap<EndPoint, C>,
}

impl<C: PeerClientRPC> Peer<C> {
    pub fn new(logs: &str, host: EndPoint, peer_hosts: Vec<EndPoint>) -> Self {
        debug!(
            "creating Peer on host={} with dir(logs)={}, other Peers={:?}",
            host, logs, peer_hosts
        );

        let mut peers = HashMap::new();
        for h in peer_hosts {
            let client = PeerClientRPC::connect(&h);
            peers.insert(h, client);
        }

        Self {
            state: Arc::new(Mutex::new(PeerState::new(logs))),
            sleep_time: Duration::from_millis(get_follower_deadline_rand()),
            host,
            peers,
        }
    }

    pub fn grant_for(
        &mut self,
        candidate: EndPoint,
        term: usize,
        log_seq: Option<LogSeq>,
    ) -> (bool, usize, Option<LogSeq>) {
        let mut s = self.state.lock().unwrap();

        debug!(
            "granting for peer={{ host={}, term={}, last_log_seq={:?} }}: self.term={}, self.last_log_seq={:?}",
            candidate,term,log_seq,
            s.logs.term,
            s.logs.get_last_seq(),
        );

        if term < s.logs.term || log_seq < s.logs.get_last_seq() {
            debug!(
                "deny to grant peer={} as leader: candiate's term or log is out of date",
                candidate
            );
            (false, s.logs.term, s.logs.get_last_seq())
        } else if term > s.logs.term {
            debug!(
                "grant peer={} as leader, convert self to follower: candiate's term is larger",
                candidate
            );
            s.role = Role::Follower {
                voted: Some(candidate),
                leader_alive: false,
            };
            s.logs.term = term;
            (true, s.logs.term, s.logs.get_last_seq())
        } else if let Role::Follower { voted, .. } = &mut s.role {
            if let Some(end_point) = voted {
                if &candidate == end_point {
                    debug!("have been granted peer={} as leader", candidate);
                    (true, s.logs.term, s.logs.get_last_seq())
                } else {
                    debug!( "deny ot grant peer={} as leader: have been granted other peer={} as leader", candidate, end_point);
                    (false, s.logs.term, s.logs.get_last_seq())
                }
            } else {
                debug!(
                    "grant peer={} as leader: not grant any other yet",
                    candidate
                );
                *voted = Some(candidate);
                s.logs.term = term;
                (true, s.logs.term, s.logs.get_last_seq())
            }
        } else {
            debug!(
                "deny ot grant peer={} as leader: self is leader or candidate and got same term from candidate",
                candidate
            );
            (false, s.logs.term, s.logs.get_last_seq())
        }
    }

    pub fn run(&mut self) {
        loop {
            thread::sleep(self.sleep_time);
            {
                let mut s = self.state.lock().unwrap();
                match s.role {
                    Role::Leader { .. } => {
                        debug!("running as Leader");
                        for (_, p) in &self.peers {
                            p.heart_beat();
                        }
                    }
                    Role::Candidate => {
                        s.logs.term += 1;
                        debug!("running as Candidate: term={}", s.logs.term);

                        let (vote_sender, vote_recver) = mpsc::channel::<Vote>();
                        let (tick_sender, tick_recver) = mpsc::channel::<()>();

                        thread::spawn(move || {
                            thread::sleep(Duration::from_millis(get_follower_deadline_rand()));
                            tick_sender.send(()).unwrap();
                        });

                        for (_, peer) in &self.peers {
                            peer.request_vote_async(
                                self.host.clone(),
                                s.logs.term,
                                s.logs.get_last_seq(),
                                vote_sender.clone(),
                            );
                        }

                        let mut votes = 0usize;
                        loop {
                            if let Ok(vote) = vote_recver.try_recv() {
                                if vote.granted {
                                    votes += 1;
                                    debug!(
                                        "receive granted vote from peer: {}/{}",
                                        votes,
                                        self.peers.len(),
                                    );
                                    if votes >= self.peers.len() {
                                        break;
                                    };
                                } else {
                                    debug!(
                                        "receive greater term or log seq from peer, convert self to follower: peers={}, self={{ term={}, log_seq={:?} }}",
                                        vote, s.logs.term,s.logs.get_last_seq(),
                                    );

                                    s.role = Role::Follower {
                                        voted: None,
                                        leader_alive: false,
                                    };
                                    self.sleep_time =
                                        Duration::from_millis(get_follower_deadline_rand());
                                    break;
                                }
                            } else if let Ok(_) = tick_recver.try_recv() {
                                debug!("timeout when election");
                                break;
                            } else {
                                thread::sleep(Duration::from_millis(10));
                            }
                        }
                        if votes >= self.peers.len() / 2 {
                            // become leader
                            debug!(
                                "receive {}/{} granted vote from peer, become leader",
                                votes,
                                self.peers.len()
                            );
                            let mut followers = HashMap::new();
                            for (peer_endpoint, _) in &self.peers {
                                followers.insert(
                                    peer_endpoint.clone(),
                                    FollowerState::new(s.logs.applied),
                                );
                            }
                            s.role = Role::Leader { followers }
                        }
                    }
                    Role::Follower {
                        ref voted,
                        leader_alive,
                        ..
                    } => {
                        debug!("running as Follower, voted for={:?}", voted);
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
