use std::{
    collections::HashMap,
    sync::{atomic::Ordering, Arc},
};

use crate::{Job, Task};

#[derive(Debug, Clone)]
pub(crate) struct Master {
    map_tasks: HashMap<String, Vec<Arc<Task>>>,
    // TODO: should hold only one task for every key
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

            match task.as_ref() {
                Task::Map { path_on_hosts, .. } => {
                    for host in path_on_hosts.keys() {
                        match m.map_tasks.get_mut(host) {
                            None => {
                                m.map_tasks.insert(host.clone(), vec![task.clone()]);
                            }
                            Some(tasks) => {
                                tasks.push(task.clone());
                            }
                        };
                    }
                }
                Task::Reduce { key, .. } => {
                    match m.reduce_tasks.get_mut(key) {
                        None => {
                            m.reduce_tasks.insert(key.clone(), vec![task.clone()]);
                        }
                        Some(tasks) => {
                            // TODO use Task::concat to merge two task in one
                            tasks.push(task.clone());
                        }
                    };
                }
            };
        }

        trace!("created successfully: master={:?}", m);

        m
    }

    pub fn alloc_job(&mut self, host: &str) -> Option<Job> {
        if !self.map_tasks.is_empty() {
            if let Some(host_tasks) = self.map_tasks.get_mut(host) {
                debug!(
                    "allocating MAP task on host={} which has {} tasks",
                    host,
                    host_tasks.len()
                );
                while let Some(task) = host_tasks.pop() {
                    trace!("found task={{ {} }}", task);
                    if let Task::Map {
                        allocated,
                        path_on_hosts,
                    } = task.as_ref()
                    {
                        let job = if !allocated.compare_and_swap(false, true, Ordering::SeqCst) {
                            let job = Some(Job::Map {
                                host: host.to_owned(),
                                path: path_on_hosts[host].clone(),
                            });
                            trace!("allocating job={{ {:?} }}", job);
                            self.allocated.push(task);

                            job
                        } else {
                            trace!("met allocated task or other thread took it before this thread");
                            continue;
                        };
                        if host_tasks.is_empty() {
                            trace!(
                                "no more MAP tasks on host={}, remove its place from master",
                                host
                            );
                            self.map_tasks.remove(host);
                        }
                        return job;
                    } else {
                        panic!("mismatch type task in master.map_tasks: task={}", task);
                    };
                }

                unreachable!();
            } else {
                // TODO when no MAP task on specified host
                unimplemented!();
            };
        } else {
            for (key, tasks) in &mut self.reduce_tasks {
                trace!(
                    "finding REDUCE task in {} tasks which has key={}",
                    tasks.len(),
                    key,
                );
                while let Some(task) = tasks.pop() {
                    trace!("found task={{ {} }}", task);
                    if let Task::Reduce {
                        allocated,
                        key,
                        paths_with_hosts,
                    } = task.as_ref()
                    {
                        if !allocated.compare_and_swap(false, true, Ordering::SeqCst) {
                            let job = Some(Job::Reduce {
                                key: key.clone(),
                                paths: paths_with_hosts.clone(),
                            });
                            trace!("allocating job={{ {:?} }}", job);
                            self.allocated.push(task);

                            return job;
                        } else {
                            trace!("met allocated task or other thread took it before this thread");
                            continue;
                        };
                    } else {
                        panic!("mismatch type task in master.reduce_tasks: task={}", task);
                    }
                }
            }
        };

        trace!("task not found");
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test::{self, Dataset};

    #[test]
    fn test_alloc_job() {
        test::setup_logger();

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
                    Job::Map { host, path } => {
                        map_count += 1;
                        assert_eq!(host, "127.0.0.1");
                        assert!(!path.is_empty());
                    }
                    Job::Reduce { key, paths } => reduce_count += 1,
                };
            }
            assert_eq!(map_count, c.dataset.map_tasks_num);
            assert_eq!(reduce_count, c.dataset.keys_max * c.dataset.replicated_num);
            assert!(m.map_tasks.is_empty());
            assert_eq!(m.allocated.len(), c.dataset.tasks.len());
        }
    }
}
