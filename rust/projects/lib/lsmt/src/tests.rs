use std::{collections::HashMap, fs, path::Path};

extern crate serde;
use serde::{Deserialize, Serialize};

use crate::{Config, LogEntryPointer, LogStructuredMergeTree, Record};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestRecord {
    pub id: usize,
    pub data: Vec<String>,
}

impl From<Vec<u8>> for TestRecord {
    fn from(data: Vec<u8>) -> Self {
        serde_json::from_slice(data.as_slice()).expect("failed to get TestRecord from bytes")
    }
}

impl Record for TestRecord {
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("failed to ser TestRecord to bytes")
    }
    fn key(&self) -> String {
        format!("{}", self.id)
    }
}

#[test]
fn test_lsmt() {
    let cases = [
        TestRecord {
            id: 0,
            data: vec![],
        },
        TestRecord {
            id: 1,
            data: vec!["hello".to_owned(), "world".to_owned()],
        },
        TestRecord {
            id: 2,
            data: vec!["end".to_owned()],
        },
    ];
    let cfg = Config {
        lsmt_dir: String::from("tmp/test_lsmt"),
        file_size: 100,
        merge_threshold: 2,
    };

    let tmp_dir = Path::new(&cfg.lsmt_dir);
    if tmp_dir.exists() && tmp_dir.is_dir() {
        fs::remove_dir_all(tmp_dir).unwrap();
    }
    fs::create_dir(tmp_dir).unwrap();

    let iterations = cfg.file_size * cfg.merge_threshold * 3 / cases.len();

    {
        let mut lt = LogStructuredMergeTree::open(cfg.clone()).unwrap();
        for _ in 0..iterations {
            for case in cases.iter() {
                let p = lt.append(case).unwrap();
                assert_eq!(&p.file_id, lt.fds.last().unwrap().header.ids.end());
                assert_eq!(p.key, case.key());

                let c = lt.read_by_pointer::<TestRecord>(&p).unwrap().unwrap();
                assert_eq!(&c, case);
            }
        }
        for fd in &lt.fds {
            assert_eq!(fd.index.len(), cases.len());
        }
    }

    let mut lt = LogStructuredMergeTree::open(cfg.clone()).unwrap();
    let mut index: HashMap<String, Vec<LogEntryPointer>> = HashMap::new();
    while let Some(p) = lt.pop().unwrap() {
        assert!(lt.fds[lt.fd_cursor].header.ids.contains(&p.file_id));
        match index.get_mut(&p.key) {
            Some(ps) => ps.push(p),
            None => {
                index.insert(p.key.clone(), vec![p]);
            }
        };
    }
    assert_eq!(index.len(), cases.len());

    let mut cs = HashMap::new();
    for c in &cases {
        cs.insert(c.key(), c.clone());
    }
    for (key, pointers) in index.iter() {
        for p in pointers {
            let c = lt.read_by_pointer::<TestRecord>(p).unwrap().unwrap();
            assert_eq!(&c.key(), key);
            assert_eq!(c, cs[&c.key()]);
        }
    }
}
