use crate::{rpc::MasterClient, Job, Result};

struct Worker {
    host: String,
    task: Option<Job>,

    client: MasterClient,
}

impl Worker {
    fn new(host: String) -> Self {
        let client = MasterClient::new(&host);
        Self {
            task: None,

            host,
            client,
        }
    }

    fn get_job(&mut self) -> Result<()> {
        self.task = self.client.get_job(self.host.clone())?;
        Ok(())
    }

    fn run(&mut self) {
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
        }
        let cases = &[TestCase { host: "127.0.0.1" }];

        for case in cases {
            let w = Worker::new(case.host.to_owned());
        }
    }
}
