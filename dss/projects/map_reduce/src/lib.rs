use std::collections::HashMap;

mod master;
mod rpc;
mod worker;

struct FileLocation {
    host: String,
    path: String,
}

enum TaskType {
    Map,
    Reduce,
}

struct Task {
    is_allocated: bool,
    task_type: TaskType,
    task_files: HashMap<String, String>,
}

impl Task {
    fn new(task_type: TaskType, task_files: HashMap<String, String>) -> Self {
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
