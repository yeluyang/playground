mod grpc;
use grpc::{
    MasterGrpcClient, MasterGrpcServer, PingRequest, PingResponse, TaskGetRequest, TaskGetResponse,
};

pub(crate) struct MasterServer {
    server: MasterGrpcServer,
}

pub(crate) struct MasterClient {
    client: MasterGrpcClient,
}
