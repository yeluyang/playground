use std::{
    fs,
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

extern crate serde;
use serde::{Deserialize, Serialize};

extern crate serde_json;

use crate::{Header, SegmentFile, HEADER_SIZE};

#[test]
fn test_header() {
    let h1 = Header {
        length: 1024,
        seq_id: 512,
        total: 256,
    };
    assert_eq!(h1.to_bytes().unwrap().len(), HEADER_SIZE);

    let h2 = Header::from(&h1.to_bytes().unwrap()).unwrap();
    assert_eq!(h2, h1);
}

#[test]
fn test_segment_file_io() {
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

    {
        let mut s_file = SegmentFile::create("tmp/tmp.txt").unwrap();
        for case in &cases {
            let bs = serde_json::to_vec(case).unwrap();
            assert_eq!(s_file.write(bs.as_slice()).unwrap(), bs.len());
        }
    }

    let mut s_file = SegmentFile::open("tmp/tmp.txt").unwrap();
    let mut buf = vec![0u8; 2048];
    for case in &cases {
        let bs = serde_json::to_vec(case).unwrap();
        assert_eq!(s_file.read(buf.as_mut_slice()).unwrap(), bs.len());
        assert_eq!(&buf[..bs.len()], bs.as_slice());
        let c: Case = serde_json::from_slice(&buf[..bs.len()]).unwrap();
        assert_eq!(&c, case);
    }

    for (i, _) in cases.iter().enumerate() {
        let bs = serde_json::to_vec(&cases[i]).unwrap();
        s_file.seek(SeekFrom::Start((i + 1) as u64)).unwrap();
        assert_eq!(s_file.read(buf.as_mut_slice()).unwrap(), bs.len());
        assert_eq!(&buf[..bs.len()], bs.as_slice());
        let c: Case = serde_json::from_slice(&buf[..bs.len()]).unwrap();
        assert_eq!(&c, &cases[i]);
    }
}
