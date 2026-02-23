use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
    Reset,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    ID(u8),
    RGB(u8, u8, u8),
}

impl Color {
    #[inline]
    pub fn as_ansi_fg(&self) -> String {
        match self {
            Self::Black => String::from("\x1b[30m"),
            Self::Red => String::from("\x1b[31m"),
            Self::Green => String::from("\x1b[32m"),
            Self::Yellow => String::from("\x1b[33m"),
            Self::Blue => String::from("\x1b[34m"),
            Self::Magenta => String::from("\x1b[35m"),
            Self::Cyan => String::from("\x1b[36m"),
            Self::White => String::from("\x1b[37m"),
            Self::Default => String::from("\x1b[39m"),
            Self::Reset => String::from("\x1b[0m"),
            Self::BrightBlack => String::from("\x1b[90m"),
            Self::BrightRed => String::from("\x1b[91m"),
            Self::BrightGreen => String::from("\x1b[92m"),
            Self::BrightYellow => String::from("\x1b[93m"),
            Self::BrightBlue => String::from("\x1b[94m"),
            Self::BrightMagenta => String::from("\x1b[95m"),
            Self::BrightCyan => String::from("\x1b[96m"),
            Self::BrightWhite => String::from("\x1b[97m"),
            Self::ID(id) => format!("\x1b[38;5{id}m"),
            Self::RGB(r, g, b) => format!("\x1b[38;2;{r};{g};{b}m"),
        }
    }
    #[inline]
    pub fn as_ansi_bg(&self) -> String {
        match self {
            Self::Black => String::from("\x1b[40m"),
            Self::Red => String::from("\x1b[41m"),
            Self::Green => String::from("\x1b[42m"),
            Self::Yellow => String::from("\x1b[43m"),
            Self::Blue => String::from("\x1b[44m"),
            Self::Magenta => String::from("\x1b[45m"),
            Self::Cyan => String::from("\x1b[46m"),
            Self::White => String::from("\x1b[47m"),
            Self::Default => String::from("\x1b[49m"),
            Self::Reset => String::from("\x1b[0m"),
            Self::BrightBlack => String::from("\x1b[100m"),
            Self::BrightRed => String::from("\x1b[101m"),
            Self::BrightGreen => String::from("\x1b[102m"),
            Self::BrightYellow => String::from("\x1b[103m"),
            Self::BrightBlue => String::from("\x1b[104m"),
            Self::BrightMagenta => String::from("\x1b[105m"),
            Self::BrightCyan => String::from("\x1b[106m"),
            Self::BrightWhite => String::from("\x1b[107m"),
            Self::ID(id) => format!("\x1b[48;5{id}m"),
            Self::RGB(r, g, b) => format!("\x1b[48;2;{r};{g};{b}m"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct Modes(pub u8);

impl Modes {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
    #[inline]
    pub fn bold() -> Self {
        Self(0b1000_0000)
    }
    #[inline]
    pub fn dim() -> Self {
        Self(0b0100_0000)
    }
    #[inline]
    pub fn italic() -> Self {
        Self(0b0010_0000)
    }
    #[inline]
    pub fn underline() -> Self {
        Self(0b0001_0000)
    }
    #[inline]
    pub fn blinking() -> Self {
        Self(0b0000_1000)
    }
    #[inline]
    pub fn inverse() -> Self {
        Self(0b0000_0100)
    }
    #[inline]
    pub fn hidden() -> Self {
        Self(0b0000_0010)
    }
    #[inline]
    pub fn strikethrough() -> Self {
        Self(0b0000_0001)
    }
    #[inline]
    pub fn with_bold(mut self) -> Self {
        self.0 |= 0b1000_0000;
        self
    }
    #[inline]
    pub fn with_dim(mut self) -> Self {
        self.0 |= 0b0100_0000;
        self
    }
    #[inline]
    pub fn with_italic(mut self) -> Self {
        self.0 |= 0b0010_0000;
        self
    }
    #[inline]
    pub fn with_underline(mut self) -> Self {
        self.0 |= 0b0001_0000;
        self
    }
    #[inline]
    pub fn with_blinking(mut self) -> Self {
        self.0 |= 0b0000_1000;
        self
    }
    #[inline]
    pub fn with_inverse(mut self) -> Self {
        self.0 |= 0b0000_0100;
        self
    }
    #[inline]
    pub fn with_hidden(mut self) -> Self {
        self.0 |= 0b0000_0010;
        self
    }
    #[inline]
    pub fn with_strikethrough(mut self) -> Self {
        self.0 |= 0b0000_0001;
        self
    }
    #[inline]
    pub fn as_u8(&self) -> u8 {
        self.0
    }
    #[inline]
    pub fn reset() -> Self {
        Self(0)
    }
    #[inline]
    pub fn as_bools(&self) -> [bool; 8] {
        [
            self.0 & 0b1000_0000 != 0,
            self.0 & 0b0100_0000 != 0,
            self.0 & 0b0010_0000 != 0,
            self.0 & 0b0001_0000 != 0,
            self.0 & 0b0000_1000 != 0,
            self.0 & 0b0000_0100 != 0,
            self.0 & 0b0000_0010 != 0,
            self.0 & 0b0000_0001 != 0,
        ]
    }
    #[inline]
    pub fn as_ansi(&self) -> String {
        if self.0 == 0 {
            return String::from("\x1b[0m");
        }

        let codes = [
            "\x1b[1m", "\x1b[2m", "\x1b[3m", "\x1b[4m", "\x1b[5m", "\x1b[7m", "\x1b[8m", "\x1b[9m",
        ];

        self.as_bools()
            .iter()
            .enumerate()
            .filter_map(|(i, b)| b.then_some(codes[i]))
            .collect::<Vec<_>>()
            .join("")
    }
}

impl Debug for Modes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names = [
            "BOLD",
            "DIM",
            "ITALIC",
            "UNDERLINE",
            "BLINKING",
            "INVERSE",
            "HIDDEN",
            "STRIKETHROUGH",
        ];
        for (i, d) in self.as_bools().iter().enumerate() {
            if *d {
                write!(f, "[{}]", names[i])?;
            }
        }
        Ok(())
    }
}

impl Display for Modes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ansi())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Style {
    fg: Color,
    bg: Color,
    mode: Modes,
}

impl Style {
    pub fn new() -> Self {
        Self {
            fg: Color::Default,
            bg: Color::Default,
            mode: Modes::new(),
        }
    }
    #[inline]
    pub fn with_fg(mut self, fg: Color) -> Self {
        self.fg = fg;
        self
    }
    #[inline]
    pub fn with_bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }
    #[inline]
    pub fn with_mode(mut self, modes: Modes) -> Self {
        self.mode = modes;
        self
    }
    #[inline]
    pub fn set_fg(&mut self, fg: Color) {
        self.fg = fg;
    }
    #[inline]
    pub fn set_bg(&mut self, bg: Color) {
        self.bg = bg;
    }
    #[inline]
    pub fn mode_mut(&mut self) -> &mut Modes {
        &mut self.mode
    }
    #[inline]
    pub fn as_ansi(&self) -> String {
        format!(
            "{}{}{}",
            self.fg.as_ansi_fg(),
            self.bg.as_ansi_bg(),
            self.mode.as_ansi()
        )
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}
