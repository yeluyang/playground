extern crate grpcio;
use grpcio::{RpcContext, UnarySink};

mod rpc;
pub(crate) use rpc::{PingRequest, PingResponse};

mod rpc_grpc;
use rpc_grpc::NodeGrpc;
pub(crate) use rpc_grpc::NodeGrpcClient;

pub(crate) struct NodeGrpcServer {}

impl NodeGrpc for NodeGrpcServer {
    fn ping(&mut self, ctx: RpcContext, req: PingRequest, sink: UnarySink<PingResponse>) {
        sink.success(PingResponse::new());
    }
}
