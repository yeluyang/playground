extern crate env_logger;
use env_logger::{Builder, Env};

use crate::{Task, TaskType};

static INIT: std::sync::Once = std::sync::Once::new();
pub(crate) fn setup_logger() {
    INIT.call_once(|| {
        Builder::from_env(Env::default().default_filter_or("trace"))
            .is_test(true)
            .init();
    })
}

fn setup_type_dataset(
    tasks: Vec<Task>,
    task_type: TaskType,
    hosts: &[&str],
    tasks_num: usize,
    replicated_num: usize,
) -> Vec<Task> {
    let mut host_index = 0usize;
    for i in 0..tasks_num {
        let mut files: Vec<(String, String)> = Vec::with_capacity(replicated_num);
        for _ in 0..replicated_num {
            files.push((
                hosts[host_index].to_owned(),
                match task_type {
                    TaskType::Map => format!("/path/to/map/files/{}", i),
                    TaskType::Reduce => format!("/path/to/reduce/files/{}", i),
                },
            ));
            host_index = (host_index + 1) % hosts.len();
        }
        Task::new(task_type.clone(), files);
    }

    tasks
}

pub(crate) fn setup_dataset(
    hosts: &[&str],
    map_tasks_num: usize,
    reduce_tasks_num: usize,
    replicated_num: usize,
) -> Vec<Task> {
    let mut tasks: Vec<Task> = Vec::new();

    tasks = setup_type_dataset(tasks, TaskType::Map, hosts, map_tasks_num, replicated_num);
    tasks = setup_type_dataset(
        tasks,
        TaskType::Reduce,
        hosts,
        reduce_tasks_num,
        replicated_num,
    );

    tasks
}
