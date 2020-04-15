mod map_reduce;
pub(crate) use self::map_reduce::{JobGetRequest, JobGetResponse, TaskType};

mod map_reduce_grpc;
pub(crate) use self::map_reduce_grpc::{MasterGrpc, MasterGrpcClient};
