use std::{
    sync::Arc,
    time::{Duration, Instant},
};

extern crate grpcio;
use grpcio::{ChannelBuilder, EnvBuilder, RpcContext, UnarySink};

extern crate signal;
use signal::{trap::Trap, Signal};

use crate::{master::Master, Job, Result, Task};

mod grpc;
use grpc::{create_master_grpc, JobGetRequest, JobGetResponse, MasterGrpc, MasterGrpcClient};

#[derive(Clone)]
struct MasterGrpcServer {
    master: Master,
}

impl MasterGrpcServer {
    pub(crate) fn new(tasks: Vec<Task>) -> Self {
        Self {
            master: Master::new(tasks),
        }
    }
}

impl MasterGrpc for MasterGrpcServer {
    fn job_get(&mut self, ctx: RpcContext, req: JobGetRequest, sink: UnarySink<JobGetResponse>) {
        let task_type = match req.get_task_type() {
            grpc::TaskType::ANY => None,
            grpc::TaskType::MAP => Some(crate::TaskType::Map),
            grpc::TaskType::REDUCE => Some(crate::TaskType::Reduce),
        };
        let task = self.master.alloc_job(task_type, req.get_host().to_owned());
        let rsp = JobGetResponse::new();
        sink.success(rsp);
    }
}

pub(crate) struct MasterServer {
    inner: grpcio::Server,
}

impl MasterServer {
    pub(crate) fn new(host: &str, port: u16, tasks: Vec<Task>) -> Result<Self> {
        Ok(Self {
            inner: grpcio::ServerBuilder::new(Arc::new(EnvBuilder::new().build()))
                .register_service(create_master_grpc(MasterGrpcServer::new(tasks)))
                .bind(host, port)
                .build()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut trap = Trap::trap(&[
            Signal::SIGINT,
            Signal::SIGABRT,
            Signal::SIGKILL,
            Signal::SIGTERM,
        ]);
        while let Some(_) = trap.next() {
            println!("exit");
            self.inner.shutdown();
            self.inner.cancel_all_calls();
            break;
        }
        Ok(())
    }
}

pub(crate) struct MasterClient {
    inner: MasterGrpcClient,
}

impl MasterClient {
    pub(crate) fn new(host: &str) -> Self {
        Self {
            inner: MasterGrpcClient::new(
                ChannelBuilder::new(Arc::new(EnvBuilder::new().build())).connect(host),
            ),
        }
    }

    pub fn get_job(&self, host: String) -> Result<Option<Job>> {
        let mut req = JobGetRequest::new();
        req.host = host;

        let mut rsp = self.inner.job_get(&req)?;

        let file = rsp.take_file_location();
        if file.host != req.host || file.path.is_empty() {
            Ok(None)
        } else {
            match rsp.get_task_type() {
                grpc::TaskType::MAP => Ok(Some(Job::Map(file.path))),
                grpc::TaskType::REDUCE => Ok(Some(Job::Reduce(file.path))),
                grpc::TaskType::ANY => unreachable!(),
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
                tasks: Vec<Task>,
            }
            let cases = &[TestCase {
                host: "127.0.0.1".to_owned(),
                port: 10086,
                tasks: vec![
                    Task::new(
                        crate::TaskType::Map,
                        vec![("127.0.0.1".to_owned(), "/path/to/map/file".to_owned())],
                    ),
                    Task::new(
                        crate::TaskType::Reduce,
                        vec![("127.0.0.1".to_owned(), "/path/to/reduce/file".to_owned())],
                    ),
                ],
            }];
            for c in cases {
                let mut server = MasterServer::new(&c.host, c.port, c.tasks.clone()).unwrap();
                server.run();
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
