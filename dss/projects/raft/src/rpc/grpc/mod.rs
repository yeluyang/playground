extern crate grpcio;
use grpcio::{RpcContext, UnarySink};

use crate::{peer::Peer, rpc::Config, EndPoint};

mod rpc;
pub use rpc::{AppendRequest, AppendResponse, VoteRequest, VoteResponse};

mod rpc_grpc;
use rpc_grpc::PeerGrpc;
pub use rpc_grpc::{create_peer_grpc, PeerGrpcClient};

#[derive(Clone)]
pub struct PeerGrpcServer {
    inner: Peer,
}

impl PeerGrpcServer {
    pub fn new(config: &Config) -> Self {
        let mut inner = Peer::new(
            config.logs.as_str(),
            EndPoint {
                ip: config.ip.clone(),
                port: config.port,
            },
            config.peers.clone(),
        );

        Self { inner }
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
