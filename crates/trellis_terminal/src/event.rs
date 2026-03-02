use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Key { code: KeyCode, modifiers: Modifiers },
    Resized(usize, usize),
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    Char(char),
    Enter,
    Up,
    Down,
    Left,
    Right,
    Escape,
    Backspace,
    Tab,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Modifiers(u8);

impl Modifiers {
    pub const NONE: u8 = 0b0000;
    pub const SUPER: u8 = 0b0001;
    pub const ALT: u8 = 0b0010;
    pub const CTRL: u8 = 0b0100;

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
    pub fn iter(&self) -> impl Iterator<Item = Modifier> {
        (0..3).filter_map(|x| {
            if self.0 & (1 << x) != 0 {
                Some(match x {
                    0 => Modifier::Super,
                    1 => Modifier::Alt,
                    2 => Modifier::Ctrl,
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
    Ctrl,
    Alt,
    Super,
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
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

        write!(f, "{}", strs.join("+"))
    }
}
