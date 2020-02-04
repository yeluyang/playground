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
        for (i, case) in cases.iter().enumerate() {
            assert_eq!(s_file.index.len(), i);

            let bs = serde_json::to_vec(case).unwrap();
            assert_eq!(s_file.write(bs.as_slice()).unwrap(), bs.len());
        }
        assert_eq!(s_file.index.len(), cases.len());

        let mut last_pos = 0u64;
        for rng in s_file.index.iter() {
            assert_eq!(rng.start(), &last_pos);
            last_pos = *rng.end();
        }
    }

    let mut s_file = SegmentFile::open("tmp/tmp.txt").unwrap();
    assert_eq!(s_file.index.len(), cases.len());
    let mut buf = vec![0u8; 2048];

    for case in &cases {
        let bs = serde_json::to_vec(case).unwrap();
        assert_eq!(s_file.read(buf.as_mut_slice()).unwrap(), bs.len());
        assert_eq!(&buf[..bs.len()], bs.as_slice());

        let c: Case = serde_json::from_slice(&buf[..bs.len()]).unwrap();
        assert_eq!(&c, case);
    }

    for (i, case) in cases.iter().enumerate() {
        assert_eq!(
            &s_file.seek(SeekFrom::Start(i as u64)).unwrap(),
            s_file.index[i].start()
        );

        let bs = serde_json::to_vec(case).unwrap();
        assert_eq!(s_file.read(buf.as_mut_slice()).unwrap(), bs.len());
        assert_eq!(&buf[..bs.len()], bs.as_slice());

        let c: Case = serde_json::from_slice(&buf[..bs.len()]).unwrap();
        assert_eq!(&c, case);
    }
    assert_eq!(s_file.seek(SeekFrom::Start(cases.len() as u64)).unwrap(), 0);

    for (i, case) in cases.iter().rev().enumerate() {
        assert_eq!(
            &s_file.seek(SeekFrom::End(i as i64)).unwrap(),
            s_file.index[s_file.index.len() - 1 - i].start()
        );

        let bs = serde_json::to_vec(case).unwrap();
        assert_eq!(s_file.read(buf.as_mut_slice()).unwrap(), bs.len(),);
        assert_eq!(&buf[..bs.len()], bs.as_slice());

        let c: Case = serde_json::from_slice(&buf[..bs.len()]).unwrap();
        assert_eq!(&c, case);
    }
    assert_eq!(s_file.seek(SeekFrom::End(cases.len() as i64)).unwrap(), 0);
}
