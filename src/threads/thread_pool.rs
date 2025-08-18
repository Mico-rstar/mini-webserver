use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use tracing::{info, error};

#[derive(Debug, thiserror::Error)]
pub enum ThreadPoolError {
    #[error("Missing sender(sender = None)")]
    MissingSender,
}

pub struct ThreadPool {
    // threads: Vec<thread::JoinHandle<()>>
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            // TO_DO: create threads
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers: workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = self.sender.as_ref() {
            sender.send(job)?;
            Ok(())
        } else {
            Err(ThreadPoolError::MissingSender.into())
        }
        
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 拿走Option中的所有权
        drop(self.sender.take());

        for worker in &mut self.workers {
            info!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                if let Err(e) = thread.join() {
                    error!("Failed to join worker thread {}: {:?}", worker.id, e);
                }
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // receiver.lock().unwrap获得一个Mutex智能指针，是一个临时变量，本行语句结束会马上释放
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        info!("Worker {id} git a job; executing.");
                        job();
                    },
                    Err(_) => {
                        info!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });
        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}
