mod grpc;
use grpc::{NodeGrpcClient, NodeGrpcServer};

struct NodeServer {
    server: NodeGrpcServer,
}

struct NodeClient {
    client: NodeGrpcClient,
}
