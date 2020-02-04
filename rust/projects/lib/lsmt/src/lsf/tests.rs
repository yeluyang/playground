use std::{fs, ops::RangeInclusive, path::Path};

extern crate serde;
use serde::{Deserialize, Serialize};

extern crate serde_json;

use super::{
    entry::{LogEntryPointer, LogFileHeader, Record},
    ls_file::LogStructuredFile,
};

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
    fn key(&self) -> String {
        format!("{}", self.id)
    }
}

#[test]
fn test_lsf_io() {
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
    let mut file_path = String::default();
    {
        let mut ls_fd = LogStructuredFile::create(
            &tmp_dir,
            LogFileHeader::new(RangeInclusive::new(0, 0), false),
        )
        .unwrap();
        file_path = ls_fd.path();
        assert_eq!(ls_fd.index.len(), 0);
        for (i, case) in cases.iter().enumerate() {
            assert_eq!(ls_fd.index.len(), i);
            ls_fd.append(case).unwrap();
            assert_eq!(ls_fd.index.len(), i + 1);
        }
        assert_eq!(ls_fd.index.len(), cases.len());
    }

    let mut ls_fd = LogStructuredFile::open(&file_path).unwrap();
    assert_eq!(ls_fd.index.len(), cases.len());

    for case in &cases {
        let c = ls_fd.pop::<TestRecord>().unwrap().unwrap();
        assert_eq!(&c, case);
    }

    for case in &cases {
        let c = ls_fd
            .read_by_pointer::<TestRecord>(&LogEntryPointer::new(0, case.key()))
            .unwrap()
            .unwrap();
        assert_eq!(&c, case);
    }
}
