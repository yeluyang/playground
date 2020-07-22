use std::sync::Arc;

extern crate serde;
use serde::Deserialize;

extern crate grpcio;
use grpcio::{ChannelBuilder, EnvBuilder, Server, ServerBuilder};

mod grpc;
use grpc::{
    AppendRequest, AppendResponse, PeerGrpcClient, PeerGrpcServer, VoteRequest, VoteResponse,
};

use raft::{EndPoint, LogSeq, PeerClientRPC, Receipt, Vote};

#[derive(Clone, Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub logs: String,
    pub peers: Vec<(String, u16)>,
}

pub struct PeerServer {
    config: Config,
    inner: Server,
    runner: PeerGrpcServer,
}

impl PeerServer {
    pub fn new(config: Config) -> Self {
        let host = EndPoint::from((config.ip.clone(), config.port));
        let peers = EndPoint::from_hosts(config.peers.clone());
        let runner = PeerGrpcServer::new(host, &config.logs, peers);

        let inner = ServerBuilder::new(Arc::new(EnvBuilder::new().build()))
            .register_service(grpc::create_peer_grpc(runner.clone()))
            .bind(config.ip.clone(), config.port)
            .build()
            .unwrap();

        Self {
            config,
            inner,
            runner,
        }
    }

    pub fn run(&mut self) {
        self.inner.start();
        self.runner.run();
    }
}

#[derive(Clone)]
pub struct PeerClient {
    inner: PeerGrpcClient,
}

impl PeerClientRPC for PeerClient {
    fn connect(host: &EndPoint) -> Self {
        Self {
            inner: PeerGrpcClient::new(
                ChannelBuilder::new(Arc::new(EnvBuilder::new().build()))
                    .connect(host.to_string().as_str()),
            ),
        }
    }

    fn heart_beat(&self, leader: EndPoint, term: usize) -> Receipt {
        let mut req = AppendRequest::new();
        req.set_term(term as i64);
        req.set_leader(grpc::grpc_end_point_from(leader));

        let mut rsp = self.inner.append(&req).unwrap();
        debug!(
            "got response of heart_beat from peer{{addr={},term={}}}: {}",
            grpc::crate_end_point_from(rsp.get_follower().clone()),
            rsp.get_term(),
            rsp.get_success(),
        );
        Receipt {
            endpoint: grpc::crate_end_point_from(rsp.take_follower()),
            term: rsp.get_term() as usize,
            success: rsp.get_success(),
        }
    }

    fn request_vote(&self, host: EndPoint, term: usize, log_seq: Option<LogSeq>) -> Vote {
        let mut req = VoteRequest::new();
        req.set_term(term as i64);
        req.set_last_log_seq(grpc::grpc_log_seq_from(log_seq));
        req.set_candidate(grpc::grpc_end_point_from(host));

        let mut rsp = self.inner.vote(&req).unwrap();
        debug!(
            "got response of heart_beat from peer{{addr={}, term={}, log_seq={:?}}}: {}",
            grpc::crate_end_point_from(rsp.get_peer().clone()),
            rsp.get_term(),
            grpc::crate_log_seq_from(rsp.get_last_log_seq().clone()),
            rsp.get_granted(),
        );
        Vote {
            peer: grpc::crate_end_point_from(rsp.take_peer()),
            granted: rsp.get_granted(),
            term: rsp.get_term() as usize,
            log_seq: grpc::crate_log_seq_from(rsp.take_last_log_seq()),
        }
    }
}
