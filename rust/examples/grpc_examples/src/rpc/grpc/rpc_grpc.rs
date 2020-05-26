// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_NODE_GRPC_PING: ::grpcio::Method<super::rpc::PingRequest, super::rpc::PingResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/NodeGRPC/Ping",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct NodeGrpcClient {
    client: ::grpcio::Client,
}

impl NodeGrpcClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        NodeGrpcClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn ping_opt(&self, req: &super::rpc::PingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::PingResponse> {
        self.client.unary_call(&METHOD_NODE_GRPC_PING, req, opt)
    }

    pub fn ping(&self, req: &super::rpc::PingRequest) -> ::grpcio::Result<super::rpc::PingResponse> {
        self.ping_opt(req, ::grpcio::CallOption::default())
    }

    pub fn ping_async_opt(&self, req: &super::rpc::PingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::PingResponse>> {
        self.client.unary_call_async(&METHOD_NODE_GRPC_PING, req, opt)
    }

    pub fn ping_async(&self, req: &super::rpc::PingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::PingResponse>> {
        self.ping_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait NodeGrpc {
    fn ping(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::PingRequest, sink: ::grpcio::UnarySink<super::rpc::PingResponse>);
}

pub fn create_node_grpc<S: NodeGrpc + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_NODE_GRPC_PING, move |ctx, req, resp| {
        instance.ping(ctx, req, resp)
    });
    builder.build()
}
