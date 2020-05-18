extern crate grpcio;
use grpcio::{RpcContext, UnarySink};

use crate::{rpc::Config, EndPoint, Peer};

mod rpc;
pub(crate) use rpc::{AppendRequest, AppendResponse, VoteRequest, VoteResponse};

mod rpc_grpc;
use rpc_grpc::PeerGrpc;
pub(crate) use rpc_grpc::{create_peer_grpc, PeerGrpcClient};

#[derive(Clone)]
pub(crate) struct PeerGrpcServer {
    inner: Peer,
}

impl PeerGrpcServer {
    pub fn new(config: Config) -> Self {
        let mut inner = Peer::new(
            config.logs,
            EndPoint {
                ip: config.ip,
                port: config.port,
            },
            config.peers,
        );

        inner.run();

        Self { inner }
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
