use crate::{rpc::MasterClient, Job};

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

    fn get_job(&mut self) {
        self.task = self.client.get_job(self.host.clone());
    }

    fn run(&mut self) {
        unimplemented!()
    }
}
