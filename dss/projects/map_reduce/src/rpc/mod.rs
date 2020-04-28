use std::{sync::Arc, thread, time::Duration};

extern crate grpcio;
use grpcio::{ChannelBuilder, EnvBuilder, RpcContext, UnarySink};

use crate::{master::Master, Job, Result, Task};

mod grpc;
use grpc::{
    crate_job_from, create_master_grpc, grpc_job_from, FileLocation, JobGetRequest, JobGetResponse,
    JobGetResponse_oneof_job, MasterGrpc, MasterGrpcClient,
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

        let job = self.master.alloc_job(req.get_host());

        trace!("job allocated from master storage: {:?}", job);
        let rsp = match job {
            None => JobGetResponse::new(),
            Some(job) => {
                let mut rsp = JobGetResponse::new();
                match grpc_job_from(job) {
                    JobGetResponse_oneof_job::map_job(map_job) => rsp.set_map_job(map_job),
                    JobGetResponse_oneof_job::reduce_job(reduce_job) => {
                        rsp.set_reduce_job(reduce_job)
                    }
                };
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

    pub fn run(&mut self, time: Option<Duration>) {
        debug!("server running: time={:?}", time);

        self.inner.start();
        match time {
            Some(time) => thread::sleep(time),
            None => loop {},
        };

        trace!("server exit");
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

    pub fn get_job(&self, host: String) -> Result<Option<Job>> {
        debug!("sending JobGetRequest to MasterServer from host={}", host);

        let mut req = JobGetRequest::new();
        req.host = host;

        let rsp = self.inner.job_get(&req)?;
        trace!("response from Masterserver: {:?}", rsp);

        Ok(if rsp.has_map_job() || rsp.has_reduce_job() {
            Some(crate_job_from(rsp.job.unwrap()))
        } else {
            None
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test::{self, Dataset, ServeTimer};

    #[test]
    fn test_job_get() {
        self::test::setup_logger();
        struct TestCase {
            host: String,
            port: u16,
            dataset: Dataset,
            serve_time: ServeTimer,
        }
        let cases = [TestCase {
            host: "127.0.0.1".to_owned(),
            port: 10086,
            dataset: Dataset::new(vec!["127.0.0.1".to_owned()], 4, 4, 1),
            serve_time: ServeTimer::new(4, 1),
        }];

        for c in &cases {
            {
                let client = MasterClient::new(&c.host, c.port);

                let mut server =
                    MasterServer::new(&c.host, c.port, c.dataset.tasks.clone()).unwrap();
                let serve_time = c.serve_time.serve;
                thread::spawn(move || server.run(Some(serve_time)));
                c.serve_time.wait_init();

                for t in &c.dataset.tasks {
                    let job = client
                        .get_job(c.host.clone())
                        .expect("get Err from `get_job`, expect Ok")
                        .expect("get None from `get_job`, expect Some");
                    match job {
                        Job::Map { host, path } => {
                            assert!(c.dataset.hosts.contains(&host));
                            assert!(!path.is_empty());
                        }
                        Job::Reduce { key, paths } => {
                            assert!(!key.is_empty());
                            assert!(!paths.is_empty());
                        }
                    };
                }

                c.serve_time.wait_exit();
            }
        }
    }
}
