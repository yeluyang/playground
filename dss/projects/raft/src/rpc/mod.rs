use std::{
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    thread,
};

extern crate futures;
use futures::Future;

extern crate grpcio;
use grpcio::{ChannelBuilder, EnvBuilder, MessageReader, Server, ServerBuilder};

mod grpc;
use grpc::{PeerGrpcClient, PeerGrpcServer, VoteRequest, VoteResponse};

use crate::{EndPoint};

#[derive(Clone)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub logs: String,
    pub peers: Vec<(String, u16)>,
}

pub struct PeerServer {
    config: Config,
    inner: Server,
    runner: PeerGrpcServer,
}

impl PeerServer {
    pub fn new(config: Config) -> Self {
        let host = EndPoint::from((config.ip.clone(), config.port));
        let peers = EndPoint::from_hosts(&config.peers);
        let runner = PeerGrpcServer::new(host, &config.logs, peers);

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

    pub fn run(&mut self) {
        self.inner.start();
        self.runner.run();
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

    pub fn heart_beat(&self) {
        unimplemented!()
    }

    pub fn request_vote_async(&self, ch: Sender<(bool, usize)>) {
        let req = VoteRequest::new();
        let grpc_ch = self.inner.vote_async(&req).unwrap();
        thread::spawn(move || {
            let rsp = grpc_ch.wait().unwrap();
            ch.send((rsp.get_granted(), rsp.get_term() as usize));
        });
    }
}
