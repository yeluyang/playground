#![feature(get_mut_unchecked)]
#![feature(option_unwrap_none)]
#![feature(option_expect_none)]

use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    sync::atomic::{AtomicBool, Ordering},
};

#[macro_use]
extern crate log;

#[macro_use]
extern crate maplit;

mod error;
mod master;
mod rpc;
#[cfg(test)]
pub(crate) mod test;
mod worker;

pub use error::{Error, Result};

#[derive(Debug)]
enum Task {
    Map {
        allocated: AtomicBool,
        path_on_hosts: HashMap<String, String>,
    },
    Reduce {
        allocated: AtomicBool,
        internal_key: String,
        paths_with_hosts: Vec<(String, String)>,
    },
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Map {
                    allocated,
                    path_on_hosts,
                } => format!(
                    "type=MAP, allocated={}, replicated in {} hosts",
                    allocated.load(Ordering::SeqCst),
                    path_on_hosts.len()
                ),
                Self::Reduce {
                    allocated,
                    internal_key,
                    paths_with_hosts,
                } => format!(
                    "type=REDUCE, allocated={}, key={}, responding {} files",
                    allocated.load(Ordering::SeqCst),
                    internal_key,
                    paths_with_hosts.len()
                ),
            }
        )
    }
}

impl Clone for Task {
    fn clone(&self) -> Self {
        match self {
            Self::Map {
                allocated,
                path_on_hosts,
            } => Self::Map {
                allocated: AtomicBool::new(allocated.load(Ordering::SeqCst)),
                path_on_hosts: path_on_hosts.clone(),
            },
            Self::Reduce {
                allocated,
                internal_key,
                paths_with_hosts,
            } => Self::Reduce {
                allocated: AtomicBool::new(allocated.load(Ordering::SeqCst)),
                internal_key: internal_key.clone(),
                paths_with_hosts: paths_with_hosts.clone(),
            },
        }
    }
}

impl Task {
    fn new(internal_key: Option<String>, files: Vec<(String, String)>) -> Self {
        match internal_key {
            None => {
                debug!("creating MAP task with {} replicated files", files.len());
                let mut path_on_hosts: HashMap<String, String> = HashMap::new();
                for (host, path) in files {
                    trace!("inserting replicated file={{ {}:{} }}", host, path);
                    path_on_hosts.insert(host, path).unwrap_none();
                }

                Self::Map {
                    allocated: AtomicBool::new(false),
                    path_on_hosts,
                }
            }
            Some(internal_key) => {
                debug!(
                    "creating REDUCE task with {} files of key={}",
                    files.len(),
                    internal_key
                );

                Self::Reduce {
                    allocated: AtomicBool::new(false),
                    internal_key,
                    paths_with_hosts: files,
                }
            }
        }
    }

    // TODO: merge two task in one
    fn concat(&mut self, other: &Self) {
        unimplemented!()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Job {
    Map {
        reducers: usize,
        host: String,
        path: String,
    },
    Reduce {
        output_dir: String,
        internal_key: String,
        paths: Vec<(String, String)>,
    },
}

#[derive(Debug, Clone, PartialEq)]
enum JobResult {
    Map {
        host: String,
        paths: HashMap<String, String>,
    },
    Reduce {
        internal_key: String,
        host: String,
        path: String,
    },
}
