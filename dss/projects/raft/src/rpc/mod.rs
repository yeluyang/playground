use std::{sync::Arc, thread, time::Duration};

extern crate grpcio;
use grpcio::{ChannelBuilder, EnvBuilder, Server, ServerBuilder};

mod grpc;
use grpc::{PeerGrpcClient, PeerGrpcServer};

use crate::EndPoint;

#[derive(Clone)]
pub struct Config {
    ip: String,
    port: u16,
    logs: String,
    peers: Vec<EndPoint>,
}

pub struct PeerServer {
    config: Config,
    inner: Server,
    runner: PeerGrpcServer,
}

impl PeerServer {
    fn new(config: Config) -> Self {
        let runner = PeerGrpcServer::new(&config);
        let inner = ServerBuilder::new(Arc::new(EnvBuilder::new().build()))
            .register_service(grpc::create_peer_grpc(runner.clone()))
            .bind(config.ip.clone(), config.port)
            .build()
            .unwrap();
        Self {
            config,
            inner,
            runner,
        }
    }

    fn run(&mut self, time: Option<Duration>) {
        self.inner.start();
        self.runner.run();
        match time {
            Some(time) => thread::sleep(time),
            None => loop {
                thread::sleep(Duration::new(u64::MAX, u32::MAX))
            },
        };
    }
}

#[derive(Clone)]
pub struct PeerClient {
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
