// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

const METHOD_ZEUS_DATA_SERVICE_QUERY: ::grpcio::Method<super::zeus_data::QueryRequest, super::zeus_data::QueryResult> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/ZeusDataService/Query",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ZeusDataServiceClient {
    client: ::grpcio::Client,
}

impl ZeusDataServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ZeusDataServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn query_opt(&self, req: &super::zeus_data::QueryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::zeus_data::QueryResult> {
        self.client.unary_call(&METHOD_ZEUS_DATA_SERVICE_QUERY, req, opt)
    }

    pub fn query(&self, req: &super::zeus_data::QueryRequest) -> ::grpcio::Result<super::zeus_data::QueryResult> {
        self.query_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_async_opt(&self, req: &super::zeus_data::QueryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::zeus_data::QueryResult>> {
        self.client.unary_call_async(&METHOD_ZEUS_DATA_SERVICE_QUERY, req, opt)
    }

    pub fn query_async(&self, req: &super::zeus_data::QueryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::zeus_data::QueryResult>> {
        self.query_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ZeusDataService {
    fn query(&self, ctx: ::grpcio::RpcContext, req: super::zeus_data::QueryRequest, sink: ::grpcio::UnarySink<super::zeus_data::QueryResult>);
}

pub fn create_zeus_data_service<S: ZeusDataService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ZEUS_DATA_SERVICE_QUERY, move |ctx, req, resp| {
        instance.query(ctx, req, resp)
    });
    builder.build()
}
