use std::{
    env,
    fs::{self, File},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use crate::Buffer;

static TXT_DATA: (&str, &[&[u8]]) = (
    "fox.txt",
    &[b"The quick brown fox\n", b"jumps over\n", b"the lazy dog\n"],
);

fn assets_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("assets")
}

#[test]
fn test_buffer_seek() {
    let fd = File::open(assets_dir().join(TXT_DATA.0)).unwrap();
    let mut buffer = Buffer::new(fd).unwrap();
    buffer
        .seek(SeekFrom::Start(TXT_DATA.1[0].len() as u64))
        .unwrap();
    let mut buf = vec![0; TXT_DATA.1[1].len()];
    let len = buffer.read(buf.as_mut_slice()).unwrap();
    assert_eq!(TXT_DATA.1[1].len(), len);
    assert_eq!(TXT_DATA.1[1], buf.as_slice());
}

#[test]
fn test_buffer_read() {
    let fd = File::open(assets_dir().join(TXT_DATA.0)).unwrap();
    let mut buffer = Buffer::new(fd).unwrap();
    for txt in TXT_DATA.1 {
        let mut buf = vec![0; txt.len()];
        let len = buffer.read(buf.as_mut_slice()).unwrap();
        assert_eq!(txt.len(), len);
        assert_eq!(*txt, buf.as_slice());
    }
}

#[test]
fn test_buffer_write() {
    let tmp_dir = Path::new("tmp");
    if !tmp_dir.exists() {
        fs::create_dir(tmp_dir).unwrap();
    }
    {
        let fd = File::create(tmp_dir.join("tmp.txt")).unwrap();
        let mut buffer = Buffer::new(fd).unwrap();
        for txt in TXT_DATA.1 {
            let len = buffer.write(txt).unwrap();
            assert_eq!(len, txt.len())
        }
    }

    let fd = File::open(tmp_dir.join("tmp.txt")).unwrap();
    let mut buffer = Buffer::new(fd).unwrap();
    for txt in TXT_DATA.1 {
        let mut buf = vec![0; txt.len()];
        let len = buffer.read(buf.as_mut_slice()).unwrap();
        assert_eq!(txt.len(), len);
        assert_eq!(*txt, buf.as_slice());
    }
}
