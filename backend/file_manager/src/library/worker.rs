use std::{sync::{mpsc, Arc, Mutex}, thread::{self, JoinHandle}};

use crate::logger::app_logger::Logger;

use super::job::Job;

pub struct Worker {
  pub id: usize,
  pub thread: JoinHandle<()>,
}

impl Worker {
  pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
    Logger::info(format!("Thread Pool - Worker: {}", id).as_str());
    let thread = thread::spawn(move || loop {
      let lock = receiver.lock().unwrap().recv();
      match lock {
        Err(_) => {
          Logger::warn("Thread Pool - Worker - Disconnected!");
          break;
        },
        Ok(job) => {
          Logger::info(format!("Thread Pool - Worker {id} - Executing a job").as_str()); 
          job();
        },
      }
    });

    Worker { id, thread }
  }
}
