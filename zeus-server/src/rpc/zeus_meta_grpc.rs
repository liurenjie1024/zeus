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

const METHOD_ZEUS_META_SERVICE_GET_DB_SCHEMA: ::grpcio::Method<super::zeus_meta::GetSchemaRequest, super::zeus_meta::GetSchemaResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/ZeusMetaService/GetDBSchema",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ZeusMetaServiceClient {
    client: ::grpcio::Client,
}

impl ZeusMetaServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ZeusMetaServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_db_schema_opt(&self, req: &super::zeus_meta::GetSchemaRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::zeus_meta::GetSchemaResponse> {
        self.client.unary_call(&METHOD_ZEUS_META_SERVICE_GET_DB_SCHEMA, req, opt)
    }

    pub fn get_db_schema(&self, req: &super::zeus_meta::GetSchemaRequest) -> ::grpcio::Result<super::zeus_meta::GetSchemaResponse> {
        self.get_db_schema_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_db_schema_async_opt(&self, req: &super::zeus_meta::GetSchemaRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::zeus_meta::GetSchemaResponse>> {
        self.client.unary_call_async(&METHOD_ZEUS_META_SERVICE_GET_DB_SCHEMA, req, opt)
    }

    pub fn get_db_schema_async(&self, req: &super::zeus_meta::GetSchemaRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::zeus_meta::GetSchemaResponse>> {
        self.get_db_schema_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ZeusMetaService {
    fn get_db_schema(&self, ctx: ::grpcio::RpcContext, req: super::zeus_meta::GetSchemaRequest, sink: ::grpcio::UnarySink<super::zeus_meta::GetSchemaResponse>);
}

pub fn create_zeus_meta_service<S: ZeusMetaService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ZEUS_META_SERVICE_GET_DB_SCHEMA, move |ctx, req, resp| {
        instance.get_db_schema(ctx, req, resp)
    });
    builder.build()
}
