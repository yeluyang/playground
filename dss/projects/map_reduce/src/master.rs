use std::{
    collections::HashMap,
    sync::{atomic::Ordering, Arc},
};

use crate::{Job, Task, TaskType};

#[derive(Clone)]
pub(crate) struct Master {
    map_tasks: HashMap<String, Vec<Arc<Task>>>,
    reduce_tasks: HashMap<String, Vec<Arc<Task>>>,
    allocated: Vec<Arc<Task>>,
}

impl Master {
    pub(crate) fn new(tasks: Vec<Task>) -> Self {
        debug!("creating master storage with {} tasks", tasks.len());

        let mut m = Self {
            map_tasks: HashMap::new(),
            reduce_tasks: HashMap::new(),
            allocated: Vec::new(),
        };

        for task in tasks {
            trace!("inserting task into master: task={{ {} }}", task);
            let task = Arc::new(task);

            let type_tasks = match task.task_type {
                TaskType::Map => &mut m.map_tasks,
                TaskType::Reduce => &mut m.reduce_tasks,
            };

            for (host, path) in &task.task_files {
                trace!(
                    "inserting replicated files of task: type={}, file={{ host={}, path={} }}",
                    task.task_type,
                    host,
                    path
                );
                match type_tasks.get_mut(host) {
                    None => {
                        type_tasks.insert(host.clone(), vec![task.clone()]);
                    }
                    Some(tasks) => {
                        tasks.push(task.clone());
                    }
                };
            }
        }

        trace!(
            "created master: {} map tasks, {} reduce tasks",
            m.map_tasks.len(),
            m.reduce_tasks.len()
        );

        m
    }

    pub fn alloc_job(&mut self, task_type: Option<TaskType>, host: &str) -> Option<Job> {
        debug!("allocating task: type={:?}, host={}", task_type, host);

        let (task_type, type_tasks) = match task_type {
            Some(task_type) => match task_type {
                TaskType::Map => (TaskType::Map, &mut self.map_tasks),
                TaskType::Reduce => (TaskType::Reduce, &mut self.reduce_tasks),
            },
            None => {
                trace!("type of task requested is not specified, now have: map_tasks.num={}, reduce_tasks.num={}", self.map_tasks.len(), self.reduce_tasks.len());
                if self.map_tasks.len() > self.reduce_tasks.len() {
                    (TaskType::Map, &mut self.map_tasks)
                } else {
                    (TaskType::Reduce, &mut self.reduce_tasks)
                }
            }
        };

        if let Some(host_tasks) = type_tasks.get_mut(host) {
            trace!("get {} tasks on {}", host_tasks.len(), host);
            while let Some(task) = host_tasks.pop() {
                trace!("handling task={{ {} }}", task);
                let job = if !task
                    .is_allocated
                    .compare_and_swap(false, true, Ordering::SeqCst)
                {
                    let job = match task.task_type {
                        TaskType::Map => Some(Job::Map {
                            host: host.to_owned(),
                            path: task.task_files[host].clone(),
                        }),
                        TaskType::Reduce => Some(Job::Reduce {
                            host: host.to_owned(),
                            path: task.task_files[host].clone(),
                        }),
                    };
                    trace!("allocating job={{ {:?} }}", job);
                    self.allocated.push(task);

                    job
                } else {
                    trace!("met allocated task or other thread took it before this thread");
                    continue;
                };
                if host_tasks.is_empty() {
                    trace!(
                        "no more {} tasks on host={}, remove its place from master",
                        task_type,
                        host
                    );
                    type_tasks.remove(host);
                }
                return job;
            }

            unreachable!();
        };

        // TODO: when no task of specified type on host
        // TODO: when no task of any type on host
        trace!("task not found on specified host");
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_alloc_job() {
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
                let job = m.alloc_job(Some(TaskType::Map), "127.0.0.1").unwrap();
                assert!(matches!(job, Job::Map{..}));
            }
            assert_eq!(m.map_tasks.len(), c.map_tasks_num - c.alloc_map_tasks_num);

            for _ in 0..c.alloc_reduce_tasks_num {
                let job = m.alloc_job(Some(TaskType::Reduce), "127.0.0.1").unwrap();
                assert!(matches!(job, Job::Reduce{..}));
            }
            assert_eq!(
                m.reduce_tasks.len(),
                c.reduce_tasks_num - c.alloc_reduce_tasks_num
            );

            assert_eq!(
                m.allocated.len(),
                c.alloc_map_tasks_num + c.alloc_reduce_tasks_num
            );
        }
    }
}
