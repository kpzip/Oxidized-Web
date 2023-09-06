use std::error::Error;
use std::fmt;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[derive(Debug)]
pub struct PoolCreationError {
    info: String,
}

impl PoolCreationError {
    fn new(msg: &str) -> PoolCreationError {
        PoolCreationError {
            info: msg.to_string(),
        }
    }
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.info)
    }
}

impl Error for PoolCreationError {
    fn description(&self) -> &str {
        &self.info
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::new("size of thread pool cannot be zero"));
        }

        let (sender, receiver) = mpsc::channel();

        let reciever = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        for id in 0..size {
            //make threads
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        Ok(ThreadPool { workers, sender })
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
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = reciever.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}
