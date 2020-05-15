use std::{sync::Arc, thread, time::Duration};

extern crate grpcio;
use grpcio::{EnvBuilder, Server, ServerBuilder};

mod grpc;
use grpc::{PeerGrpcClient, PeerGrpcServer};

struct Config {
    ip: String,
    port: u16,
}

struct PeerServer {
    config: Config,
    inner: Server,
}

impl PeerServer {
    fn new(config: Config) -> Self {
        let inner = ServerBuilder::new(Arc::new(EnvBuilder::new().build()))
            .register_service(grpc::create_peer_grpc(PeerGrpcServer::new()))
            .bind(config.ip.clone(), config.port)
            .build()
            .unwrap();
        Self { config, inner }
    }

    fn run(&mut self, time: Option<Duration>) {
        self.inner.start();
        match time {
            Some(time) => thread::sleep(time),
            None => loop {
                thread::sleep(Duration::new(u64::MAX, u32::MAX))
            },
        };
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
