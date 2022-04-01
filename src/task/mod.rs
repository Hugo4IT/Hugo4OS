use core::{pin::Pin, future::Future, task::{Context, Poll}, sync::atomic::{AtomicU64, Ordering}};

use alloc::boxed::Box;

pub mod executor;
pub mod keyboard;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

impl TaskId {
    fn new() -> TaskId {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub struct Task {
    id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new<F>(future: F) -> Task
    where
        F: Future<Output = ()> + 'static
    {
        Task {
            id: TaskId::new(),
            future: Box::pin(future)
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}