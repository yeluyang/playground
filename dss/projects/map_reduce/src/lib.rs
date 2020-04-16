#![feature(get_mut_unchecked)]
#![feature(option_unwrap_none)]
#![feature(matches_macro)]

use std::collections::HashMap;

mod master;
mod rpc;
mod worker;

struct FileLocation {
    host: String,
    path: String,
}

#[derive(Debug, Clone)]
enum TaskType {
    Map,
    Reduce,
}

#[derive(Debug, Clone)]
struct Task {
    is_allocated: bool,
    task_type: TaskType,
    task_files: HashMap<String, String>,
}

impl Task {
    fn new(task_type: TaskType, files: Vec<(String, String)>) -> Self {
        let mut task_files = HashMap::new();
        for (host, path) in files {
            task_files.insert(host, path).unwrap_none();
        }
        Self {
            is_allocated: false,

            task_type,
            task_files,
        }
    }
}

enum Job {
    Map(String),
    Reduce(String),
}
