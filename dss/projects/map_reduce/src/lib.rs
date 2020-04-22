#![feature(get_mut_unchecked)]
#![feature(option_unwrap_none)]

use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
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

#[derive(Debug, Clone, PartialEq)]
enum TaskType {
    Map,
    Reduce,
}

impl Display for TaskType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Map => "MAP",
                Self::Reduce => "REDUCE",
            }
        )
    }
}

#[derive(Debug)]
struct Task {
    is_allocated: AtomicBool,
    task_type: TaskType,
    task_files: HashMap<String, String>,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "allocated={:?}, type={}, files.num={}",
            self.is_allocated,
            self.task_type,
            self.task_files.len()
        )
    }
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
        debug!("create a {} task with {} files", task_type, files.len());

        let mut task_files = HashMap::new();
        for (host, path) in files {
            trace!("insert files {{ {}:{} }} into task", host, path);
            task_files.insert(host, path).unwrap_none();
        }

        Self {
            is_allocated: AtomicBool::new(false),

            task_type,
            task_files,
        }
    }
}

#[derive(Debug)]
enum Job {
    Map { host: String, path: String },
    Reduce { host: String, path: String },
}

impl<'a> Job {
    fn get_file_location(&'a self) -> (&'a str, &'a str) {
        match self {
            Job::Map { host, path } => (host, path),
            Job::Reduce { host, path } => (host, path),
        }
    }
}
