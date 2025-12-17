use std::{
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread,
};

// => What Relaxed Actually Means
// static COUNTER: AtomicUsize = AtomicUsize::new(0);

// pub fn run() {
//     println!("Lock free counter and queue");
//     let mut handles = vec![];
//     for _ in 0..8 {
//         handles.push(thread::spawn(|| {
//             for _ in 0..100_000 {
//                 COUNTER.fetch_add(1, Ordering::Relaxed);
//             }
//         }));
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Final Counter: {}", COUNTER.load(Ordering::Relaxed));

//     println!("___________________________");
// }

// => When Relaxed FAILS (Visibility Example)
// static READY: AtomicBool = AtomicBool::new(false);
// static DATA: AtomicUsize = AtomicUsize::new(0);

// pub fn run() {
//     println!("Lock free counter and queue");

//     let reader = thread::spawn(|| {
//         while !READY.load(Ordering::Relaxed) {}
//         println!("DATA = {}", DATA.load(Ordering::Relaxed));
//     });
//     let writer = thread::spawn(|| {
//         DATA.store(42, Ordering::Relaxed);
//         READY.store(true, Ordering::Relaxed);
//     });

//     writer.join().unwrap();
//     reader.join().unwrap();

//     println!("___________________________");
// }
// - Stores can be reordered
// - Visibility not synchronized

// => Fix with Acquire / Release
static READY: AtomicBool = AtomicBool::new(false);
static DATA: AtomicUsize = AtomicUsize::new(0);
pub fn run() {
    println!("Lock free counter and queue");

    let reader = thread::spawn(|| {
        while !READY.load(Ordering::Acquire) {}
        println!("DATA = {}", DATA.load(Ordering::Relaxed));
    });
    let writer = thread::spawn(|| {
        DATA.store(42, Ordering::Relaxed);
        READY.store(true, Ordering::Release);
    });

    writer.join().unwrap();
    reader.join().unwrap();

    println!("___________________________");
}
