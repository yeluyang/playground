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
        debug!("JobGet invoked: request={{ {:?} }}", req);

        let task_type = grpc::crate_task_type_from(&req.get_task_type());
        let job = self.master.alloc_job(task_type, req.get_host());

        trace!("get job={:?}", job);
        let rsp = match job {
            None => JobGetResponse::new(),
            Some(job) => {
                let (task_type, host, path) = match job {
                    Job::Map { host, path } => (grpc::TaskType::MAP, host, path),
                    Job::Reduce { host, path } => (grpc::TaskType::REDUCE, host, path),
                };

                let mut rsp = JobGetResponse::new();
                rsp.task_type = task_type;

                let mut file_loc = FileLocation::new();
                file_loc.host = host;
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
        debug!(
            "creating server on {}:{} with {} tasks",
            host,
            port,
            tasks.len()
        );
        Ok(Self {
            inner: grpcio::ServerBuilder::new(Arc::new(EnvBuilder::new().build()))
                .register_service(create_master_grpc(MasterGrpcServer::new(tasks)))
                .bind(host, port)
                .build()?,
        })
    }

    pub fn run(&mut self, time: Option<Duration>) -> Result<()> {
        debug!("server running: time={:?}", time);

        self.inner.start();
        match time {
            Some(time) => thread::sleep(time),
            None => loop {},
        };

        trace!("server exit");
        Ok(())
    }
}

pub(crate) struct MasterClient {
    inner: MasterGrpcClient,
}

impl MasterClient {
    pub(crate) fn new(host: &str, port: u16) -> Self {
        debug!("creating client connected to {}:{}", host, port);
        Self {
            inner: MasterGrpcClient::new(
                ChannelBuilder::new(Arc::new(EnvBuilder::new().build()))
                    .connect(&format!("{}:{}", host, port)),
            ),
        }
    }

    pub fn get_job(&self, host: String, task_type: Option<crate::TaskType>) -> Result<Option<Job>> {
        debug!(
            "getting job: expected={{ type={:?}, host={} }}",
            task_type, host
        );

        let mut req = JobGetRequest::new();
        req.host = host;
        req.task_type = grpc::grpc_task_type_from(&task_type);

        let mut rsp = self.inner.job_get(&req)?;
        trace!("got response={:?}", rsp);

        let file = rsp.take_file_location();
        if file.path.is_empty() {
            Ok(None)
        } else {
            match rsp.get_task_type() {
                grpc::TaskType::MAP => Ok(Some(Job::Map {
                    host: file.host,
                    path: file.path,
                })),
                grpc::TaskType::REDUCE => Ok(Some(Job::Reduce {
                    host: file.host,
                    path: file.path,
                })),
                grpc::TaskType::ANY => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::{
        test::{self, ServeTime},
        Task,
    };

    #[test]
    fn test_job_get() {
        self::test::setup_logger();
        struct TestCase {
            host: String,
            port: u16,
            tasks: Vec<Task>,
            serve_time: ServeTime,
        }
        let cases = [TestCase {
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
            serve_time: ServeTime::new(4, 1),
        }];

        for c in &cases {
            {
                let client = MasterClient::new(&c.host, c.port);

                let mut server = MasterServer::new(&c.host, c.port, c.tasks.clone()).unwrap();
                let serve_time = c.serve_time.serve;
                thread::spawn(move || server.run(Some(serve_time)).unwrap());
                c.serve_time.wait_init();

                for t in &c.tasks {
                    for (host, path) in &t.task_files {
                        let job = client
                            .get_job(host.clone(), Some(t.task_type.clone()))
                            .expect("get Err from `get_job`, expect Ok")
                            .expect("get None from `get_job`, expect Some");
                        match t.task_type {
                            crate::TaskType::Map => assert!(matches!(job, Job::Map{..})),
                            crate::TaskType::Reduce => assert!(matches!(job, Job::Reduce{..})),
                        };
                        let (job_host, job_path) = &match job {
                            Job::Map { host, path } => (host, path),
                            Job::Reduce { host, path } => (host, path),
                        };
                        assert_eq!(job_host, host);
                        assert_eq!(job_path, path);
                    }
                }

                c.serve_time.wait_exit();
            }
        }
    }
}
