#[cfg(test)]
mod tests;

use std::{
    fs,
    io::{Read, Write},
    net,
    sync::{self, mpsc},
    thread, time,
};

pub fn handle(mut stream: net::TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, file) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "server/assets/index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(time::Duration::from_secs(3600));
        ("HTTP/1.1 200 OK\r\n\r\n", "server/assets/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "assets/404.html")
    };

    let contents = fs::read_to_string(file).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, job_receiver: sync::Arc<sync::Mutex<mpsc::Receiver<Option<Job>>>>) -> Worker {
        return Worker {
            id: id,
            handle: Some(thread::spawn(move || loop {
                log::info!("worker{{id={}}} waiting", id);
                let job = job_receiver.lock().unwrap().recv().unwrap();
                match job {
                    Some(job) => job.call_box(),
                    None => break,
                }
            })),
        };
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    job_sender: mpsc::Sender<Option<Job>>,
}

impl ThreadPool {
    pub fn new(pool_size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = sync::Arc::new(sync::Mutex::new(receiver));

        let mut t = ThreadPool {
            workers: Vec::with_capacity(pool_size),
            job_sender: sender,
        };
        for i in 0..pool_size {
            t.workers.push(Worker::new(i, sync::Arc::clone(&receiver)));
        }
        return t;
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.job_sender.send(Some(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.workers.len() {
            self.job_sender.send(None).unwrap();
        }
        for w in &mut self.workers {
            if let Some(handle) = w.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}
