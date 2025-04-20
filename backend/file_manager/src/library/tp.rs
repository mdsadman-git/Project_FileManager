use std::sync::{mpsc, Arc, Mutex};

use crate::logger::app_logger::Logger;

use super::worker::Worker;
use super::job::Job;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
  pub fn new(capacity: usize) -> Self {
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec::with_capacity(capacity);
    for id in 0..capacity {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool { workers, sender: Some(sender) }
  }

  pub fn execute<F>(&self, f: F) 
  where 
    F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);
    self.sender.as_ref().unwrap().send(job).unwrap();
  }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
      drop(self.sender.take());

      for worker in self.workers.drain(..) {
        Logger::warn(format!("Thread Pool - Worker {} - Shutdown", worker.id).as_str());
        worker.thread.join().unwrap();
      }
    }
}
