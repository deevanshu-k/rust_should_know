use std::{cell::UnsafeCell, thread};

struct Counter {
    value: UnsafeCell<i32>,
}

unsafe impl Sync for Counter {}

static COUNTER: Counter = Counter {
    value: UnsafeCell::new(0),
};

pub fn run() {
    println!("Create real race condition");
    let mut handlers = vec![];

    for _ in 0..5 {
        handlers.push(thread::spawn(|| {
            for _ in 0..100_000 {
                unsafe {
                    *COUNTER.value.get() += 1;
                }
            }
        }));
    }

    for h in handlers {
        h.join().unwrap();
    }

    #[allow(static_mut_refs)]
    unsafe {
        println!("Counter: {}", *COUNTER.value.get());
    }

    println!("__________________________");
}
