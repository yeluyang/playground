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

const METHOD_MASTER_GRPC_PING: ::grpcio::Method<super::map_reduce::PingRequest, super::map_reduce::PingResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MasterGRPC/Ping",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MASTER_GRPC_TASK_GET: ::grpcio::Method<super::map_reduce::TaskGetRequest, super::map_reduce::TaskGetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MasterGRPC/TaskGet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct MasterGrpcClient {
    client: ::grpcio::Client,
}

impl MasterGrpcClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        MasterGrpcClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn ping_opt(&self, req: &super::map_reduce::PingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::map_reduce::PingResponse> {
        self.client.unary_call(&METHOD_MASTER_GRPC_PING, req, opt)
    }

    pub fn ping(&self, req: &super::map_reduce::PingRequest) -> ::grpcio::Result<super::map_reduce::PingResponse> {
        self.ping_opt(req, ::grpcio::CallOption::default())
    }

    pub fn ping_async_opt(&self, req: &super::map_reduce::PingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::map_reduce::PingResponse>> {
        self.client.unary_call_async(&METHOD_MASTER_GRPC_PING, req, opt)
    }

    pub fn ping_async(&self, req: &super::map_reduce::PingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::map_reduce::PingResponse>> {
        self.ping_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn task_get_opt(&self, req: &super::map_reduce::TaskGetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::map_reduce::TaskGetResponse> {
        self.client.unary_call(&METHOD_MASTER_GRPC_TASK_GET, req, opt)
    }

    pub fn task_get(&self, req: &super::map_reduce::TaskGetRequest) -> ::grpcio::Result<super::map_reduce::TaskGetResponse> {
        self.task_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn task_get_async_opt(&self, req: &super::map_reduce::TaskGetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::map_reduce::TaskGetResponse>> {
        self.client.unary_call_async(&METHOD_MASTER_GRPC_TASK_GET, req, opt)
    }

    pub fn task_get_async(&self, req: &super::map_reduce::TaskGetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::map_reduce::TaskGetResponse>> {
        self.task_get_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait MasterGrpc {
    fn ping(&mut self, ctx: ::grpcio::RpcContext, req: super::map_reduce::PingRequest, sink: ::grpcio::UnarySink<super::map_reduce::PingResponse>);
    fn task_get(&mut self, ctx: ::grpcio::RpcContext, req: super::map_reduce::TaskGetRequest, sink: ::grpcio::UnarySink<super::map_reduce::TaskGetResponse>);
}

pub fn create_master_grpc<S: MasterGrpc + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MASTER_GRPC_PING, move |ctx, req, resp| {
        instance.ping(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_MASTER_GRPC_TASK_GET, move |ctx, req, resp| {
        instance.task_get(ctx, req, resp)
    });
    builder.build()
}
