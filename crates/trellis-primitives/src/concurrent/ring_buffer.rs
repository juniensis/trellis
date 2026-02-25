use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    panic::{RefUnwindSafe, UnwindSafe},
    sync::atomic::{self, AtomicBool, AtomicUsize, Ordering},
};

/// A generic slot for holding the values within the ring buffer. This is
/// how they are implemented in crossbeam. I initially thought it seemed likely
/// to be more optimized than I would need, however each of my attempts ran
/// into some sort of issue until it eventually became the crossbeam
/// implementation. Very educational though.
///
/// At first I thought I could just use Option<T>, but that would be a pain
/// to initialize since it would require T to implement Clone to initialize
/// the buffer with None. If a generic structure's initialize empty function
/// (i.e. Self::new()) requires a trait bound not required by the structure
/// declaration, something is probably a bit wonky.
///
/// The Option<T> troubles clarified the need for these slots to be allocated
/// arbitrarily, you just want a region of memory that a T would fit into,
/// which is exactly what MaybeUninit is for. But it also has to have interior
/// mutability which is exactly what the Cell types are for. UnsafeCell is used
/// since the other safety guarantees of Cell are not needed.
#[derive(Debug)]
struct Slot<T> {
    // UnsafeCell opts in to unrestricted mutability, MaybeUninit opts in to
    // unitialized data. As far as I can tell, this is practically like using a
    // raw *mut T pointing to an allocation matching T's size. It is done this
    // way to explicitly state how this memory region will be used, whereas
    // *mut T does not clearly communicate what checks must be done to avoid
    // undefined behavior.
    inner: UnsafeCell<MaybeUninit<T>>,
    stamp: AtomicUsize,
}

/// A MPMC ring buffer. Written after studying crossbeam's ArrayQueue, I
/// intended it to differ more than it does but fixing issues with my ideas
/// resulted in it converging towards crossbeam's implementation, turns out
/// those fellows know what they're doing.
#[derive(Debug)]
pub struct RingBuffer<T> {
    head: AtomicUsize,
    tail: AtomicUsize,
    capacity: usize,
    buffer: Box<[Slot<T>]>,
}

unsafe impl<T: Send> Sync for RingBuffer<T> {}
unsafe impl<T: Send> Send for RingBuffer<T> {}

impl<T> UnwindSafe for RingBuffer<T> {}
impl<T> RefUnwindSafe for RingBuffer<T> {}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> RingBuffer<T> {
        assert!(capacity > 0);

        let buffer = (0..capacity)
            .map(|i| Slot {
                inner: UnsafeCell::new(MaybeUninit::uninit()),
                stamp: AtomicUsize::new(i),
            })
            .collect::<Box<[Slot<T>]>>();

        Self {
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            capacity,
            buffer,
        }
    }
    // TODO: Needs to update head
    pub fn push(&self, value: T) {
        let tail = self.tail.load(Ordering::Relaxed);
        let idx = tail % self.capacity;
        unsafe {
            *self.buffer[idx].inner.get() = MaybeUninit::new(value);
        }
        self.buffer[idx].stamp.store(tail + 1, Ordering::Relaxed);
        self.tail.fetch_add(1, Ordering::Relaxed);
    }
    pub fn try_pop(&self) -> Option<T> {
        let mut head = self.head.load(Ordering::Relaxed);
        let idx = head % self.capacity;
        let exp = head + 1;

        let slot = unsafe { self.buffer.get_unchecked(idx) };
        let stamp = slot.stamp.load(Ordering::Acquire);

        if stamp == exp {
            let new = (head + 1) % self.capacity;
            match self
                .head
                .compare_exchange(head, new, Ordering::SeqCst, Ordering::Relaxed)
            {
                Ok(_) => {
                    let ret = unsafe { slot.inner.get().read().assume_init_read() };
                    slot.stamp.store(head + self.capacity, Ordering::Release);
                    return Some(ret);
                }
                Err(h) => {
                    head = h;
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop() {
        let mut buffer = RingBuffer::new(4);
        buffer.push(1);
        assert_eq!(Some(1), buffer.try_pop());
        assert_eq!(None, buffer.try_pop());

        for i in 0..5 {
            buffer.push(i);
        }

        println!("{:?}", buffer);

        assert_eq!(Some(4), buffer.try_pop());
    }
}
