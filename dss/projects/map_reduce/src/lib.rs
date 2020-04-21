#![feature(get_mut_unchecked)]
#![feature(option_unwrap_none)]
#![feature(matches_macro)]

use std::{
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
};

#[macro_use]
extern crate log;

mod error;
mod master;
mod rpc;
#[cfg(test)]
pub(crate) mod test;
mod worker;

pub use error::{Error, Result};

#[derive(Debug, Clone)]
enum TaskType {
    Map,
    Reduce,
}

#[derive(Debug)]
struct Task {
    is_allocated: AtomicBool,
    task_type: TaskType,
    task_files: HashMap<String, String>,
}

impl Clone for Task {
    fn clone(&self) -> Self {
        Self {
            is_allocated: AtomicBool::new(self.is_allocated.load(Ordering::SeqCst)),
            task_type: self.task_type.clone(),
            task_files: self.task_files.clone(),
        }
    }
}

impl Task {
    fn new(task_type: TaskType, files: Vec<(String, String)>) -> Self {
        let mut task_files = HashMap::new();
        for (host, path) in files {
            task_files.insert(host, path).unwrap_none();
        }
        Self {
            is_allocated: AtomicBool::new(false),

            task_type,
            task_files,
        }
    }
}

enum Job {
    Map { host: String, path: String },
    Reduce { host: String, path: String },
}
