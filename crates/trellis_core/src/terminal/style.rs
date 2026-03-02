use std::fmt;

/// An abstraction of ANSI colors. Assumes truecolor support, for compatability
/// with non-truecolor terminals, colors must be explicitly converted upstream.
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
    /// Get the ANSI escape sequence required to set the foreground to the
    /// color in question.
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
    /// Get the ANSI escape sequence required to set the background to the
    /// color in question.
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

/// A structure representing currently active ANSI modes.
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
    pub fn set_bold(&mut self) {
        self.0 |= 0b1000_0000;
    }
    #[inline]
    pub fn set_dim(&mut self) {
        self.0 |= 0b0100_0000;
    }
    #[inline]
    pub fn set_italic(&mut self) {
        self.0 |= 0b0010_0000;
    }
    #[inline]
    pub fn set_underline(&mut self) {
        self.0 |= 0b0001_0000;
    }
    #[inline]
    pub fn set_blinking(&mut self) {
        self.0 |= 0b0000_1000;
    }
    #[inline]
    pub fn set_inverse(&mut self) {
        self.0 |= 0b0000_0100;
    }
    #[inline]
    pub fn set_hidden(&mut self) {
        self.0 |= 0b0000_0010;
    }
    #[inline]
    pub fn set_strikethrough(&mut self) {
        self.0 |= 0b0000_0001;
    }
    #[inline]
    pub fn unset_bold(&mut self) {
        self.0 &= 0b0111_1111;
    }
    #[inline]
    pub fn unset_dim(&mut self) {
        self.0 &= 0b1011_1111;
    }
    #[inline]
    pub fn unset_italic(&mut self) {
        self.0 &= 0b1101_1111;
    }
    #[inline]
    pub fn unset_underline(&mut self) {
        self.0 &= 0b1110_1111;
    }
    #[inline]
    pub fn unset_blinking(&mut self) {
        self.0 &= 0b1111_0111;
    }
    #[inline]
    pub fn unset_inverse(&mut self) {
        self.0 &= 0b1111_1011;
    }
    #[inline]
    pub fn unset_hidden(&mut self) {
        self.0 &= 0b1111_1101;
    }
    #[inline]
    pub fn unset_strikethrough(&mut self) {
        self.0 &= 0b1111_1110;
    }
    #[inline]
    pub fn is_bold(&self) -> bool {
        self.0 & 0b1000_0000 != 0
    }
    #[inline]
    pub fn is_dim(&self) -> bool {
        self.0 & 0b0100_0000 != 0
    }
    #[inline]
    pub fn is_italic(&self) -> bool {
        self.0 & 0b0010_0000 != 0
    }
    #[inline]
    pub fn is_underline(&self) -> bool {
        self.0 & 0b0001_0000 != 0
    }
    #[inline]
    pub fn is_blinking(&self) -> bool {
        self.0 & 0b0000_1000 != 0
    }
    #[inline]
    pub fn is_inverse(&self) -> bool {
        self.0 & 0b0000_0100 != 0
    }
    #[inline]
    pub fn is_hidden(&self) -> bool {
        self.0 & 0b0000_0010 != 0
    }
    #[inline]
    pub fn is_strikethrough(&self) -> bool {
        self.0 & 0b0000_0001 != 0
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

impl fmt::Debug for Modes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl fmt::Display for Modes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ansi())
    }
}

/// A structure representing a foreground, background, and active modes
/// (bold, dim, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Style {
    fg: Color,
    bg: Color,
    modes: Modes,
}

impl Style {
    /// Creates a new style with all defaults, displaying this will reset
    /// all colors and modes active in the terminal. Use the 'with_*' builders
    /// to assign colors and modes, or initialize mutably and use the 'set_*'
    /// functions to change the colors/modes.
    pub fn new() -> Self {
        Self {
            fg: Color::Default,
            bg: Color::Default,
            modes: Modes::new(),
        }
    }
    #[inline]
    pub fn with_colors(mut self, fg: Color, bg: Color) -> Self {
        self.fg = fg;
        self.bg = bg;
        self
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
    pub fn with_modes(mut self, modes: Modes) -> Self {
        self.modes = modes;
        self
    }
    #[inline]
    pub fn with_bold(mut self) -> Self {
        self.modes.set_bold();
        self
    }
    #[inline]
    pub fn with_dim(mut self) -> Self {
        self.modes.set_dim();
        self
    }
    #[inline]
    pub fn with_italic(mut self) -> Self {
        self.modes.set_italic();
        self
    }
    #[inline]
    pub fn with_underline(mut self) -> Self {
        self.modes.set_underline();
        self
    }
    #[inline]
    pub fn with_blinking(mut self) -> Self {
        self.modes.set_blinking();
        self
    }
    #[inline]
    pub fn with_inverse(mut self) -> Self {
        self.modes.set_inverse();
        self
    }
    #[inline]
    pub fn with_hidden(mut self) -> Self {
        self.modes.set_hidden();
        self
    }
    #[inline]
    pub fn with_strikethrough(mut self) -> Self {
        self.modes.set_strikethrough();
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
    pub fn set_modes(&mut self, modes: Modes) {
        self.modes = modes;
    }
    #[inline]
    pub fn set_bold(&mut self) {
        self.modes.set_bold();
    }
    #[inline]
    pub fn set_dim(&mut self) {
        self.modes.set_dim();
    }
    #[inline]
    pub fn set_italic(&mut self) {
        self.modes.set_italic();
    }
    #[inline]
    pub fn set_underline(&mut self) {
        self.modes.set_underline();
    }
    #[inline]
    pub fn set_blinking(&mut self) {
        self.modes.set_blinking();
    }
    #[inline]
    pub fn set_inverse(&mut self) {
        self.modes.set_inverse();
    }
    #[inline]
    pub fn set_hidden(&mut self) {
        self.modes.set_hidden();
    }
    #[inline]
    pub fn set_strikethrough(&mut self) {
        self.modes.set_strikethrough();
    }
    #[inline]
    pub fn modes(&self) -> &Modes {
        &self.modes
    }
    #[inline]
    pub fn modes_mut(&mut self) -> &mut Modes {
        &mut self.modes
    }
    #[inline]
    pub fn as_ansi(&self) -> String {
        format!(
            "{}{}{}",
            self.modes.as_ansi(),
            self.fg.as_ansi_fg(),
            self.bg.as_ansi_bg(),
        )
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Stylable<T> {
    fn stylize(&self, style: Style) -> T;
}
