use std::{sync::Arc, thread, time::Duration};

extern crate grpcio;
use grpcio::{ChannelBuilder, EnvBuilder, RpcContext, UnarySink};

use crate::{master::Master, Job, Result, Task};

mod grpc;
use grpc::{
    create_master_grpc, FileLocation, JobGetRequest, JobGetResponse, MasterGrpc, MasterGrpcClient,
};

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
        debug!("JobGet invoked: request={:?}", req);

        let task_type = grpc::crate_task_type_from(req.get_task_type());
        let job = self.master.alloc_job(task_type, req.get_host().to_owned());

        let rsp = match job {
            None => JobGetResponse::new(),
            Some(job) => {
                let (task_type, path) = match job {
                    Job::Map(path) => (grpc::TaskType::MAP, path),
                    Job::Reduce(path) => (grpc::TaskType::REDUCE, path),
                };

                let mut rsp = JobGetResponse::new();
                rsp.task_type = task_type;

                let mut file_loc = FileLocation::new();
                file_loc.host = req.get_host().to_owned();
                file_loc.path = path;
                rsp.set_file_location(file_loc);

                rsp
            }
        };

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

    pub fn run(&mut self, time: Option<Duration>) -> Result<()> {
        self.inner.start();
        match time {
            Some(time) => thread::sleep(time),
            None => loop {},
        };
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

    pub fn get_job(&self, host: String, task_type: Option<crate::TaskType>) -> Result<Option<Job>> {
        let mut req = JobGetRequest::new();
        req.host = host;
        req.task_type = grpc::grpc_task_type_from(task_type);

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

    use crate::{test, Task};

    #[test]
    fn test_rpc() {
        self::test::init();
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
            let client = MasterClient::new(&format!("{}:{}", c.host, c.port));

            let mut server = MasterServer::new(&c.host, c.port, c.tasks.clone()).unwrap();
            thread::spawn(move || server.run(Some(Duration::from_secs(60))).unwrap());
            client
                .get_job("127.0.0.1".to_owned(), Some(crate::TaskType::Map))
                .unwrap()
                .unwrap();
            thread::sleep(Duration::from_secs(4));
        }
    }
}
