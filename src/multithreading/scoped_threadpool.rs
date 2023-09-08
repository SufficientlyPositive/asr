// directly ripped from scoped_threadpool, maybe better to just use rayon, a lot of this seems pretty hacky...

// LICENSE for scoped_threadpool.
//
// The MIT License (MIT)
//
// Copyright (c) 2015 Marvin Löbel
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::thread::{self, JoinHandle};
use std::sync::mpsc::{channel, Sender, Receiver, SyncSender, sync_channel, RecvError};
use std::sync::{Arc, Mutex};
use std::marker::PhantomData;
use std::mem;

use anyhow::Result;

enum Message {
    NewJob(Thunk<'static>),
    Join,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Thunk<'a> = Box<dyn FnBox + Send + 'a>;

impl Drop for Pool {
    fn drop(&mut self) {
        self.job_sender = None;
    }
}

/// A threadpool that acts as a handle to a number
/// of threads spawned at construction.
pub struct Pool {
    threads: Vec<ThreadData>,
    job_sender: Option<Sender<Message>>
}

struct ThreadData {
    _thread_join_handle: JoinHandle<()>,
    pool_sync_rx: Receiver<()>,
    thread_sync_tx: SyncSender<()>,
}

impl Pool {
    /// Construct a threadpool with the given number of threads.
    /// Minimum value is `1`.
    pub fn new(n: u32) -> Result<Pool> {
        assert!(n >= 1);

        let (job_sender, job_receiver) = channel();
        let job_receiver = Arc::new(Mutex::new(job_receiver));

        let mut threads = Vec::with_capacity(n as usize);

        // spawn n threads, put them in waiting mode
        for _ in 0..n {
            let job_receiver = job_receiver.clone();

            let (pool_sync_tx, pool_sync_rx) =
                sync_channel::<()>(0);
            let (thread_sync_tx, thread_sync_rx) =
                sync_channel::<()>(0);

            let thread = thread::spawn(move || {
                loop {
                    let message = {
                        // Only lock jobs for the time it takes
                        // to get a job, not run it.
                        let lock = job_receiver.lock().unwrap();
                        lock.recv()
                    };

                    match message {
                        Ok(Message::NewJob(job)) => {
                            job.call_box();
                        }
                        Ok(Message::Join) => {
                            // Syncronize/Join with pool.
                            // This has to be a two step
                            // process to ensure that all threads
                            // finished their work before the pool
                            // can continue

                            // Wait until the pool started syncing with threads
                            if pool_sync_tx.send(()).is_err() {
                                // The pool was dropped.
                                break;
                            }

                            // Wait until the pool finished syncing with threads
                            if thread_sync_rx.recv().is_err() {
                                // The pool was dropped.
                                break;
                            }
                        }
                        Err(..) => {
                            // The pool was dropped.
                            break
                        }
                    }
                }
            });

            threads.push(ThreadData {
                _thread_join_handle: thread,
                pool_sync_rx: pool_sync_rx,
                thread_sync_tx: thread_sync_tx,
            });
        }

        Ok(Pool {
            threads: threads,
            job_sender: Some(job_sender),
        })
    }

    /// Borrows the pool and allows executing jobs on other
    /// threads during that scope via the argument of the closure.
    ///
    /// This method will block until the closure and all its jobs have
    /// run to completion.
    pub fn scoped<'pool, 'scope, F, R>(&'pool mut self, f: F) -> R
        where F: FnOnce(&Scope<'pool, 'scope>) -> R
    {
        let scope = Scope {
            pool: self,
            _marker: PhantomData,
        };
        f(&scope)
    }

    /// Returns the number of threads inside this pool.
    pub fn thread_count(&self) -> u32 {
        self.threads.len() as u32
    }
}

/////////////////////////////////////////////////////////////////////////////

/// Handle to the scope during which the threadpool is borrowed.
pub struct Scope<'pool, 'scope> {
    pool: &'pool mut Pool,
    // The 'scope needs to be invariant... it seems?
    _marker: PhantomData<::std::cell::Cell<&'scope mut ()>>,
}

impl<'pool, 'scope> Scope<'pool, 'scope> {
    /// Execute a job on the threadpool.
    ///
    /// The body of the closure will be send to one of the
    /// internal threads, and this method itself will not wait
    /// for its completion.
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'scope {
        self.execute_(f)
    }

    fn execute_<F>(&self, f: F) where F: FnOnce() + Send + 'scope {
        let b = unsafe {
            mem::transmute::<Thunk<'scope>, Thunk<'static>>(Box::new(f))
        };
        self.pool.job_sender.as_ref().unwrap().send(Message::NewJob(b)).unwrap();
    }

    /// Blocks until all currently queued jobs have run to completion.
    pub fn join_all(&self) {
        for _ in 0..self.pool.threads.len() {
            self.pool.job_sender.as_ref().unwrap().send(Message::Join).unwrap();
        }

        // Synchronize/Join with threads
        // This has to be a two step process
        // to make sure _all_ threads received _one_ Join message each.

        // This loop will block on every thread until it
        // received and reacted to its Join message.
        let mut worker_panic = false;
        for thread_data in &self.pool.threads {
            if let Err(RecvError) = thread_data.pool_sync_rx.recv() {
                worker_panic = true;
            }
        }
        if worker_panic {
            // Now that all the threads are paused, we can safely panic
            panic!("Thread pool worker panicked");
        }

        // Once all threads joined the jobs, send them a continue message
        for thread_data in &self.pool.threads {
            thread_data.thread_sync_tx.send(()).unwrap();
        }
    }
}

impl<'pool, 'scope> Drop for Scope<'pool, 'scope> {
    fn drop(&mut self) {
        self.join_all();
    }
}