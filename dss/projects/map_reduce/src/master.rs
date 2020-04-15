use std::{collections::HashMap, path::Path, rc::Rc};

use crate::{FileLocation, Job, Task, TaskType};

use super::rpc::MasterServer;

pub(crate) struct Master {
    map_tasks: HashMap<String, Vec<Rc<Task>>>,
    reduce_tasks: HashMap<String, Vec<Rc<Task>>>,
    tasks: Vec<Rc<Task>>,
}

impl Master {
    pub(crate) fn new<F>(tasks: Vec<Task>) -> Self {
        let mut m = Self {
            map_tasks: HashMap::new(),
            reduce_tasks: HashMap::new(),
            tasks: Vec::new(),
        };

        for task in tasks {
            let task = Rc::new(task);

            let specified_tasks = match task.task_type {
                TaskType::Map => &mut m.map_tasks,
                TaskType::Reduce => &mut m.reduce_tasks,
            };

            for (host, _) in &task.task_files {
                match specified_tasks.get_mut(host) {
                    None => {
                        specified_tasks.insert(host.clone(), vec![task.clone()]);
                    }
                    Some(tasks) => {
                        tasks.push(task.clone());
                    }
                };
            }
        }

        m
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
                    unsafe {
                        Rc::get_mut_unchecked(&mut task).is_allocated = true;
                    }
                    let job = match task.task_type {
                        TaskType::Map => Some(Job::Map(task.task_files[&host].clone())),
                        TaskType::Reduce => Some(Job::Reduce(task.task_files[&host].clone())),
                    };
                    self.tasks.push(task);
                    return job;
                };
            }
        };

        None
    }
}
