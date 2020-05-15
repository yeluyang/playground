mod grpc;
use grpc::{PeerGrpcClient, PeerGrpcServer};

struct PeerServer {
    inner: PeerGrpcServer,
}

impl PeerServer {
    fn new() -> Self {
        unimplemented!()
    }

    fn run(&mut self) -> Self {
        unimplemented!()
    }
}

struct PeerClient {
    inner: PeerGrpcClient,
}

impl PeerClient {
    fn connect() -> Self {
        unimplemented!()
    }
}
