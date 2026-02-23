use std::{
    collections::VecDeque,
    fmt,
    sync::{Arc, Condvar, Mutex},
    time::Duration,
};

use crossterm::event::{KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    Char(char),
    Enter,
    Escape,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Modifiers(u8);

impl Modifiers {
    pub const NONE: u8 = 0b0000_0000;
    pub const SUPER: u8 = 0b0000_0001;
    pub const ALT: u8 = 0b0000_0010;
    pub const CTRL: u8 = 0b0000_0100;
    pub const SHIFT: u8 = 0b0000_1000;
    pub const LEFT: u8 = 0b1000_0000;
    pub const RIGHT: u8 = 0b0100_0000;
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
    #[inline]
    pub fn with_alt(mut self) -> Self {
        self.0 |= Self::ALT;
        self
    }
    #[inline]
    pub fn with_ctrl(mut self) -> Self {
        self.0 |= Self::CTRL;
        self
    }
    #[inline]
    pub fn with_super(mut self) -> Self {
        self.0 |= Self::SUPER;
        self
    }
    #[inline]
    pub fn with_shift(mut self) -> Self {
        self.0 |= Self::SHIFT;
        self
    }
    #[inline]
    pub fn left(mut self) -> Self {
        self.0 |= Self::LEFT;
        self
    }
    #[inline]
    pub fn right(mut self) -> Self {
        self.0 |= Self::RIGHT;
        self
    }
    #[inline]
    pub fn is_alt(&self) -> bool {
        self.0 & Self::ALT != 0
    }
    #[inline]
    pub fn is_super(&self) -> bool {
        self.0 & Self::SUPER != 0
    }
    #[inline]
    pub fn is_ctrl(&self) -> bool {
        self.0 & Self::CTRL != 0
    }
    #[inline]
    pub fn is_shift(&self) -> bool {
        self.0 & Self::SHIFT != 0
    }
    #[inline]
    pub fn is_left(&self) -> bool {
        self.0 & Self::LEFT != 0
    }
    #[inline]
    pub fn is_right(&self) -> bool {
        self.0 & Self::RIGHT != 0
    }
}

impl fmt::Debug for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

impl fmt::Display for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut strs = Vec::new();
        if self.is_alt() {
            strs.push("ALT");
        }
        if self.is_super() {
            strs.push("SUPER")
        }
        if self.is_ctrl() {
            strs.push("CTRL")
        }
        if self.is_shift() {
            strs.push("SHIFT")
        }

        if self.is_right() {
            write!(f, "RIGHT: ")?;
        } else if self.is_left() {
            write!(f, "LEFT: ")?;
        }

        write!(f, "{}", strs.join("+"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Key { code: KeyCode, modifiers: Modifiers },
    Resized(usize, usize),
    Quit,
}

impl Event {
    pub fn from_crossterm_event(e: crossterm::event::Event) -> Option<Self> {
        match e {
            crossterm::event::Event::Key(KeyEvent {
                code,
                modifiers,
                kind: _,
                state: _,
            }) => {
                let kc = match code {
                    crossterm::event::KeyCode::Esc => KeyCode::Escape,
                    crossterm::event::KeyCode::Enter => KeyCode::Enter,
                    crossterm::event::KeyCode::Char(ch) => KeyCode::Char(ch),
                    _ => return None,
                };
                let mut mods = Modifiers::new();
                for modi in modifiers.iter() {
                    match modi {
                        KeyModifiers::ALT => mods = mods.with_alt(),
                        KeyModifiers::SUPER => mods = mods.with_super(),
                        KeyModifiers::CONTROL => mods = mods.with_ctrl(),
                        _ => {}
                    }
                }

                Some(Event::Key {
                    code: kc,
                    modifiers: mods,
                })
            }
            crossterm::event::Event::Resize(x, y) => Some(Event::Resized(x as usize, y as usize)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct EventQueueInner {
    queue: Mutex<VecDeque<Event>>,
    condvar: Condvar,
}

pub struct EventSender {
    inner: Arc<EventQueueInner>,
}

impl EventSender {
    pub fn send(&self, e: Event) {
        self.inner.queue.lock().unwrap().push_back(e);
        self.inner.condvar.notify_one();
    }
}

impl Clone for EventSender {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

pub struct EventQueue {
    inner: Arc<EventQueueInner>,
}

impl EventQueueInner {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        }
    }
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(EventQueueInner::new()),
        }
    }
    pub fn sender(&self) -> EventSender {
        EventSender {
            inner: self.inner.clone(),
        }
    }
    pub fn push(&self, e: Event) {
        self.inner.queue.lock().unwrap().push_back(e);
        self.inner.condvar.notify_one();
    }
    pub fn try_recv(&self) -> Option<Event> {
        self.inner
            .queue
            .lock()
            .expect("EventQueue::try_recv() lock failed.")
            .pop_front()
    }
    pub fn recv(&self) -> Event {
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
    pub fn recv_timeout(&self, timeout: Duration) -> Option<Event> {
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

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}
