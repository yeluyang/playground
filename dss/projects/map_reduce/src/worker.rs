use crate::{rpc::MasterClient, Job, Result, TaskType};

struct Worker {
    host: String,
    port: u16,
    task: Option<Job>,

    client: MasterClient,
}

impl Worker {
    fn new(host: String, port: u16) -> Self {
        debug!("creating worker on: host={}, port={}", host, port);

        let client = MasterClient::new(&host, port);

        Self {
            task: None,

            host,
            port,
            client,
        }
    }

    fn get_job(&mut self, task_type: Option<TaskType>) -> Result<()> {
        debug!("getting job: type={:?}", task_type);

        self.task = self.client.get_job(self.host.clone(), task_type)?;
        Ok(())
    }

    fn run(&mut self) {
        debug!("worker running");
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        struct TestCase {
            host: &'static str,
            port: u16,
        }
        let cases = &[TestCase {
            host: "127.0.0.1",
            port: 10087,
        }];

        for case in cases {
            let w = Worker::new(case.host.to_owned(), case.port);
        }
    }
}
