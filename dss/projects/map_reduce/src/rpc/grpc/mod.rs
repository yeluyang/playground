mod map_reduce;
pub(crate) use map_reduce::{TaskGetRequest, TaskGetResponse, TaskType};

mod map_reduce_grpc;
pub(crate) use map_reduce_grpc::{MasterGrpc, MasterGrpcClient};
