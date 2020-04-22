use crate::{rpc::MasterClient, Job, Result, TaskType};

struct Worker {
    host: String,
    job: Option<Job>,

    master_host: String,
    master_port: u16,
    master_client: MasterClient,
}

impl Worker {
    fn new(host: String, master_host: String, master_port: u16) -> Self {
        debug!(
            "creating worker connected to {}:{}",
            master_host, master_port
        );

        let master_client = MasterClient::new(&master_host, master_port);

        Self {
            host,
            job: None,

            master_host,
            master_port,
            master_client,
        }
    }

    fn get_job(&mut self, task_type: Option<TaskType>) -> Result<()> {
        debug!("getting job: type={:?}", task_type);

        self.job = self.master_client.get_job(self.host.clone(), task_type)?;
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
        test::{self, DataSet, ServeTime},
    };

    #[test]
    fn test_new() {
        self::test::setup_logger();

        struct TestCase {
            host: String,
            master_host: String,
            master_port: u16,
            dataset: DataSet,
            serve_time: ServeTime,
        }
        let cases = [TestCase {
            host: "127.0.0.1".to_owned(),
            master_host: "127.0.0.1".to_owned(),
            master_port: 10086,
            dataset: DataSet::new(vec!["127.0.0.1".to_owned()], 4, 4, 1),
            serve_time: ServeTime::new(4, 1),
        }];

        for c in &cases {
            {
                let mut worker = Worker::new(c.host.clone(), c.master_host.clone(), c.master_port);

                let mut server =
                    MasterServer::new(&c.master_host, c.master_port, c.dataset.tasks.clone())
                        .unwrap();
                let serve_time = c.serve_time.serve;
                thread::spawn(move || server.run(Some(serve_time)).unwrap());
                c.serve_time.wait_init();

                worker.get_job(None).unwrap();

                c.serve_time.wait_exit();
            }
        }
    }
}
