#[cfg(test)]
mod tests;

pub struct ThreadPool {}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        unimplemented!()
    }

    pub fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unimplemented!()
    }
}
