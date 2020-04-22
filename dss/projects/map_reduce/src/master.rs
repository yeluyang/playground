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

    pub fn alloc_job(&mut self, host: &str) -> Option<Job> {
        debug!("allocating task: host={}", host);

        let (task_type, type_tasks) = if self.map_tasks.len() > self.reduce_tasks.len() {
            (TaskType::Map, &mut self.map_tasks)
        } else {
            (TaskType::Reduce, &mut self.reduce_tasks)
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

    use crate::test::Dataset;

    #[test]
    fn test_alloc_job() {
        struct TestCase {
            dataset: Dataset,
        }
        let cases = &[
            TestCase {
                dataset: Dataset::new(vec![], 0, 0, 0),
            },
            TestCase {
                dataset: Dataset::new(vec!["127.0.0.1".to_owned()], 4, 4, 1),
            },
        ];

        for c in cases {
            let mut m = Master::new(c.dataset.tasks.clone());

            let mut map_count = 0usize;
            let mut reduce_count = 0usize;
            for _ in 0..c.dataset.tasks.len() {
                let job = m.alloc_job("127.0.0.1").unwrap();
                match job {
                    Job::Map { .. } => map_count += 1,
                    Job::Reduce { .. } => reduce_count += 1,
                };
                let (host, path) = job.get_file_location();
                assert_eq!(host, "127.0.0.1");
                assert!(!path.is_empty());
            }
            assert_eq!(map_count, c.dataset.map_tasks_num);
            assert_eq!(reduce_count, c.dataset.reduce_tasks_num);
            assert!(m.map_tasks.is_empty());
            assert!(m.reduce_tasks.is_empty());
            assert_eq!(m.allocated.len(), c.dataset.tasks.len());
        }
    }
}
