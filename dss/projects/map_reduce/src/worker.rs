use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fs,
    fs::OpenOptions,
    hash::Hasher,
    io::{BufWriter, Read, Write},
    path::PathBuf,
    time::Duration,
};

extern crate uuid;
use uuid::Uuid;

use crate::{rpc::MasterClient, Job, Result};

pub trait Map {
    fn mapping(&self, input: String) -> HashMap<String, String>;
}

pub trait Reduce {
    fn reducing(&self, inputs: Vec<String>) -> String;
}

#[derive(Debug, Clone)]
pub struct WorkerConfig {
    master_host: String,
    master_port: u16,

    flush_interval: Duration,
    map_output_dir: String,
}

pub struct Worker<M: Map, R: Reduce> {
    config: WorkerConfig,

    host: String,
    job: Option<Job>,

    master_client: MasterClient,

    mapper: M,
    reducer: R,
}

impl<M: Map, R: Reduce> Worker<M, R> {
    pub fn new(host: String, config: WorkerConfig, mapper: M, reducer: R) -> Self {
        debug!(
            "creating worker: on host={}, with config={:?}",
            host, config
        );

        let master_client = MasterClient::new(&config.master_host, config.master_port);

        Self {
            config,
            host,
            job: None,
            master_client,
            mapper,
            reducer,
        }
    }

    pub fn run(&mut self) {
        debug!("worker running");
        loop {
            self.get_job().unwrap();
            self.work();
        }
    }

    fn get_job(&mut self) -> Result<()> {
        debug!("getting job for host={}", self.host);

        self.job = self.master_client.get_job(self.host.clone())?;
        trace!("job requested from rpc: {:?}", self.job);

        Ok(())
    }

    fn work(&mut self) -> Result<()> {
        if self.job.is_none() {
            return Ok(());
        }
        let job = self.job.take().unwrap();

        match job {
            // TODO handling when host is not local
            Job::Map {
                reducers,
                host,
                path,
            } => {
                let mut input = String::new();
                if OpenOptions::new()
                    .read(true)
                    .open(path)?
                    .read_to_string(&mut input)?
                    != 0
                {
                    let dir =
                        PathBuf::from(&self.config.map_output_dir).join(Uuid::new_v4().to_string());
                    if !dir.exists() {
                        fs::create_dir_all(&dir)?;
                    }

                    for (k, v) in self.mapper.mapping(input) {
                        trace!("handling result from user mapping: {}:{}", k, v);

                        let region_id = {
                            let mut hasher = DefaultHasher::new();
                            hasher.write(k.as_bytes());
                            hasher.finish() as usize % reducers
                        };

                        let path = dir.join(format!("{}.txt", region_id));
                        let mut output = BufWriter::new(
                            OpenOptions::new().create(true).append(true).open(&path)?,
                        );

                        trace!("write result into file: {:?}", path);
                        output.write_all(format!("{} {}\n", k, v).as_bytes())?;
                    }
                };
            }
            Job::Reduce {
                output_dir,
                internal_key,
                paths,
            } => {
                let mut inputs = Vec::with_capacity(paths.len());
                // TODO handling when host is not local
                for (host, path) in &paths {
                    let mut input = String::new();
                    if OpenOptions::new()
                        .read(true)
                        .open(path)?
                        .read_to_string(&mut input)?
                        != 0
                    {
                        inputs.push(input);
                    };
                }
                if !inputs.is_empty() {
                    self.reducer.reducing(inputs);
                };
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::{thread, time::Duration};

    use super::*;

    use crate::{
        rpc::{MasterServer, ServerConfig},
        test::{self, Dataset, ServeTimer, TestMapper, TestReducer},
        Task,
    };

    #[test]
    fn test_get_job() {
        self::test::setup_logger();

        struct TestCase {
            host: String,
            config: WorkerConfig,
            dataset: Dataset,
            serve_time: ServeTimer,
        }
        let cases = [TestCase {
            host: "172.20.25.184".to_owned(),
            config: WorkerConfig {
                master_host: "127.0.0.1".to_owned(),
                master_port: 10087,
                map_output_dir: "tmp/test/worker/test_get_job/map".to_owned(),
                flush_interval: Duration::from_secs(1),
            },
            dataset: Dataset::new(
                vec!["127.0.0.1".to_owned(), "172.20.25.184".to_owned()],
                2,
                2,
                2,
            ),
            serve_time: ServeTimer::new(4, 1),
        }];

        for c in &cases {
            {
                let mut worker = Worker::new(
                    c.host.clone(),
                    c.config.clone(),
                    TestMapper {},
                    TestReducer {},
                );

                let mut server = MasterServer::new(
                    ServerConfig {
                        host: c.config.master_host.clone(),
                        port: c.config.master_port,
                        reducers: 4,
                        output_dir: "tmp/test/worker/get_job/reduce".to_owned(),
                    },
                    c.dataset.tasks.clone(),
                )
                .unwrap();
                let serve_time = c.serve_time.serve;
                thread::spawn(move || server.run(Some(serve_time)));
                c.serve_time.wait_init();

                worker.get_job().unwrap();
                assert!(worker.job.is_some());

                let job = worker.job.unwrap();
                match job {
                    Job::Map {
                        reducers,
                        host,
                        path,
                    } => {
                        assert!(!path.is_empty());
                        assert_eq!(host, worker.host);
                    }
                    Job::Reduce {
                        output_dir,
                        internal_key,
                        paths,
                    } => {
                        assert!(!internal_key.is_empty());
                        for (host, path) in paths {
                            assert!(!path.is_empty());
                            assert_eq!(host, worker.host);
                        }
                    }
                }

                c.serve_time.wait_exit();
            }
        }
    }

    #[test]
    fn test_work() {
        self::test::setup_logger();

        struct TestCase {
            host: String,
            config: WorkerConfig,
            tasks: Vec<Task>,
            serve_time: ServeTimer,
        }
        let cases = [TestCase {
            host: "127.0.0.1".to_owned(),
            config: WorkerConfig {
                master_host: "127.0.0.1".to_owned(),
                master_port: 10087,
                map_output_dir: "tmp/test/worker/work/map".to_owned(),
                flush_interval: Duration::from_secs(1),
            },
            tasks: vec![
                Task::new(
                    None,
                    vec![(
                        "127.0.0.1".to_owned(),
                        "assets/test/pg-being_ernest.txt".to_owned(),
                    )],
                ),
                Task::new(
                    None,
                    vec![(
                        "127.0.0.1".to_owned(),
                        "assets/test/pg-metamorphosis.txt".to_owned(),
                    )],
                ),
            ],
            serve_time: ServeTimer::new(4, 1),
        }];

        for c in &cases {
            {
                let mut worker = Worker::new(
                    c.host.clone(),
                    c.config.clone(),
                    TestMapper {},
                    TestReducer {},
                );

                let mut server = MasterServer::new(
                    ServerConfig {
                        host: c.config.master_host.clone(),
                        port: c.config.master_port,
                        reducers: 4,
                        output_dir: "tmp/test/worker/work/reduce".to_owned(),
                    },
                    c.tasks.clone(),
                )
                .unwrap();
                let serve_time = c.serve_time.serve;
                thread::spawn(move || server.run(Some(serve_time)));
                c.serve_time.wait_init();

                for _ in 0..c.tasks.len() {
                    worker.get_job().unwrap();
                    assert!(worker.job.is_some());

                    worker.work().unwrap();
                }

                c.serve_time.wait_exit();
            }
        }
    }
}
