use std::{collections::HashMap, path::Path};

use crate::{FileLocation, Job, Task, TaskType};

use super::rpc::MasterServer;

pub(crate) struct Master {
    map_tasks: HashMap<String, Vec<Task>>,
    reduce_tasks: HashMap<String, Vec<Task>>,
}

impl Master {
    pub(crate) fn new<F>(inputs: Vec<String>, partitioner: F) -> Self
    where
        F: Fn(),
    {
        unimplemented!()
    }

    pub fn alloc_job(&mut self, task_type: Option<TaskType>, host: String) -> Option<Job> {
        let tasks = match task_type {
            Some(task_type) => match task_type {
                TaskType::Map => &mut self.map_tasks,
                TaskType::Reduce => &mut self.reduce_tasks,
            },
            None => {
                if self.map_tasks.len() > self.reduce_tasks.len() {
                    &mut self.map_tasks
                } else {
                    &mut self.reduce_tasks
                }
            }
        };

        if let Some(tasks) = tasks.get_mut(&host) {
            while let Some(mut task) = tasks.pop() {
                if !task.is_allocated {
                    task.is_allocated = true;
                    match task.task_type {
                        TaskType::Map => return Some(Job::Map(task.task_files[&host].clone())),
                        TaskType::Reduce => {
                            return Some(Job::Reduce(task.task_files[&host].clone()))
                        }
                    };
                };
            }
        };

        None
    }
}
