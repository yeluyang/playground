use crate::{FileLocation, Task, TaskType};

use std::collections::HashMap;

use super::rpc::MasterServer;

pub(crate) struct Master {
    map_tasks: HashMap<String, Vec<String>>,
    reduce_tasks: HashMap<String, Vec<String>>,
}

impl Master {
    pub(crate) fn new() -> Self {
        unimplemented!()
    }

    pub fn alloc_task(&mut self, task_type: TaskType, host: String) -> Option<Task> {
        let tasks = match task_type {
            TaskType::Map => &mut self.map_tasks,
            TaskType::Reduce => &mut self.reduce_tasks,
            TaskType::Any => {
                if self.map_tasks.len() > self.reduce_tasks.len() {
                    &mut self.map_tasks
                } else {
                    &mut self.reduce_tasks
                }
            }
        };

        if let Some(paths) = tasks.get_mut(&host) {
            if let Some(path) = paths.pop() {
                return Some(Task::new(task_type, host, path));
            }
        }

        for (host, paths) in tasks {
            if let Some(path) = paths.pop() {
                return Some(Task::new(task_type, host.clone(), path));
            }
        }

        None
    }
}
