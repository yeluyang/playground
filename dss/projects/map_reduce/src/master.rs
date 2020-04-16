use std::{collections::HashMap, rc::Rc};

use crate::{Job, Task, TaskType};

pub(crate) struct Master {
    map_tasks: HashMap<String, Vec<Rc<Task>>>,
    reduce_tasks: HashMap<String, Vec<Rc<Task>>>,
    tasks: Vec<Rc<Task>>,
}

impl Master {
    pub(crate) fn new(tasks: Vec<Task>) -> Self {
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

        if let Some(host_tasks) = tasks.get_mut(&host) {
            while let Some(mut task) = host_tasks.pop() {
                let job = if !task.is_allocated {
                    unsafe {
                        Rc::get_mut_unchecked(&mut task).is_allocated = true;
                    }
                    let job = match task.task_type {
                        TaskType::Map => Some(Job::Map(task.task_files[&host].clone())),
                        TaskType::Reduce => Some(Job::Reduce(task.task_files[&host].clone())),
                    };
                    self.tasks.push(task);

                    job
                } else {
                    None
                };
                if host_tasks.is_empty() {
                    tasks.remove(&host);
                }
                return job;
            }
        };

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        struct TestCase {
            tasks: Vec<Task>,
            map_tasks_num: usize,
            reduce_tasks_num: usize,
            alloc_map_tasks_num: usize,
            alloc_reduce_tasks_num: usize,
        }
        let cases = &[
            TestCase {
                tasks: Vec::new(),
                map_tasks_num: 0,
                reduce_tasks_num: 0,
                alloc_map_tasks_num: 0,
                alloc_reduce_tasks_num: 0,
            },
            TestCase {
                tasks: vec![
                    Task::new(
                        TaskType::Map,
                        vec![("127.0.0.1".to_owned(), "/path/to/map/file".to_owned())],
                    ),
                    Task::new(
                        TaskType::Reduce,
                        vec![("127.0.0.1".to_owned(), "/path/to/reduce/file".to_owned())],
                    ),
                ],
                map_tasks_num: 1,
                reduce_tasks_num: 1,
                alloc_map_tasks_num: 1,
                alloc_reduce_tasks_num: 1,
            },
        ];

        for c in cases {
            let mut m = Master::new(c.tasks.clone());
            assert_eq!(m.map_tasks.len(), c.map_tasks_num);
            assert_eq!(m.reduce_tasks.len(), c.reduce_tasks_num);

            for _ in 0..c.alloc_map_tasks_num {
                let job = m
                    .alloc_job(Some(TaskType::Map), "127.0.0.1".to_owned())
                    .unwrap();
                assert!(matches!(job, Job::Map(_)));
            }
            assert_eq!(m.map_tasks.len(), c.map_tasks_num - c.alloc_map_tasks_num);

            for _ in 0..c.alloc_reduce_tasks_num {
                let job = m
                    .alloc_job(Some(TaskType::Reduce), "127.0.0.1".to_owned())
                    .unwrap();
                assert!(matches!(job, Job::Reduce(_)));
            }
            assert_eq!(
                m.reduce_tasks.len(),
                c.reduce_tasks_num - c.alloc_reduce_tasks_num
            );

            assert_eq!(
                m.tasks.len(),
                c.alloc_map_tasks_num + c.alloc_reduce_tasks_num
            );
        }
    }
}
