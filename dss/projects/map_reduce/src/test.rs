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
        let mut tmp = HashMap::new();
        for word in input.split_whitespace() {
            if let Some(count) = tmp.get_mut(word) {
                *count += 1usize;
            } else {
                tmp.insert(word, 1usize);
            }
        }

        let mut m = HashMap::new();
        for (word, count) in tmp {
            m.insert(word.to_owned(), count.to_string());
        }

        m
    }
}

pub(crate) struct TestReducer {}

impl Reduce for TestReducer {
    fn reducing(&self, inputs: Vec<String>) -> String {
        let mut tmp = HashMap::new();
        for input in &inputs {
            for line in input.split('\n') {
                let (word, count) = line.split_at(line.rfind(' ').expect("format mismatch"));
                let count = count.parse::<usize>().expect("expect integer");
                if let Some(c) = tmp.get_mut(word) {
                    *c += count;
                } else {
                    tmp.insert(word, count);
                }
            }
        }

        let mut result = String::new();
        for (word, count) in tmp {
            result.push_str(&format!("{} {}\n", word, count));
        }
        result
    }
}
