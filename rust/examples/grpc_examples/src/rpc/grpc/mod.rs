extern crate grpcio;
use grpcio::{RpcContext, UnarySink};

use crate::node::Node;

mod rpc;
pub use rpc::{PingRequest, PingResponse};

mod rpc_grpc;
use rpc_grpc::NodeGrpc;
pub use rpc_grpc::{create_node_grpc, NodeGrpcClient};

#[derive(Clone)]
pub struct NodeGrpcServer {
    inner: Node,
}

impl NodeGrpcServer {
    pub fn new(id: String, peers_host: Vec<(String, u16)>) -> Self {
        Self {
            inner: Node::new(id, peers_host),
        }
    }

    pub fn run(&self) {
        self.inner.run();
    }
}

impl NodeGrpc for NodeGrpcServer {
    fn ping(&mut self, ctx: RpcContext, mut req: PingRequest, sink: UnarySink<PingResponse>) {
        let id = self.inner.pong(req.take_id()).to_owned();

        let mut rsp = PingResponse::new();
        rsp.set_id(id);

        sink.success(rsp);
    }
}
