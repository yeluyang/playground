extern crate grpcio;
use grpcio::{RpcContext, UnarySink};

mod rpc;
pub use rpc::{PingRequest, PingResponse};

mod rpc_grpc;
use rpc_grpc::NodeGrpc;
pub use rpc_grpc::{create_node_grpc, NodeGrpcClient};

#[derive(Clone)]
pub struct NodeGrpcServer {}

impl NodeGrpcServer {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn run(&self) {
        unimplemented!()
    }
}

impl NodeGrpc for NodeGrpcServer {
    fn ping(&mut self, ctx: RpcContext, req: PingRequest, sink: UnarySink<PingResponse>) {
        sink.success(PingResponse::new());
    }
}
