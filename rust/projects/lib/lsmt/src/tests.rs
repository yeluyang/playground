use std::{fs, path::Path};

extern crate serde;
use serde::{Deserialize, Serialize};

extern crate serde_json;

use crate::LogStructuredMergeTree;

#[test]
fn test_lsmt() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Case {
        id: usize,
        data: Vec<String>,
    }
    let cases = [
        Case {
            id: 0,
            data: vec![],
        },
        Case {
            id: 1,
            data: vec!["hello".to_owned(), "world".to_owned()],
        },
        Case {
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
            let bs = serde_json::to_vec(case).unwrap();
            lsmter.append(bs.as_slice()).unwrap();
        }
    }

    let mut lsmter = LogStructuredMergeTree::open(&file_path).unwrap();
    for case in &cases {
        let bs = serde_json::to_vec(case).unwrap();
        let data = lsmter.read_next().unwrap().unwrap();
        assert_eq!(data.len(), bs.len());
        assert_eq!(data, bs);
        let c: Case = serde_json::from_slice(data.as_slice()).unwrap();
        assert_eq!(&c, case);
    }
}
