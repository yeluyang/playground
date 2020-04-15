use std::sync::Arc;

extern crate grpcio;
use grpcio::{ChannelBuilder, EnvBuilder, RpcContext, UnarySink};

use crate::{master::Master, FileLocation, Job, Task};

mod grpc;
use grpc::{JobGetRequest, JobGetResponse, MasterGrpc, MasterGrpcClient, TaskType};

pub(crate) struct MasterServer {
    master: Master,
}

impl MasterServer {
    pub(crate) fn new<F: Fn()>(master: Master) -> Self {
        Self { master }
    }
}

impl MasterGrpc for MasterServer {
    fn job_get(&mut self, ctx: RpcContext, req: JobGetRequest, sink: UnarySink<JobGetResponse>) {
        let task_type = match req.get_task_type() {
            TaskType::ANY => None,
            TaskType::MAP => Some(crate::TaskType::Map),
            TaskType::REDUCE => Some(crate::TaskType::Reduce),
        };
        let task = self.master.alloc_job(task_type, req.get_host().to_owned());
        sink.success(JobGetResponse::new());
    }
}

pub(crate) struct MasterClient {
    client: MasterGrpcClient,
}

impl MasterClient {
    pub(crate) fn new(host: &str) -> Self {
        Self {
            client: MasterGrpcClient::new(
                ChannelBuilder::new(Arc::new(EnvBuilder::new().build())).connect(host),
            ),
        }
    }

    pub fn get_job(&self, host: String) -> Option<Job> {
        let mut req = JobGetRequest::new();
        req.host = host;

        let mut rsp = self.client.job_get(&req).unwrap();

        let file = rsp.take_file_location();
        if file.host != req.host || file.path.is_empty() {
            None
        } else {
            match rsp.get_task_type() {
                TaskType::MAP => Some(Job::Map(file.path)),
                TaskType::REDUCE => Some(Job::Reduce(file.path)),
                TaskType::ANY => unreachable!(),
            }
        }
    }
}
