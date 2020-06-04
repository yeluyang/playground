extern crate grpcio;
use grpcio::{RpcContext, UnarySink};

use crate::peer::{self, Peer};

mod rpc;
pub use rpc::{AppendRequest, AppendResponse, VoteRequest, VoteResponse};

mod rpc_grpc;
use rpc_grpc::PeerGrpc;
pub use rpc_grpc::{create_peer_grpc, PeerGrpcClient};

pub fn crate_end_point_from(mut rpc_end_point: rpc::EndPoint) -> crate::EndPoint {
    crate::EndPoint {
        ip: rpc_end_point.take_ip(),
        port: rpc_end_point.get_port() as u16,
    }
}

pub fn grpc_end_point_from(crate_end_point: crate::EndPoint) -> rpc::EndPoint {
    rpc::EndPoint {
        ip: crate_end_point.ip,
        port: crate_end_point.port as i64,
        unknown_fields: Default::default(),
        cached_size: Default::default(),
    }
}

pub fn crate_log_seq_from(grpc_log_seq: rpc::LogSeq) -> peer::LogSeq {
    peer::LogSeq {
        term: grpc_log_seq.term as usize,
        index: grpc_log_seq.index as usize,
    }
}

pub fn grpc_log_seq_from(crate_log_seq: peer::LogSeq) -> rpc::LogSeq {
    rpc::LogSeq {
        term: crate_log_seq.term as i64,
        index: crate_log_seq.index as i64,
        unknown_fields: Default::default(),
        cached_size: Default::default(),
    }
}

#[derive(Clone)]
pub struct PeerGrpcServer {
    inner: Peer,
}

impl PeerGrpcServer {
    pub fn new(host: crate::EndPoint, logs: &str, peers: Vec<crate::EndPoint>) -> Self {
        Self {
            inner: Peer::new(logs, host, peers),
        }
    }

    pub fn run(&mut self) {
        self.inner.run();
    }
}

impl PeerGrpc for PeerGrpcServer {
    fn vote(&mut self, ctx: RpcContext, req: VoteRequest, sink: UnarySink<VoteResponse>) {
        unimplemented!()
    }

    fn append(&mut self, ctx: RpcContext, req: AppendRequest, sink: UnarySink<AppendResponse>) {
        unimplemented!()
    }
}
