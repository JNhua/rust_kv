use super::ThreadPool;
use std::thread;
use crate::Result;

/// It is actually not a thread pool. It spawns a new thread every time
/// the `spawn` method is called.
pub struct NaiveThreadPool;

impl ThreadPool for NaiveThreadPool{
    fn new(_num: u32) -> Result<Self> where
        Self: Sized {
        Ok(NaiveThreadPool)
    }

    fn spawn<F>(&self, job: F) where
        F: FnOnce() + Send + 'static {
        thread::spawn(job);
    }
}