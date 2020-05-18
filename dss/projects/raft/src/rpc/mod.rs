use std::{sync::Arc, thread, time::Duration};

extern crate grpcio;
use grpcio::{ChannelBuilder, EnvBuilder, Server, ServerBuilder};

mod grpc;
use grpc::{PeerGrpcClient, PeerGrpcServer};

use crate::EndPoint;

#[derive(Clone)]
pub(crate) struct Config {
    ip: String,
    port: u16,
    logs: String,
    peers: Vec<EndPoint>,
}

struct PeerServer {
    config: Config,
    inner: Server,
}

impl PeerServer {
    fn new(config: Config) -> Self {
        let inner = ServerBuilder::new(Arc::new(EnvBuilder::new().build()))
            .register_service(grpc::create_peer_grpc(PeerGrpcServer::new(&config)))
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

#[derive(Clone)]
pub(crate) struct PeerClient {
    inner: PeerGrpcClient,
}

impl PeerClient {
    pub fn connect(host: &EndPoint) -> Self {
        Self {
            inner: PeerGrpcClient::new(
                ChannelBuilder::new(Arc::new(EnvBuilder::new().build()))
                    .connect(host.to_string().as_str()),
            ),
        }
    }
}
