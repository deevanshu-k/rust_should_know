use std::{
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

struct Node<T> {
    value: Option<T>,
    next: AtomicPtr<Node<T>>,
}

pub struct MpscQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: *mut Node<T>,
}

impl<T> MpscQueue<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node {
            value: None,
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        Self {
            head: AtomicPtr::new(dummy),
            tail: dummy,
        }
    }

    pub fn push(&self, value: T) {
        let node = Box::into_raw(Box::new(Node {
            value: Some(value),
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        let prev = self.head.swap(node, Ordering::AcqRel);
        unsafe {
            (*prev).next.store(node, Ordering::Release);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            let next = (*self.tail).next.load(Ordering::Acquire);
            if next.is_null() {
                return None;
            }

            let value = (*next).value.take();
            let old = self.tail;
            self.tail = next;

            drop(Box::from_raw(old));
            value
        }
    }
}

pub fn run() {
    println!("Lock free MPSC queue");

    let mut q = MpscQueue::new();
    for i in 0..10 {
        q.push(i + 1);
    }

    for i in 0..10 {
        assert_eq!(q.pop(), Some(i + 1));
    }

    println!("____________________");
}
