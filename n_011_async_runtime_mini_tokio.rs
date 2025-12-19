use std::{
    collections::VecDeque,
    pin::Pin,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

// Pin → cannot move in memory
// Futures must not move after polling → that’s why Pin.
struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
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
    queue: VecDeque<Task>,
}

impl Executor {
    fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
        let task = Task {
            future: Box::pin(future),
        };
        self.queue.push_back(task);
    }

    fn run(&mut self) {
        while let Some(mut task) = self.queue.pop_front() {
            let waker = dummy_waker();
            let mut cx = Context::from_waker(&waker);

            match task.future.as_mut().poll(&mut cx) {
                Poll::Ready(()) => {
                    // task finished, drop it
                    println!("Poll ready!");
                }
                Poll::Pending => {
                    self.queue.push_back(task);
                }
            }
        }
    }
}

async fn hello() {
    println!("Hello async world");
}

pub fn run() {
    println!("Async runtine mini tokio");

    let mut ex = Executor {
        queue: VecDeque::new(),
    };
    ex.spawn(hello());
    ex.run();

    println!("________________________");
}
