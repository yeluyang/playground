use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

#[cfg(test)]
mod tests;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    pool: Vec<Option<JoinHandle<()>>>,
    ch: Sender<Option<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Option<Job>>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut p = Self {
            pool: Vec::new(),
            ch: sender,
        };

        for _ in 0..size {
            let recv = receiver.clone();
            p.pool.push(Some(thread::spawn(move || loop {
                let job = recv.lock().unwrap().recv().unwrap();
                match job {
                    Some(job) => {
                        job();
                    }
                    None => break,
                };
            })))
        }

        p
    }

    pub fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.ch.send(Some(Job::from(Box::new(job)))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.pool.len() {
            self.ch.send(None).unwrap();
        }
        for p in &mut self.pool {
            if let Some(h) = p.take() {
                h.join().unwrap();
            }
        }
    }
}
