use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    time::Duration,
};

#[derive(Debug)]
struct QueueInner<T> {
    queue: Mutex<VecDeque<T>>,
    condvar: Condvar,
}

#[derive(Debug)]
pub struct Queue<T> {
    inner: Arc<QueueInner<T>>,
}

impl<T> Clone for Queue<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Queue<T> {
    /// Create a new empty queue. Queue<T> wraps a Mutex protected VecDeque,
    /// providing locking but thread safe MPMC usage.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(QueueInner {
                queue: Mutex::new(VecDeque::new()),
                condvar: Condvar::new(),
            }),
        }
    }
    /// Push an item to the back of the queue.
    pub fn push(&self, item: T) {
        self.inner
            .queue
            .lock()
            .expect("Queue::push() lock failed.")
            .push_back(item);
        self.inner.condvar.notify_one();
    }
    /// Try to pop an item without blocking.
    pub fn pop(&self) -> Option<T> {
        let mut queue = self
            .inner
            .queue
            .lock()
            .expect("EventQueue::recv() lock failed.");
        queue.pop_front()
    }
    /// Block until an item is popped. I can't really think of a time where
    /// this should be used since it risks blocking forever if an item never
    /// gets pushed.
    pub fn blocking_pop(&self) -> T {
        let mut queue = self
            .inner
            .queue
            .lock()
            .expect("EventQueue::recv() lock failed.");
        loop {
            if let Some(val) = queue.pop_front() {
                return val;
            }
            queue = self
                .inner
                .condvar
                .wait(queue)
                .expect("EventQueue::recv() condvar wait failed.");
        }
    }
    pub fn pop_timeout(&self, timeout: Duration) -> Option<T> {
        let mut queue = self
            .inner
            .queue
            .lock()
            .expect("EventQueue::recv() lock failed.");
        loop {
            if let Some(val) = queue.pop_front() {
                return Some(val);
            }
            let (q, timed_out) = self
                .inner
                .condvar
                .wait_timeout(queue, timeout)
                .expect("EventQueue::recv_timeout() condvar wait_timeout failed.");
            queue = q;
            if timed_out.timed_out() {
                return queue.pop_front();
            }
        }
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<T> Send for Queue<T> {}
unsafe impl<T> Sync for Queue<T> {}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, thread};

    use crate::collections::queue::Queue;

    // Verifies that push and pop work in the simplest of cases.
    #[test]
    fn single_threaded_push_pop() {
        let queue = Queue::new();

        for i in 0..128 {
            queue.push(i);
        }

        for j in 0..128 {
            assert_eq!(queue.pop(), Some(j));
        }
    }
    // Verifies that multiple threads can all push without items being lost.
    #[test]
    fn multithreaded_push_pop() {
        let queue = Queue::new();

        let mut samples = Vec::new();

        for i in 0..128 {
            samples.push((i, (0..128).collect::<Vec<_>>()));
        }
        let mut exp = HashSet::new();
        for smp in &samples {
            for j in &smp.1 {
                exp.insert((smp.0, *j));
            }
        }
        for sample in samples {
            let handle = queue.clone();
            thread::spawn(move || {
                let i = sample.0;
                for x in sample.1 {
                    handle.push((i, x));
                }
            });
        }

        let mut got = HashSet::new();

        while let Some(popped) = queue.pop() {
            got.insert(popped);
        }

        assert_eq!(got, exp);
    }
}
