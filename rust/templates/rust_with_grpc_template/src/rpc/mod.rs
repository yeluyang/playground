use std::sync::Arc;

extern crate grpcio;
use grpcio::{ChannelBuilder, EnvBuilder, Server, ServerBuilder};

use crate::EndPoint;

mod grpc;
use grpc::{create_node_grpc, NodeGrpcClient, NodeGrpcServer};

pub struct NodeServer {
    inner: Server,
    runner: NodeGrpcServer,
}

impl NodeServer {
    pub fn new(host: EndPoint) -> Self {
        let runner = NodeGrpcServer::new();
        Self {
            inner: ServerBuilder::new(Arc::new(EnvBuilder::new().build()))
                .register_service(grpc::create_node_grpc(runner.clone()))
                .bind(host.ip.clone(), host.port)
                .build()
                .unwrap(),
            runner,
        }
    }

    pub fn run(&mut self) {
        self.inner.start();
        self.runner.run();
    }
}

pub struct NodeClient {
    inner: NodeGrpcClient,
}

impl NodeClient {
    pub fn new(host: EndPoint) -> Self {
        Self {
            inner: NodeGrpcClient::new(
                ChannelBuilder::new(Arc::new(EnvBuilder::new().build()))
                    .connect(host.to_string().as_str()),
            ),
        }
    }
}
