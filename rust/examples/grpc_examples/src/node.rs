use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::rpc::NodeClient;

#[derive(Clone)]
pub struct Node {
    id: String,
    ping: Arc<Mutex<HashMap<String, usize>>>,
    peers: Vec<NodeClient>,
}

impl Node {
    pub fn new(id: String, peers_host: Vec<(String, u16)>) -> Self {
        let mut peers = Vec::new();
        for (ip, port) in peers_host {
            peers.push(NodeClient::new(ip, port));
        }

        Self {
            id,
            ping: Arc::new(Mutex::new(HashMap::new())),
            peers,
        }
    }

    pub fn pong(&mut self, id: String) -> &str {
        {
            let mut ping = self.ping.lock().unwrap();
            if let Some(count) = ping.get_mut(&id) {
                *count += 1;
            } else {
                ping.insert(id, 1);
            }
        }
        &self.id
    }

    pub fn run(&self) {
        loop {
            thread::sleep(Duration::from_secs(4));
            println!("node={{id={}}}: {:?}", self.id, self.ping);
            for p in &self.peers {
                p.ping(self.id.clone());
            }
        }
    }
}
