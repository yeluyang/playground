use std::{sync::Arc, thread, time::Duration};

extern crate uuid;

extern crate grpcio;
use grpcio::{ChannelBuilder, EnvBuilder, RpcContext, UnarySink};

use crate::{
    master::{Master, MasterConfig},
    Job, JobResult, Result, Task,
};

mod grpc;
use grpc::{
    crate_job_from, crate_job_result_from, create_master_grpc, grpc_job_from, grpc_job_result_from,
    FileLocation, JobDoneRequest, JobDoneRequest_oneof_result, JobDoneResponse, JobGetRequest,
    JobGetResponse, JobGetResponse_oneof_job, MasterGrpc, MasterGrpcClient,
};

#[derive(Clone)]
struct MasterGrpcServer {
    master: Master,
}

impl MasterGrpcServer {
    pub(crate) fn new(master: Master) -> Self {
        Self { master }
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

    fn job_done(&mut self, ctx: RpcContext, req: JobDoneRequest, sink: UnarySink<JobDoneResponse>) {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub reducers: usize,
    pub output_dir: String,
}

pub(crate) struct MasterServer {
    config: ServerConfig,
    inner: grpcio::Server,
}

impl MasterServer {
    pub(crate) fn new(config: ServerConfig, tasks: Vec<Task>) -> Result<Self> {
        debug!(
            "creating server on {}:{} with {} tasks",
            config.host,
            config.port,
            tasks.len()
        );

        let inner = grpcio::ServerBuilder::new(Arc::new(EnvBuilder::new().build()))
            .register_service(create_master_grpc(MasterGrpcServer::new(Master::new(
                MasterConfig {
                    reducers: config.reducers,
                    output_dir: config.output_dir.clone(),
                },
                tasks,
            ))))
            .bind(config.host.clone(), config.port)
            .build()?;

        Ok(Self { config, inner })
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

    pub fn done_job(&self, job_result: JobResult) -> Result<()> {
        let mut req = JobDoneRequest::new();
        match grpc_job_result_from(job_result) {
            JobDoneRequest_oneof_result::map_result(map_result) => req.set_map_result(map_result),
            JobDoneRequest_oneof_result::reduce_result(reduce_result) => {
                req.set_reduce_result(reduce_result)
            }
        };

        self.inner.job_done(&req)?;
        Ok(())
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
            config: ServerConfig,
            dataset: Dataset,
            serve_time: ServeTimer,
        }
        let cases = [TestCase {
            config: ServerConfig {
                host: "127.0.0.1".to_owned(),
                port: 10086,
                reducers: 4,
                output_dir: format!("tmp/test/rpc/job_get/reducer"),
            },
            dataset: Dataset::new(vec!["127.0.0.1".to_owned()], 4, 4, 1),
            serve_time: ServeTimer::new(4, 1),
        }];

        for c in &cases {
            {
                let client = MasterClient::new(&c.config.host, c.config.port);

                let mut server =
                    MasterServer::new(c.config.clone(), c.dataset.tasks.clone()).unwrap();
                let serve_time = c.serve_time.serve;
                thread::spawn(move || server.run(Some(serve_time)));
                c.serve_time.wait_init();

                for t in &c.dataset.tasks {
                    let job = client
                        .get_job(c.config.host.clone())
                        .expect("get Err from `get_job`, expect Ok")
                        .expect("get None from `get_job`, expect Some");
                    match job {
                        Job::Map {
                            reducers,
                            host,
                            path,
                        } => {
                            assert_eq!(reducers, c.config.reducers);
                            assert!(c.dataset.hosts.contains(&host));
                            assert!(!path.is_empty());
                        }
                        Job::Reduce {
                            output_dir,
                            internal_key,
                            paths,
                        } => {
                            assert!(!internal_key.is_empty());
                            assert!(!paths.is_empty());
                        }
                    };
                }
                assert!(client
                    .get_job(c.config.host.clone())
                    .expect("get Err from `get_job`, expect Ok")
                    .is_none());

                c.serve_time.wait_exit();
            }
        }
    }
}
