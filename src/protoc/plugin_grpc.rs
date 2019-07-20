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

const METHOD_PLUGIN_GET_JOBS: ::grpcio::Method<super::plugin::Empty, super::plugin::Job> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/proto.Plugin/GetJobs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PLUGIN_EXECUTE_JOB: ::grpcio::Method<super::plugin::Job, super::plugin::JobResult> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/proto.Plugin/ExecuteJob",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct PluginClient {
    client: ::grpcio::Client,
}

impl PluginClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        PluginClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_jobs_opt(&self, req: &super::plugin::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::plugin::Job>> {
        self.client.server_streaming(&METHOD_PLUGIN_GET_JOBS, req, opt)
    }

    pub fn get_jobs(&self, req: &super::plugin::Empty) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::plugin::Job>> {
        self.get_jobs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn execute_job_opt(&self, req: &super::plugin::Job, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::plugin::JobResult> {
        self.client.unary_call(&METHOD_PLUGIN_EXECUTE_JOB, req, opt)
    }

    pub fn execute_job(&self, req: &super::plugin::Job) -> ::grpcio::Result<super::plugin::JobResult> {
        self.execute_job_opt(req, ::grpcio::CallOption::default())
    }

    pub fn execute_job_async_opt(&self, req: &super::plugin::Job, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::plugin::JobResult>> {
        self.client.unary_call_async(&METHOD_PLUGIN_EXECUTE_JOB, req, opt)
    }

    pub fn execute_job_async(&self, req: &super::plugin::Job) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::plugin::JobResult>> {
        self.execute_job_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Plugin {
    fn get_jobs(&mut self, ctx: ::grpcio::RpcContext, req: super::plugin::Empty, sink: ::grpcio::ServerStreamingSink<super::plugin::Job>);
    fn execute_job(&mut self, ctx: ::grpcio::RpcContext, req: super::plugin::Job, sink: ::grpcio::UnarySink<super::plugin::JobResult>);
}

pub fn create_plugin<S: Plugin + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_PLUGIN_GET_JOBS, move |ctx, req, resp| {
        instance.get_jobs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PLUGIN_EXECUTE_JOB, move |ctx, req, resp| {
        instance.execute_job(ctx, req, resp)
    });
    builder.build()
}
