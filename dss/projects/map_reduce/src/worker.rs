use crate::{rpc::MasterClient, Job, Result};

struct Worker {
    host: String,
    job: Option<Job>,

    master_client: MasterClient,
}

impl Worker {
    fn new(host: String, master_host: &str, master_port: u16) -> Self {
        debug!(
            "creating worker connected to {}:{}",
            master_host, master_port
        );

        let master_client = MasterClient::new(master_host, master_port);

        Self {
            host,
            job: None,

            master_client,
        }
    }

    fn get_job(&mut self) -> Result<()> {
        debug!("getting job for host={}", self.host);

        self.job = self.master_client.get_job(self.host.clone())?;
        trace!("got job: job={:?}", self.job);

        Ok(())
    }

    fn run(&mut self) {
        debug!("worker running");
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use std::thread;

    use super::*;

    use crate::{
        rpc::MasterServer,
        test::{self, Dataset, ServeTime},
    };

    #[test]
    fn test_get_job() {
        self::test::setup_logger();

        struct TestCase {
            host: String,
            master_host: &'static str,
            master_port: u16,
            dataset: Dataset,
            serve_time: ServeTime,
        }
        let cases = [TestCase {
            host: "172.20.25.184".to_owned(),
            master_host: "127.0.0.1",
            master_port: 10087,
            dataset: Dataset::new(
                vec!["127.0.0.1".to_owned(), "172.20.25.184".to_owned()],
                2,
                2,
                2,
            ),
            serve_time: ServeTime::new(4, 1),
        }];

        for c in &cases {
            {
                let mut worker = Worker::new(c.host.clone(), c.master_host, c.master_port);

                let mut server =
                    MasterServer::new(&c.master_host, c.master_port, c.dataset.tasks.clone())
                        .unwrap();
                let serve_time = c.serve_time.serve;
                thread::spawn(move || server.run(Some(serve_time)).unwrap());
                c.serve_time.wait_init();

                worker.get_job().unwrap();
                assert!(worker.job.is_some());

                let job = worker.job.unwrap();
                let (host, path) = job.get_file_location();
                assert!(!path.is_empty());
                assert_eq!(host, &worker.host);

                c.serve_time.wait_exit();
            }
        }
    }
}
