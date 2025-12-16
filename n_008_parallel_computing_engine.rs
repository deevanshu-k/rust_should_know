use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct JobQueue {
    queue: Mutex<VecDeque<Job>>,
    not_empty: Condvar,
    not_full: Condvar,
    capacity: usize,
    shutdown: Mutex<bool>,
}

impl JobQueue {
    fn push(&self, job: Job) {
        let mut q = self.queue.lock().unwrap();

        while q.len() == self.capacity {
            q = self.not_full.wait(q).unwrap();
        }

        q.push_back(job);
        self.not_empty.notify_all();
    }

    fn pop(&self) -> Option<Job> {
        let mut q = self.queue.lock().unwrap();

        loop {
            if let Some(job) = q.pop_front() {
                self.not_full.notify_one();
                return Some(job);
            }

            if *self.shutdown.lock().unwrap() {
                return None;
            }

            q = self.not_empty.wait(q).unwrap();
        }
    }
}

fn worker_loop(queue: Arc<JobQueue>) {
    loop {
        let job = queue.pop();
        match job {
            Some(job) => {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(job));
                if result.is_err() {
                    eprintln!("Worker: job panicked, isolated");
                }
            }
            None => break, // SHUTDOWN
        }
    }
}

struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    queue: Arc<JobQueue>,
}

impl ThreadPool {
    pub fn new(workers: usize, capacity: usize) -> Self {
        let queue = Arc::new(JobQueue {
            queue: Mutex::new(VecDeque::new()),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
            capacity,
            shutdown: Mutex::new(false),
        });

        let mut handlers = vec![];
        for _ in 0..workers {
            let queue_clone = Arc::clone(&queue);
            handlers.push(thread::spawn(move || worker_loop(queue_clone)));
        }

        Self {
            workers: handlers,
            queue,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.queue.push(Box::new(f));
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        *self.queue.shutdown.lock().unwrap() = true;

        self.queue.not_empty.notify_all();

        for worker in self.workers.drain(..) {
            worker.join().unwrap();
        }
    }
}

pub fn run() {
    println!("Parallel Computing Engine");

    {
        let pool = ThreadPool::new(4, 8);

        for i in 1..101 {
            pool.execute(move || {
                println!("Job {}", i);
            });
        }
    }

    println!("_________________________")
}
