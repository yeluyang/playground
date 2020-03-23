use std::{fs, ops::RangeInclusive, path::Path};

extern crate serde_json;

use super::{
    entry::{LogEntryPointer, LogFileHeader, Record},
    ls_file::LogStructuredFile,
};

use crate::tests::TestRecord;

#[test]
fn test_lsf() {
    let mut case_placeholder = TestRecord::default();
    let factor = 10;
    let keys_num = 10;

    let tmp_dir = Path::new("tmp/test_lsf");
    if tmp_dir.exists() {
        fs::remove_dir_all(tmp_dir).unwrap();
    }
    fs::create_dir(tmp_dir).unwrap();
    let file_path = String::from("tmp/test_lsf/0.wal");

    let mut value = vec![];

    {
        let mut ls_fd = LogStructuredFile::create(
            &tmp_dir,
            LogFileHeader::new(RangeInclusive::new(0, 0), false),
        )
        .unwrap();
        assert_eq!(ls_fd.index.len(), 0);
        for v in 0..factor {
            value.push(format!("{}", v));
            case_placeholder.data = value.clone();
            for k in 0..keys_num {
                if v == 0 {
                    assert_eq!(ls_fd.index.len(), k);
                } else {
                    assert_eq!(ls_fd.index.len(), keys_num);
                }
                case_placeholder.id = format!("key{}", k);
                ls_fd.append(&case_placeholder).unwrap();
                if v == 0 {
                    assert_eq!(ls_fd.index.len(), k + 1);
                } else {
                    assert_eq!(ls_fd.index.len(), keys_num);
                }
            }
        }
        assert_eq!(ls_fd.index.len(), keys_num);
    }

    let mut ls_fd = LogStructuredFile::open(&file_path).unwrap();
    assert_eq!(ls_fd.index.len(), keys_num);

    for v in 0..factor {
        for k in 0..keys_num {
            let c = ls_fd.pop::<TestRecord>().unwrap().unwrap();
            assert_eq!(c.key(), format!("key{}", k));
            assert_eq!(c.data.as_slice(), &value[0..=v]);
        }
    }

    for k in 0..keys_num {
        let c = ls_fd
            .read_by_pointer::<TestRecord>(&LogEntryPointer::new(0, format!("key{}", k)))
            .unwrap()
            .unwrap();
        assert_eq!(c.data, value);
    }

    ls_fd.compact().unwrap();
    ls_fd.fd.seek_segment(0).unwrap();

    for k in 0..keys_num {
        let c = ls_fd.pop::<TestRecord>().unwrap().unwrap();
        assert_eq!(c.key(), format!("key{}", k));
        assert_eq!(c.data, value);
    }

    for k in 0..keys_num {
        let c = ls_fd
            .read_by_pointer::<TestRecord>(&LogEntryPointer::new(0, format!("key{}", k)))
            .unwrap()
            .unwrap();
        assert_eq!(c.data, value);
    }
}
