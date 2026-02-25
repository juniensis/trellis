use std::{
    collections::VecDeque,
    fmt,
    sync::{Arc, Condvar, Mutex},
    time::Duration,
};

#[derive(Debug)]
pub(crate) struct EventQueueInner<T> {
    queue: Mutex<VecDeque<T>>,
    condvar: Condvar,
}

pub struct EventSender<T> {
    inner: Arc<EventQueueInner<T>>,
}

impl<T> EventSender<T> {
    pub fn send(&self, e: T) {
        self.inner.queue.lock().unwrap().push_back(e);
        self.inner.condvar.notify_one();
    }
}

impl<T: Clone> Clone for EventSender<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

pub struct EventReceiver<T> {
    inner: Arc<EventQueueInner<T>>,
}

impl<T> EventReceiver<T> {
    pub fn try_recv(&self) -> Option<T> {
        self.inner
            .queue
            .lock()
            .expect("EventQueue::try_recv() lock failed.")
            .pop_front()
    }
    pub fn recv(&self) -> T {
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
    pub fn recv_timeout(&self, timeout: Duration) -> Option<T> {
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

impl<T> Clone for EventReceiver<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

pub struct EventQueue<T> {
    inner: Arc<EventQueueInner<T>>,
}

impl<T> EventQueueInner<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        }
    }
}

impl<T> EventQueue<T> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(EventQueueInner::new()),
        }
    }
    pub fn sender(&self) -> EventSender<T> {
        EventSender {
            inner: self.inner.clone(),
        }
    }
    pub fn receiver(&self) -> EventReceiver<T> {
        EventReceiver {
            inner: self.inner.clone(),
        }
    }
    pub fn push(&self, e: T) {
        self.inner.queue.lock().unwrap().push_back(e);
        self.inner.condvar.notify_one();
    }
    pub fn try_recv(&self) -> Option<T> {
        self.inner
            .queue
            .lock()
            .expect("EventQueue::try_recv() lock failed.")
            .pop_front()
    }
    pub fn recv(&self) -> T {
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
    pub fn recv_timeout(&self, timeout: Duration) -> Option<T> {
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

impl<T: Default> Default for EventQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}
