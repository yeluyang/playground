use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    thread,
};

/// 各线程clone文件描述符来并发读写，始终自动同步file cursor，不会重复读取某个position，也不会在某个position上出现写竞争
/// 各线程新建文件描述符来并发读写：
/// - 并发写：
///   - write写模式：不会同步file cursor，会在某个position上出现写竞争
///   - append写模式：会自动同步file cursor，不会在某个position上出现写竞争
/// - 并发读：不会同步file cursor，会重复读取某个position
fn main() {
    let file_dir = Path::new("tmp/examples/fs_examples/fd_copy");
    fs::create_dir_all(file_dir).unwrap();
    println!("==== same_fd_clone_io ====");
    same_fd_clone_io(file_dir);
    println!("==== different_fd_new_io ====");
    different_fd_new_io(file_dir);
}

fn same_fd_clone_io(file_dir: &Path) {
    async_fd_write(file_dir);

    let file_path = async_writer(file_dir);
    async_reader_read(file_path.as_path());
}

fn different_fd_new_io(file_dir: &Path) {
    let file_path = new_fd_write(file_dir);
    new_fd_read(file_path.as_path());
}

/// 文件描述符`clone`给多个thread并发写，是自动同步file cursor的。不会在某个position上出现写竞争。
fn async_fd_write(file_dir: &Path) -> PathBuf {
    let file_path = file_dir.join("async_fd_write.txt");
    let fd = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path.as_path())
        .unwrap();
    let mut join_handl = Vec::new();
    for id in 0..3 {
        let f = fd.try_clone().unwrap();
        join_handl.push(thread::spawn(move || do_fd_write(id, 100, f)));
    }
    for hdl in join_handl {
        hdl.join().unwrap();
    }
    return file_path;
}

/// 每个thread各自新建文件描述符并发写：
/// - write: 每个thread各自新建文件描述符并发写，在write模式下不会同步file cursor，会在某个position上写竞争。
/// - append: 每个thread各自新建文件描述符并发写，在append模式下会自动同步file cursor，不会在某个position上写竞争。
fn new_fd_write(file_dir: &Path) -> PathBuf {
    let file_path = file_dir.join("new_fd_write.txt");

    if file_path.exists() {
        fs::remove_file(file_path.as_path()).unwrap()
    };

    let mut join_handl = Vec::new();
    for id in 0..3 {
        let fd = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path.as_path())
            .unwrap();
        join_handl.push(thread::spawn(move || do_fd_write(id, 100, fd)));
    }
    for hdl in join_handl {
        hdl.join().unwrap();
    }
    return file_path;
}

fn do_fd_write(id: usize, lines: usize, mut fd: File) {
    for l in 0..lines {
        fd.write(format!("writer_{}: {}\n", id, l).as_bytes())
            .unwrap();
        fd.flush().unwrap();
    }
}

/// 文件描述符`clone`给多个writer并发写，是自动同步file cursor的。不会在某个position上出现写竞争。
fn async_writer(file_dir: &Path) -> PathBuf {
    let file_path = file_dir.join("async_writer.txt");
    let fd = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path.as_path())
        .unwrap();
    let mut join_handl = Vec::new();
    for id in 0..1 {
        let f = fd.try_clone().unwrap();
        join_handl.push(thread::spawn(move || do_writer_write(id, 100, f)));
    }
    for hdl in join_handl {
        hdl.join().unwrap();
    }
    return file_path;
}

fn do_writer_write(id: usize, lines: usize, fd: File) {
    let mut w = BufWriter::new(fd);
    for l in 0..lines {
        w.write(format!("writer_{}: {}\n", id, l).as_bytes())
            .unwrap();
        w.flush().unwrap();
    }
}

/// 文件描述符`clone`给多个reader并发读，是自动同步file cursor的，不会重复读取某个position。
fn async_reader_read(file_path: &Path) {
    let fd = OpenOptions::new().read(true).open(file_path).unwrap();
    let mut join_handl = Vec::new();
    for id in 0..3 {
        let f = fd.try_clone().unwrap();
        join_handl.push(thread::spawn(move || do_reader_read(id, f)));
    }
    for hdl in join_handl {
        hdl.join().unwrap();
    }
}

/// 每个thread各自新建文件描述符并发读，不会同步file cursor，会重复读取某个position。
fn new_fd_read(file_path: &Path) {
    let mut join_handl = Vec::new();
    for id in 0..3 {
        let fd = OpenOptions::new().read(true).open(file_path).unwrap();
        join_handl.push(thread::spawn(move || do_reader_read(id, fd)));
    }
    for hdl in join_handl {
        hdl.join().unwrap();
    }
}

fn do_reader_read(id: usize, fd: File) {
    let mut r = BufReader::new(fd);
    println!(
        "reader-{} start at offset={}",
        id,
        r.seek(SeekFrom::Current(0)).unwrap()
    );
    let mut lines = 0usize;
    let mut buf = String::new();
    while r.read_line(&mut buf).unwrap() > 0 {
        lines += 1;
        println!(
            "reader-{}: line-{}=({}) in offset={}",
            id,
            lines,
            &buf[..buf.len() - 1],
            r.seek(SeekFrom::Current(0)).unwrap(),
        );
        buf.clear();
    }
}
