extern crate grpcio;

use grpcio::{RpcContext, UnarySink};

mod rpc;
mod rpc_grpc;

use rpc::{PingRequest, PingResponse, TaskGetRequest, TaskGetResponse};

pub struct MasterServer {}

impl rpc_grpc::MasterRpc for MasterServer {
    fn ping(&mut self, ctx: RpcContext, req: PingRequest, sink: UnarySink<PingResponse>) {
        sink.success(PingResponse::new());
    }

    fn task_get(&mut self, ctx: RpcContext, req: TaskGetRequest, sink: UnarySink<TaskGetResponse>) {
        sink.success(TaskGetResponse::new());
    }
}
