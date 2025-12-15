use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicI32, AtomicUsize, Ordering},
    },
    thread,
    time::Instant,
};

const THREADS: usize = 8;
const ITERATIONS: usize = 1_000_000;

fn mutex_test() {
    let counter = Arc::new(Mutex::new(0));
    let start = Instant::now();

    let mut handles = vec![];
    for _ in 0..THREADS {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..ITERATIONS {
                *c.lock().unwrap() += 1;
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Mutex time: {:?}", start.elapsed());
}

fn atomic_test() {
    let counter = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();

    let mut handles = vec![];
    for _ in 0..THREADS {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..ITERATIONS {
                c.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Atomic time: {:?}", start.elapsed());
}

pub fn run() {
    println!("Data race demo");

    let t1 = thread::spawn(mutex_test);
    let t2 = thread::spawn(atomic_test);

    t1.join().unwrap();
    t2.join().unwrap();

    println!("______________");
}
