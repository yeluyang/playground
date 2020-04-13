extern crate grpcio;
use grpcio::{RpcContext, UnarySink};

mod map_reduce;
pub(crate) use map_reduce::{PingRequest, PingResponse, TaskGetRequest, TaskGetResponse};

mod map_reduce_grpc;
use map_reduce_grpc::MasterGrpc;
pub(crate) use map_reduce_grpc::MasterGrpcClient;

pub(crate) struct MasterGrpcServer {}

impl MasterGrpc for MasterGrpcServer {
    fn ping(&mut self, ctx: RpcContext, req: PingRequest, sink: UnarySink<PingResponse>) {
        sink.success(PingResponse::new());
    }

    fn task_get(&mut self, ctx: RpcContext, req: TaskGetRequest, sink: UnarySink<TaskGetResponse>) {
        sink.success(TaskGetResponse::new());
    }
}
