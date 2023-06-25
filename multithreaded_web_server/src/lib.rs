#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    unreachable_code,
    clippy::vec_init_then_push,
    clippy::unnecessary_sort_by,
    clippy::match_like_matches_macro,
    clippy::mutable_key_type,
    clippy::single_component_path_imports
)]
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

// make a document for this lib, it is a library for a thread pool

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // create a channel to send jobs to the threads
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        // create a vector to hold the threads
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    // execute a closure on a thread in the pool
    pub fn execute<F>(&self, job: F)
    // we make use of zero abstraction here, so we need to specify
    // the trait bounds for the closure
    where
        F: FnOnce() + Send + 'static,
    {
        // send the job down the channel
        self.sender.as_ref().unwrap().send(Box::new(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Drop the sender to force all the workers to finish up.
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // If there's a thread in this worker, wait for
            // it to finish.  If thread is None, there's
            // nothing to clean up.
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>, // indicates that the job will return unit
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // lock the mutex, unwrap the result to panic on error
            // let job = receiver.lock().unwrap().recv() ; // this will also release the lock before executing the job
            let lock = receiver.lock().unwrap();
            let job = lock.recv();
            drop(lock); // release the lock before executing the job
            match job {
                Ok(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Err(err) => {
                    println!("Worker {} got a error: {}; finishing worker", id, err);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn test() {
    let (p, c) = mpsc::channel::<i32>();
    let _ = p.send(5);
    drop(p);
    let r1 = c.recv().unwrap();
    println!("Got {}", r1);
    let r2 = c.recv().unwrap();
    println!("Got {}", r2);
}
