mod master;
mod rpc;
mod worker;

struct FileLocation {
    host: String,
    path: String,
}

enum TaskType {
    Any,
    Map,
    Reduce,
}

struct Task {
    task_type: TaskType,
    task_file: FileLocation,
}

impl Task {
    fn new(task_type: TaskType, host: String, path: String) -> Self {
        Self {
            task_type,
            task_file: FileLocation { host, path },
        }
    }
}
