use std::{
    collections::VecDeque,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

// Pin → cannot move in memory
// Futures must not move after polling → that’s why Pin.
struct Task {
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
}

fn dummy_waker() -> Waker {
    unsafe fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    fn dummy_raw_waker() -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

struct Executor {
    queue: Arc<Mutex<VecDeque<Arc<Mutex<Task>>>>>,
}

impl Executor {
    fn spawn(&mut self, future: impl Future<Output = ()> + 'static + Send) {
        let task = Task {
            future: Box::pin(future),
        };
        self.queue
            .lock()
            .unwrap()
            .push_back(Arc::new(Mutex::new(task)));
    }

    fn run(&mut self) {
        while let Some(task) = self.queue.lock().unwrap().pop_front() {
            let waker = dummy_waker();
            let mut cx = Context::from_waker(&waker);

            let mut task_lock = task.lock().unwrap();

            match task_lock.future.as_mut().poll(&mut cx) {
                Poll::Ready(()) => {
                    // task finished, drop it
                    println!("Poll ready!");
                }
                Poll::Pending => {
                    self.queue.lock().unwrap().push_back(task.clone());
                }
            }
        }
    }
}

async fn hello(n: i8) {
    println!("{}> Async function", n);
}

pub fn run() {
    println!("Async runtine mini tokio");

    let mut ex = Executor {
        queue: Arc::new(Mutex::new(VecDeque::new())),
    };
    for i in 0..10 {
        ex.spawn(hello(i));
    }
    ex.run();

    println!("________________________");
}
