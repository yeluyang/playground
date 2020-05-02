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

const METHOD_MASTER_GRPC_JOB_GET: ::grpcio::Method<super::map_reduce::JobGetRequest, super::map_reduce::JobGetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MasterGRPC/JobGet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MASTER_GRPC_JOB_DONE: ::grpcio::Method<super::map_reduce::JobDoneRequest, super::map_reduce::JobDoneResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MasterGRPC/JobDone",
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

    pub fn job_get_opt(&self, req: &super::map_reduce::JobGetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::map_reduce::JobGetResponse> {
        self.client.unary_call(&METHOD_MASTER_GRPC_JOB_GET, req, opt)
    }

    pub fn job_get(&self, req: &super::map_reduce::JobGetRequest) -> ::grpcio::Result<super::map_reduce::JobGetResponse> {
        self.job_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn job_get_async_opt(&self, req: &super::map_reduce::JobGetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::map_reduce::JobGetResponse>> {
        self.client.unary_call_async(&METHOD_MASTER_GRPC_JOB_GET, req, opt)
    }

    pub fn job_get_async(&self, req: &super::map_reduce::JobGetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::map_reduce::JobGetResponse>> {
        self.job_get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn job_done_opt(&self, req: &super::map_reduce::JobDoneRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::map_reduce::JobDoneResponse> {
        self.client.unary_call(&METHOD_MASTER_GRPC_JOB_DONE, req, opt)
    }

    pub fn job_done(&self, req: &super::map_reduce::JobDoneRequest) -> ::grpcio::Result<super::map_reduce::JobDoneResponse> {
        self.job_done_opt(req, ::grpcio::CallOption::default())
    }

    pub fn job_done_async_opt(&self, req: &super::map_reduce::JobDoneRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::map_reduce::JobDoneResponse>> {
        self.client.unary_call_async(&METHOD_MASTER_GRPC_JOB_DONE, req, opt)
    }

    pub fn job_done_async(&self, req: &super::map_reduce::JobDoneRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::map_reduce::JobDoneResponse>> {
        self.job_done_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait MasterGrpc {
    fn job_get(&mut self, ctx: ::grpcio::RpcContext, req: super::map_reduce::JobGetRequest, sink: ::grpcio::UnarySink<super::map_reduce::JobGetResponse>);
    fn job_done(&mut self, ctx: ::grpcio::RpcContext, req: super::map_reduce::JobDoneRequest, sink: ::grpcio::UnarySink<super::map_reduce::JobDoneResponse>);
}

pub fn create_master_grpc<S: MasterGrpc + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MASTER_GRPC_JOB_GET, move |ctx, req, resp| {
        instance.job_get(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_MASTER_GRPC_JOB_DONE, move |ctx, req, resp| {
        instance.job_done(ctx, req, resp)
    });
    builder.build()
}
