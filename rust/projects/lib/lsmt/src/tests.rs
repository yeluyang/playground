use std::{collections::HashMap, fs, path::Path};

extern crate serde;
use serde::{Deserialize, Serialize};

use crate::{Config, LogEntryPointer, LogStructuredMergeTree, Record};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestRecord {
    pub id: String,
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
        self.id.clone()
    }
}

#[test]
fn test_lsmt_without_compact_and_merge() {
    let mut case_placeholder = TestRecord::default();
    let factor = 10;
    let keys_num = 10;
    let files_num = 4;
    let cfg = Config {
        lsmt_dir: String::from("tmp/test_lsmt/without_compact_and_merge"),
        file_size: keys_num * factor,
        compact_enable: false,
        merge_threshold: None,
    };

    let tmp_dir = Path::new(&cfg.lsmt_dir);
    if tmp_dir.exists() && tmp_dir.is_dir() {
        fs::remove_dir_all(tmp_dir).unwrap();
    }
    fs::create_dir_all(tmp_dir).unwrap();

    let mut value = vec![];

    {
        let mut lt = LogStructuredMergeTree::open(cfg.clone()).unwrap();
        for _ in 0..files_num {
            for v in 0..factor {
                value.push(format!("{}", v));
                case_placeholder.data = value.clone();
                for k in 0..keys_num {
                    case_placeholder.id = format!("key{}", k);
                    let p = lt.append(&case_placeholder).unwrap();
                    assert_eq!(&p.file_id, lt.fds.last().unwrap().header.ids.end());
                    assert_eq!(p.key, case_placeholder.key());

                    let c = lt.read_by_pointer::<TestRecord>(&p).unwrap().unwrap();
                    assert_eq!(c, case_placeholder);
                }
            }
        }
        assert_eq!(lt.fds.len(), files_num);
        for fd in &lt.fds {
            assert_eq!(fd.index.len(), keys_num);
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
    assert_eq!(index.len(), keys_num);

    for (key, pointers) in index.iter() {
        assert_eq!(pointers.len(), files_num * factor);
        for (i, p) in pointers.iter().enumerate() {
            let c = lt.read_by_pointer::<TestRecord>(p).unwrap().unwrap();
            assert_eq!(&c.key(), key);
            assert_eq!(c.data.as_slice(), &value[0..factor * (i / factor + 1)]);
        }
    }
}

#[test]
fn test_lsmt_with_compact_but_merge() {
    let mut case_placeholder = TestRecord::default();
    let factor = 10;
    let keys_num = 10;
    let files_num = 4;
    let cfg = Config {
        lsmt_dir: String::from("tmp/test_lsmt/with_compact_but_merge"),
        file_size: keys_num * factor,
        compact_enable: true,
        merge_threshold: None,
    };

    let tmp_dir = Path::new(&cfg.lsmt_dir);
    if tmp_dir.exists() && tmp_dir.is_dir() {
        fs::remove_dir_all(tmp_dir).unwrap();
    }
    fs::create_dir_all(tmp_dir).unwrap();

    let mut value = vec![];

    {
        let mut lt = LogStructuredMergeTree::open(cfg.clone()).unwrap();
        for _ in 0..files_num {
            for v in 0..factor {
                value.push(format!("{}", v));
                case_placeholder.data = value.clone();
                for k in 0..keys_num {
                    case_placeholder.id = format!("key{}", k);
                    let p = lt.append(&case_placeholder).unwrap();
                    assert_eq!(&p.file_id, lt.fds.last().unwrap().header.ids.end());
                    assert_eq!(p.key, case_placeholder.key());

                    let c = lt.read_by_pointer::<TestRecord>(&p).unwrap().unwrap();
                    assert_eq!(c, case_placeholder);
                }
            }
        }
        assert_eq!(lt.fds.len(), files_num);
        for fd in &lt.fds {
            assert_eq!(fd.index.len(), keys_num);
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
    assert_eq!(index.len(), keys_num);

    for (key, pointers) in index.iter() {
        assert_eq!(pointers.len(), files_num - 1 + factor);
        for (i, p) in pointers.iter().enumerate() {
            let c = lt.read_by_pointer::<TestRecord>(p).unwrap().unwrap();
            assert_eq!(&c.key(), key);
            if i >= files_num {
                assert_eq!(c.data.as_slice(), &value[0..factor * files_num]);
            } else {
                assert_eq!(c.data.as_slice(), &value[0..factor * (i + 1)]);
            }
        }
    }
}

#[test]
fn test_lsmt_with_compact_and_merge() {
    let mut case_placeholder = TestRecord::default();
    let factor = 10;
    let keys_num = 10;
    let files_num = 7;
    let cfg = Config {
        lsmt_dir: String::from("tmp/test_lsmt/with_compact_and_merge"),
        file_size: keys_num * factor,
        compact_enable: true,
        merge_threshold: Some(4),
    };
    let merged_files_num =
        (files_num / cfg.merge_threshold.unwrap()) * cfg.merge_threshold.unwrap();
    let files_final_num = cfg.merge_threshold.unwrap();

    let tmp_dir = Path::new(&cfg.lsmt_dir);
    if tmp_dir.exists() && tmp_dir.is_dir() {
        fs::remove_dir_all(tmp_dir).unwrap();
    }
    fs::create_dir_all(tmp_dir).unwrap();

    let mut value = vec![];

    {
        let mut lt = LogStructuredMergeTree::open(cfg.clone()).unwrap();
        for _ in 0..files_num {
            for v in 0..factor {
                value.push(format!("{}", v));
                case_placeholder.data = value.clone();
                for k in 0..keys_num {
                    case_placeholder.id = format!("key{}", k);
                    let p = lt.append(&case_placeholder).unwrap();
                    assert_eq!(&p.file_id, lt.fds.last().unwrap().header.ids.end());
                    assert_eq!(p.key, case_placeholder.key());

                    let c = lt.read_by_pointer::<TestRecord>(&p).unwrap().unwrap();
                    assert_eq!(c, case_placeholder);
                }
            }
        }
        assert_eq!(lt.fds.len(), files_final_num);
        for fd in &lt.fds {
            assert_eq!(fd.index.len(), keys_num);
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
    assert_eq!(index.len(), keys_num);

    for (key, pointers) in index.iter() {
        assert_eq!(pointers.len(), files_final_num - 1 + factor);
        for (i, p) in pointers.iter().enumerate() {
            let c = lt.read_by_pointer::<TestRecord>(p).unwrap().unwrap();
            assert_eq!(&c.key(), key);
            if i < 1 {
                assert_eq!(c.data.as_slice(), &value[0..factor * merged_files_num]);
            } else if i < files_final_num {
                assert_eq!(
                    c.data.as_slice(),
                    &value[0..factor * (i + merged_files_num)]
                );
            } else {
                assert_eq!(c.data.as_slice(), &value[0..factor * files_num]);
            }
        }
    }
}
