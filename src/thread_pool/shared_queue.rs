use super::ThreadPool;
use crate::Result;

/// A thread pool using a shared queue inside.
///
/// If a spawned task panics, the old thread will be destroyed and a new one will be
/// created. It fails silently when any failure to create the thread at the OS level
/// is captured after the thread pool is created. So, the thread number in the pool
/// can decrease to zero, then spawning a task to the thread pool will panic.
pub struct SharedQueueThreadPool;

impl ThreadPool for SharedQueueThreadPool{
    fn new(num: u32) -> Result<Self> where
        Self: Sized {
        unimplemented!()
    }

    fn spawn<F>(&self, job: F) where
        F: FnOnce() + Send + 'static {
        unimplemented!()
    }
}