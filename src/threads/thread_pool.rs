use std::thread::{self, AccessError};
use std::sync::{mpsc, Arc, Mutex};

use tracing::info;

pub struct ThreadPool {
    // threads: Vec<thread::JoinHandle<()>>
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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

        ThreadPool{ workers: workers, sender: sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}


struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            let job = receiver.lock().unwrap().recv().unwrap();
            info!("Worker {id} git a jbo; executing.");
            job();
        });
        Worker { id: id, thread: thread }
    }
}
