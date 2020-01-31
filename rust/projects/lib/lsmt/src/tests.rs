use std::{fs, path::Path};

extern crate serde;
use serde::{Deserialize, Serialize};

extern crate serde_json;

use crate::{LogStructuredMergeTree, Record};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TestRecord {
    id: usize,
    data: Vec<String>,
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
    fn get_entry_key(&self) -> String {
        format!("{}", self.id)
    }
}

#[test]
fn test_lsmt_io() {
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

    let tmp_dir = Path::new("tmp");
    if !tmp_dir.exists() {
        fs::create_dir(tmp_dir).unwrap();
    }
    let file_path = tmp_dir.join("tmp.wal");
    {
        let mut lsmter = LogStructuredMergeTree::create(&file_path).unwrap();
        for case in &cases {
            lsmter.append(case).unwrap();
        }
    }

    let mut lsmter = LogStructuredMergeTree::open(&file_path).unwrap();
    for case in &cases {
        let c = lsmter.read_next::<TestRecord>().unwrap().unwrap();
        assert_eq!(&c, case);
    }

    for (i, case) in cases.iter().enumerate() {
        let c = lsmter.read_by_seek::<TestRecord>(i + 1).unwrap().unwrap();
        assert_eq!(&c, case);
    }
}
