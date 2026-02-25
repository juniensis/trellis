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
    Backspace,
    Tab,
    Escape,
    Left,
    Right,
    Up,
    Down,
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
    pub fn iter(&self) -> impl Iterator<Item = Modifier> {
        (0..4).filter_map(|x| {
            if self.0 & (1 << x) != 0 {
                Some(match x {
                    0 => Modifier::Super,
                    1 => Modifier::Alt,
                    2 => Modifier::Ctrl,
                    3 => Modifier::Shift,
                    _ => unreachable!(),
                })
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modifier {
    None,
    Shift,
    Ctrl,
    Alt,
    Super,
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::Shift => write!(f, "SHIFT"),
            Self::Ctrl => write!(f, "CTRL"),
            Self::Alt => write!(f, "ALT"),
            Self::Super => write!(f, "SUPER"),
        }
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

        write!(f, "{}", strs.join("+"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalEvent {
    Key { code: KeyCode, modifiers: Modifiers },
    Resized(usize, usize),
    Quit,
}

impl TerminalEvent {
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
                    crossterm::event::KeyCode::Backspace => KeyCode::Enter,
                    crossterm::event::KeyCode::Tab => KeyCode::Tab,
                    crossterm::event::KeyCode::Left => KeyCode::Left,
                    crossterm::event::KeyCode::Right => KeyCode::Right,
                    crossterm::event::KeyCode::Up => KeyCode::Up,
                    crossterm::event::KeyCode::Down => KeyCode::Down,
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

                Some(TerminalEvent::Key {
                    code: kc,
                    modifiers: mods,
                })
            }
            crossterm::event::Event::Resize(x, y) => {
                Some(TerminalEvent::Resized(x as usize, y as usize))
            }
            _ => None,
        }
    }
}

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
