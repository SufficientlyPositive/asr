use std::{sync::{mpsc, Arc, Mutex}, thread};
use anyhow::Result;

// maybe want to remove the Box<> here and reinstate it inside execute
// not sure it needs to be 'static either since we're dealing with samples...
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Message>>
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

enum Message {
    NewJob(Job),
    Finish
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool> {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id,Arc::clone(&receiver))?);
        }

        Ok(ThreadPool { workers, sender:Some(sender) })
    }

    // might need execute to push something to a new channel
    pub fn execute(&self, job: Job) {
        // let job = Box::new(job);
        match self.sender.as_ref().unwrap().send(Message::NewJob(job)) {
            Ok(_) => {},
            Err(e) => {panic!("An error occured while queuing a job to workers {}.", e)}
        };
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            match self.sender.as_ref().unwrap().send(Message::Finish) {
                Ok(_) => {debug!("Shutting down worker {}.", worker.id)},
                Err(e) => {panic!("An error occured while queuing the finish job for worker {}, they may not have terminated! {}.", worker.id, e)}
            };

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    pub fn new(
        id: usize, 
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>
    ) -> Result<Worker> {
        let builder = thread::Builder::new();

        let thread = builder.spawn(move || 
            loop {
            let job = receiver.lock().expect("Mutex failed to unwrap [inside multithreading\\threadpool.rs].").recv().unwrap();

            match job {
                Message::NewJob(job) => {
                    trace!("Worker {id} has successfully got a job; Executing.");
                    job();
                }
                Message::Finish => {
                    debug!("Worker {id} finishing; shutting down thread.");
                    break
                }
            }
        })?;

        Ok(Worker { id, thread: Some(thread) })
    }
}