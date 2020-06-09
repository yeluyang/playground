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

const METHOD_PEER_GRPC_VOTE: ::grpcio::Method<super::rpc::VoteRequest, super::rpc::VoteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/PeerGRPC/Vote",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PEER_GRPC_APPEND: ::grpcio::Method<super::rpc::AppendRequest, super::rpc::AppendResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/PeerGRPC/Append",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct PeerGrpcClient {
    client: ::grpcio::Client,
}

impl PeerGrpcClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        PeerGrpcClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn vote_opt(&self, req: &super::rpc::VoteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::VoteResponse> {
        self.client.unary_call(&METHOD_PEER_GRPC_VOTE, req, opt)
    }

    pub fn vote(&self, req: &super::rpc::VoteRequest) -> ::grpcio::Result<super::rpc::VoteResponse> {
        self.vote_opt(req, ::grpcio::CallOption::default())
    }

    pub fn vote_async_opt(&self, req: &super::rpc::VoteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::VoteResponse>> {
        self.client.unary_call_async(&METHOD_PEER_GRPC_VOTE, req, opt)
    }

    pub fn vote_async(&self, req: &super::rpc::VoteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::VoteResponse>> {
        self.vote_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn append_opt(&self, req: &super::rpc::AppendRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AppendResponse> {
        self.client.unary_call(&METHOD_PEER_GRPC_APPEND, req, opt)
    }

    pub fn append(&self, req: &super::rpc::AppendRequest) -> ::grpcio::Result<super::rpc::AppendResponse> {
        self.append_opt(req, ::grpcio::CallOption::default())
    }

    pub fn append_async_opt(&self, req: &super::rpc::AppendRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AppendResponse>> {
        self.client.unary_call_async(&METHOD_PEER_GRPC_APPEND, req, opt)
    }

    pub fn append_async(&self, req: &super::rpc::AppendRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AppendResponse>> {
        self.append_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait PeerGrpc {
    fn vote(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::VoteRequest, sink: ::grpcio::UnarySink<super::rpc::VoteResponse>);
    fn append(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AppendRequest, sink: ::grpcio::UnarySink<super::rpc::AppendResponse>);
}

pub fn create_peer_grpc<S: PeerGrpc + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PEER_GRPC_VOTE, move |ctx, req, resp| {
        instance.vote(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_PEER_GRPC_APPEND, move |ctx, req, resp| {
        instance.append(ctx, req, resp)
    });
    builder.build()
}
