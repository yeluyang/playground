use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

extern crate rand;
use rand::Rng;

use crate::{
    error::Result,
    logger::{LogSeq, Logger},
    rpc::{EndPoint, PeerClientRPC},
};

#[derive(Debug)]
pub struct Vote {
    pub peer: EndPoint,
    pub granted: bool,
    pub term: usize,
    pub log_seq: Option<LogSeq>,
}

impl Display for Vote {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(log_seq) = &self.log_seq {
            write!(
                f,
                "{{ granted?={} from peer {{endpoint={}, term={}, log_seq={}}} }}",
                self.peer, self.granted, self.term, log_seq
            )
        } else {
            write!(
                f,
                "{{ granted?={} from peer {{endpoint={}, term={}, log_seq=None }} }}",
                self.peer, self.granted, self.term,
            )
        }
    }
}

#[derive(Debug)]
pub struct Receipt {
    pub success: bool,
    pub term: usize,
    pub endpoint: EndPoint,
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
            let client = PeerClientRPC::connect(h.clone());
            peers.insert(h, client);
        }

        Self {
            state: Arc::new(Mutex::new(PeerState::new(logs))),
            sleep_time: get_raft_sleep_deadline_rand(),
            host,
            peers,
        }
    }

    pub fn append(&mut self, leader: EndPoint, term: usize) -> Receipt {
        let mut s = self.state.lock().unwrap();
        if term < s.logs.term {
            Receipt {
                endpoint: self.host.clone(),
                success: false,
                term: s.logs.term,
            }
        } else {
            s.logs.term = term;
            match s.role {
                Role::Follower {
                    ref mut voted,
                    ref mut leader_alive,
                } => {
                    trace!(
                        "get heart beat from leader={{host={}, term={}}}, stay in follower",
                        leader,
                        term,
                    );
                    *voted = None;
                    *leader_alive = true;
                }
                Role::Candidate => {
                    debug!(
                        "get heart beat from leader={{host={}, term={}}}, convert self to follower",
                        leader, term,
                    );
                    s.role = Role::Follower {
                        voted: None,
                        leader_alive: true,
                    };
                }
                Role::Leader { .. } => unimplemented!(),
            }
            Receipt {
                endpoint: self.host.clone(),
                success: true,
                term: s.logs.term,
            }
        }
    }

    pub fn grant_for(&mut self, candidate: EndPoint, term: usize, log_seq: Option<LogSeq>) -> Vote {
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
            Vote {
                peer: self.host.clone(),
                granted: false,
                term: s.logs.term,
                log_seq: s.logs.get_last_seq(),
            }
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
            Vote {
                peer: self.host.clone(),
                granted: true,
                term: s.logs.term,
                log_seq: s.logs.get_last_seq(),
            }
        } else if let Role::Follower { voted, .. } = &mut s.role {
            if let Some(end_point) = voted {
                if &candidate == end_point {
                    debug!("have been granted peer={} as leader", candidate);
                    Vote {
                        peer: self.host.clone(),
                        granted: true,
                        term: s.logs.term,
                        log_seq: s.logs.get_last_seq(),
                    }
                } else {
                    debug!( "deny ot grant peer={} as leader: have been granted other peer={} as leader", candidate, end_point);
                    Vote {
                        peer: self.host.clone(),
                        granted: false,
                        term: s.logs.term,
                        log_seq: s.logs.get_last_seq(),
                    }
                }
            } else {
                debug!(
                    "grant peer={} as leader: not grant any other yet",
                    candidate
                );
                *voted = Some(candidate);
                s.logs.term = term;
                Vote {
                    peer: self.host.clone(),
                    granted: true,
                    term: s.logs.term,
                    log_seq: s.logs.get_last_seq(),
                }
            }
        } else {
            debug!(
                "deny ot grant peer={} as leader: self is leader or candidate and got same term from candidate",
                candidate
            );
            Vote {
                peer: self.host.clone(),
                granted: false,
                term: s.logs.term,
                log_seq: s.logs.get_last_seq(),
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            thread::sleep(self.sleep_time);
            {
                let mut s = self.state.lock().unwrap();
                match s.role {
                    Role::Leader { .. } => {
                        trace!("running as Leader");
                        for (_, p) in &self.peers {
                            p.heart_beat(self.host.clone(), s.logs.term);
                        }
                    }
                    Role::Candidate => {
                        s.logs.term += 1;
                        self.sleep_time = get_raft_sleep_deadline_rand();
                        debug!(
                            "running as Candidate: term={}, timeout_after={:?}",
                            s.logs.term, self.sleep_time
                        );

                        let (vote_sender, vote_recver) = mpsc::channel::<Result<Vote>>();
                        let (tick_sender, tick_recver) = mpsc::channel::<()>();

                        let timeout_duration = self.sleep_time;
                        thread::spawn(move || {
                            thread::sleep(timeout_duration);
                            tick_sender.send(()).unwrap_or_else(|err| {
                                error!("candidate timeout ticker error: {}", err)
                            });
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
                                match vote {
                                    Ok(vote) => {
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
                                                vote,
                                                s.logs.term,
                                                s.logs.get_last_seq()
                                            );

                                            s.role = Role::Follower {
                                                voted: None,
                                                leader_alive: false,
                                            };
                                            self.sleep_time = get_raft_sleep_deadline_rand();
                                            break;
                                        }
                                    }
                                    Err(err) => debug!("failed to get vote: {}", err),
                                };
                            } else if let Ok(_) = tick_recver.try_recv() {
                                debug!("timeout when election");
                                break;
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
                        ref mut leader_alive,
                        ..
                    } => {
                        trace!("running as Follower, voted for={:?}", voted);
                        if !*leader_alive {
                            s.role = Role::Candidate;
                            self.sleep_time = Duration::from_millis(0);
                        } else {
                            *leader_alive = false
                        }
                    }
                }
            }
        }
    }
}

fn get_raft_sleep_deadline_rand() -> Duration {
    Duration::from_millis(rand::thread_rng().gen_range(100..=500))
}
