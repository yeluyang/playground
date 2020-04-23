use std::{
    cmp,
    collections::HashMap,
    fs::OpenOptions,
    io::{BufRead, BufReader, BufWriter, Write},
    result, thread,
    time::Duration,
};

extern crate env_logger;
use env_logger::{Builder, Env};

use crate::{
    error::Error,
    worker::{Map, Reduce},
    Task, TaskType,
};

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
    pub(crate) reduce_tasks_num: usize,
    pub(crate) replicated_num: usize,
}

impl Dataset {
    pub(crate) fn new(
        hosts: Vec<String>,
        map_tasks_num: usize,
        reduce_tasks_num: usize,
        replicated_num: usize,
    ) -> Self {
        let mut dataset = Self {
            hosts,
            map_tasks_num,
            reduce_tasks_num,
            replicated_num,
            tasks: Vec::new(),
        };
        dataset.setup_type_dataset(TaskType::Map, map_tasks_num);
        dataset.setup_type_dataset(TaskType::Reduce, reduce_tasks_num);

        dataset
    }

    fn setup_type_dataset(&mut self, task_type: TaskType, tasks_num: usize) {
        let mut host_index = 0usize;
        for i in 0..tasks_num {
            let mut files: Vec<(String, String)> = Vec::with_capacity(self.replicated_num);
            for _ in 0..cmp::min(self.replicated_num, self.hosts.len()) {
                files.push((
                    self.hosts[host_index].clone(),
                    match task_type {
                        TaskType::Map => format!("/path/to/map/files/{}", i),
                        TaskType::Reduce => format!("/path/to/reduce/files/{}", i),
                    },
                ));
                host_index = (host_index + 1) % self.hosts.len();
            }
            self.tasks.push(Task::new(task_type.clone(), files));
        }
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
    type Error = Error;
    fn mapping(&self, path: String) -> result::Result<HashMap<String, String>, Self::Error> {
        let input = BufReader::new(OpenOptions::new().read(true).open(path)?);
        unimplemented!()
    }
}

pub(crate) struct TestReducer {}

impl Reduce for TestReducer {
    type Error = Error;
    fn reducing(&self, path: String) -> result::Result<String, Self::Error> {
        let input = BufReader::new(OpenOptions::new().read(true).open(path)?);
        unimplemented!()
    }
}
