use std::sync::Arc;

extern crate grpcio;
use grpcio::{ChannelBuilder, EnvBuilder, RpcContext, UnarySink};

use crate::{master::Master, Job, Result};

mod grpc;
use grpc::{
    create_master_grpc, JobGetRequest, JobGetResponse, MasterGrpc, MasterGrpcClient, TaskType,
};

#[derive(Clone)]
pub(crate) struct MasterServer {
    master: Master,
}

impl MasterServer {
    pub(crate) fn new(master: Master) -> Self {
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

    pub fn get_job(&self, host: String) -> Result<Option<Job>> {
        let mut req = JobGetRequest::new();
        req.host = host;

        let mut rsp = self.client.job_get(&req)?;

        let file = rsp.take_file_location();
        if file.host != req.host || file.path.is_empty() {
            Ok(None)
        } else {
            match rsp.get_task_type() {
                TaskType::MAP => Ok(Some(Job::Map(file.path))),
                TaskType::REDUCE => Ok(Some(Job::Reduce(file.path))),
                TaskType::ANY => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(test)]
    mod server {
        use crate::Task;

        use super::*;

        #[test]
        fn test_new() {
            struct TestCase {
                host: String,
                port: u16,
                master: Master,
            }
            let cases = &[TestCase {
                host: "127.0.0.1".to_owned(),
                port: 10086,
                master: Master::new(vec![Task::new(
                    crate::TaskType::Map,
                    vec![("127.0.0.1".to_owned(), "/path/to/map/file".to_owned())],
                )]),
            }];
            for c in cases {
                let server = grpcio::ServerBuilder::new(Arc::new(EnvBuilder::new().build()))
                    .register_service(create_master_grpc(MasterServer::new(c.master.clone())))
                    .bind(c.host.clone(), c.port)
                    .build()
                    .unwrap();
            }
        }
    }

    #[cfg(test)]
    mod client {
        use super::*;

        #[test]
        fn test_new() {
            struct TestCase {
                host: &'static str,
            }
            let cases = &[TestCase { host: "127.0.0.1" }];

            for case in cases {
                let client = MasterClient::new(case.host);
            }
        }
    }
}
