use std::{cmp, collections::HashMap, fs::OpenOptions, result, thread, time::Duration};

extern crate env_logger;
use env_logger::{Builder, Env};

use crate::{
    worker::{Map, Reduce},
    Error, Task,
};

pub(crate) static MAP_OUTPUT_DIR: &str = "tmp/path/to/map/files";
pub(crate) static REDUCE_OUTPUT_DIR: &str = "tmp/path/to/reduce/files";

static INIT: std::sync::Once = std::sync::Once::new();
pub(crate) fn setup_logger() {
    INIT.call_once(|| {
        Builder::from_env(Env::default().default_filter_or("trace"))
            .is_test(true)
            .init();
    })
}

pub(crate) struct Dataset {
    pub(crate) tasks: Vec<Task>,
    pub(crate) hosts: Vec<String>,
    pub(crate) map_tasks_num: usize,
    pub(crate) replicated_num: usize,
    pub(crate) keys_max: usize,
}

impl Dataset {
    pub(crate) fn new(
        hosts: Vec<String>,
        map_tasks_num: usize,
        keys_max: usize,
        replicated_num: usize,
    ) -> Self {
        let mut dataset = Self {
            hosts,
            map_tasks_num,
            replicated_num,
            keys_max,
            tasks: Vec::new(),
        };

        // setup map dataset
        {
            let mut host_index = 0usize;
            for i in 0..dataset.map_tasks_num {
                let mut files: Vec<(String, String)> = Vec::with_capacity(dataset.replicated_num);
                for _ in 0..cmp::min(dataset.replicated_num, dataset.hosts.len()) {
                    files.push((
                        dataset.hosts[host_index].clone(),
                        format!("{}/{}", MAP_OUTPUT_DIR, i),
                    ));
                    host_index = (host_index + 1) % dataset.hosts.len();
                }
                dataset.tasks.push(Task::new(None, files));
            }
        }

        // setup reduce dataset
        {
            let mut host_index = 0usize;
            for i in 0..dataset.keys_max {
                let mut files: Vec<(String, String)> = Vec::with_capacity(dataset.replicated_num);
                for _ in 0..cmp::min(dataset.replicated_num, dataset.hosts.len()) {
                    files.push((
                        dataset.hosts[host_index].clone(),
                        format!("{}/key_{}", REDUCE_OUTPUT_DIR, i),
                    ));
                    host_index = (host_index + 1) % dataset.hosts.len();
                }
                dataset
                    .tasks
                    .push(Task::new(Some(format!("key_{}", i)), files));
            }
        }

        dataset
    }
}

pub(crate) struct ServeTimer {
    pub(crate) serve: Duration,
    wait_init: Duration,
}

impl ServeTimer {
    pub(crate) fn new(serve_time: u64, wait_init_time: u64) -> Self {
        assert!(serve_time > wait_init_time);
        Self {
            serve: Duration::from_secs(serve_time),
            wait_init: Duration::from_secs(wait_init_time),
        }
    }

    pub fn wait_init(&self) {
        debug!("wait master server init: {:?}", self.wait_init);
        thread::sleep(self.wait_init);
    }

    pub fn wait_exit(&self) {
        let wait_exit = self.serve - self.wait_init;
        debug!("wait master server exit: {:?}", wait_exit);
        thread::sleep(wait_exit);
    }
}

pub(crate) struct TestMapper {}

impl Map for TestMapper {
    fn mapping(&self, input: String) -> HashMap<String, String> {
        let mut m = HashMap::new();

        m.insert(input.len().to_string(), input.len().to_string());

        m
    }
}

pub(crate) struct TestReducer {}

impl Reduce for TestReducer {
    fn reducing(&self, inputs: Vec<String>) -> String {
        let mut count = 0usize;
        for input in inputs {
            let len = input.parse::<usize>().expect("expected usize");
            count += len;
        }

        count.to_string()
    }
}
