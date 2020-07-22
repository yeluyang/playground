extern crate grpcio;
use grpcio::{RpcContext, UnarySink};

use raft::{self, Peer};

use crate::rpc::PeerClient;

mod rpc;
pub use rpc::{AppendRequest, AppendResponse, VoteRequest, VoteResponse};

mod rpc_grpc;
use rpc_grpc::PeerGrpc;
pub use rpc_grpc::{create_peer_grpc, PeerGrpcClient};

pub fn crate_end_point_from(mut rpc_end_point: rpc::EndPoint) -> raft::EndPoint {
    raft::EndPoint {
        ip: rpc_end_point.take_ip(),
        port: rpc_end_point.get_port() as u16,
    }
}

pub fn grpc_end_point_from(crate_end_point: raft::EndPoint) -> rpc::EndPoint {
    rpc::EndPoint {
        ip: crate_end_point.ip,
        port: crate_end_point.port as i64,
        unknown_fields: Default::default(),
        cached_size: Default::default(),
    }
}

pub fn crate_log_seq_from(grpc_log_seq: rpc::LogSeq) -> Option<raft::LogSeq> {
    assert!(grpc_log_seq.term * grpc_log_seq.index > 0);
    if grpc_log_seq.term > 0 {
        Some(raft::LogSeq {
            term: grpc_log_seq.term as usize,
            index: grpc_log_seq.index as usize,
        })
    } else {
        None
    }
}

pub fn grpc_log_seq_from(crate_log_seq: Option<raft::LogSeq>) -> rpc::LogSeq {
    if let Some(crate_log_seq) = crate_log_seq {
        rpc::LogSeq {
            term: crate_log_seq.term as i64,
            index: crate_log_seq.index as i64,
            unknown_fields: Default::default(),
            cached_size: Default::default(),
        }
    } else {
        rpc::LogSeq {
            term: -1i64,
            index: -1i64,
            unknown_fields: Default::default(),
            cached_size: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct PeerGrpcServer {
    inner: Peer<PeerClient>,
}

impl PeerGrpcServer {
    pub fn new(host: raft::EndPoint, logs: &str, peers: Vec<raft::EndPoint>) -> Self {
        Self {
            inner: Peer::new(logs, host, peers),
        }
    }

    pub fn run(&mut self) {
        self.inner.run();
    }
}

impl PeerGrpc for PeerGrpcServer {
    fn vote(&mut self, ctx: RpcContext, mut req: VoteRequest, sink: UnarySink<VoteResponse>) {
        debug!("got vote request: request={{ {:?} }}", req);

        let vote = self.inner.grant_for(
            crate_end_point_from(req.take_candidate()),
            req.get_term() as usize,
            crate_log_seq_from(req.take_last_log_seq()),
        );

        let mut rsp = VoteResponse::new();
        rsp.set_granted(vote.granted);
        rsp.set_term(vote.term as i64);
        rsp.set_last_log_seq(grpc_log_seq_from(vote.log_seq));

        sink.success(rsp);
    }

    fn append(&mut self, ctx: RpcContext, mut req: AppendRequest, sink: UnarySink<AppendResponse>) {
        let receipt = self.inner.append(
            crate_end_point_from(req.take_leader()),
            req.get_term() as usize,
        );

        let mut rsp = AppendResponse::new();
        rsp.set_follower(grpc_end_point_from(receipt.endpoint));
        rsp.set_term(receipt.term as i64);
        rsp.set_success(receipt.success);

        sink.success(rsp);
    }
}
