use std::sync::Arc;

extern crate grpcio;
use grpcio::{ChannelBuilder, EnvBuilder, Server, ServerBuilder};

mod grpc;
use grpc::{NodeGrpcClient, NodeGrpcServer, PingRequest};

pub struct NodeServer {
    inner: NodeGrpcServer,
    runner: Server,
}

impl NodeServer {
    pub fn new(id: String, peers_host: Vec<(String, u16)>, ip: String, port: u16) -> Self {
        let inner = NodeGrpcServer::new(id, peers_host);
        let runner = ServerBuilder::new(Arc::new(EnvBuilder::new().build()))
            .register_service(grpc::create_node_grpc(inner.clone()))
            .bind(ip.clone(), port)
            .build()
            .unwrap();
        Self { inner, runner }
    }

    pub fn run(&mut self) {
        self.runner.start();
        self.inner.run();
    }
}

#[derive(Clone)]
pub struct NodeClient {
    client: NodeGrpcClient,
}

impl NodeClient {
    pub fn new(ip: String, port: u16) -> Self {
        Self {
            client: NodeGrpcClient::new(
                ChannelBuilder::new(Arc::new(EnvBuilder::new().build()))
                    .connect(format!("{}:{}", ip, port).as_str()),
            ),
        }
    }

    pub fn ping(&self, id: String) {
        let mut req = PingRequest::new();
        req.set_id(id);

        self.client.ping(&req);
    }
}
