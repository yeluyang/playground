use crate::{rpc::MasterClient, Task};

struct Worker {
    host: String,
    task: Option<Task>,

    client: MasterClient,
}

impl Worker {
    fn new() -> Self {
        unimplemented!()
    }

    fn get_task(&mut self) {
        self.task = self.client.get_task(self.host.clone());
    }

    fn run(&mut self) {
        unimplemented!()
    }
}
