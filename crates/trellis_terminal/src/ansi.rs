use std::fmt;

pub const ESC: &str = "\x1b";
pub const CSI: &str = "\x1b[";
pub const DCS: &str = "\x1bP";
pub const OSC: &str = "\x1b]";
pub const BEL: &str = "\x07";
pub const BS: &str = "\x08";
pub const HT: &str = "\x09";
pub const LF: &str = "\n";
pub const VT: &str = "\x0b";
pub const FF: &str = "\x0c";
pub const CR: &str = "\x0d";
pub const DEL: &str = "\x7f";
pub const HOME: &str = "\x1b[H";
pub const REQCP: &str = "\x1b[6n";
pub const ONE_UP: &str = "\x1bM";
pub const SV_DEC: &str = "\x1b7";
pub const RSTR_DEC: &str = "\x1b8";
pub const SV_SCO: &str = "\x1b[s";
pub const RSTR_SCO: &str = "\x1b[u";
pub const CL_CTE: &str = "\x1b[0J";
pub const CL_CTB: &str = "\x1b[1J";
pub const CL_ALL: &str = "\x1b[2J";
pub const CL_SVD: &str = "\x1b[3J";
pub const CL_CTN: &str = "\x1b[0K";
pub const CL_STC: &str = "\x1b[1K";
pub const CL_LNE: &str = "\x1b[2K";
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const ITALIC: &str = "\x1b[3m";
pub const UNDER: &str = "\x1b[4m";
pub const BLINK: &str = "\x1b[5m";
pub const REV: &str = "\x1b[7m";
pub const HIDE: &str = "\x1b[8m";
pub const STRIKE: &str = "\x1b[9m";
pub const BOLD_R: &str = "\x1b[21m";
pub const DIM_R: &str = "\x1b[22m";
pub const ITALIC_R: &str = "\x1b[23m";
pub const UNDER_R: &str = "\x1b[24m";
pub const BLINK_R: &str = "\x1b[25m";
pub const REV_R: &str = "\x1b[27m";
pub const HIDE_R: &str = "\x1b[28m";
pub const STRIKE_R: &str = "\x1b[29m";
pub const BLACK_FG: &str = "\x1b[30m";
pub const RED_FG: &str = "\x1b[31m";
pub const GREEN_FG: &str = "\x1b[32m";
pub const YELLOW_FG: &str = "\x1b[33m";
pub const BLUE_FG: &str = "\x1b[34m";
pub const MAGENTA_FG: &str = "\x1b[35m";
pub const CYAN_FG: &str = "\x1b[36m";
pub const WHITE_FG: &str = "\x1b[37m";
pub const DEFAULT_FG: &str = "\x1b[39m";
pub const BLACK_BG: &str = "\x1b[40m";
pub const RED_BG: &str = "\x1b[41m";
pub const GREEN_BG: &str = "\x1b[42m";
pub const YELLOW_BG: &str = "\x1b[43m";
pub const BLUE_BG: &str = "\x1b[44m";
pub const MAGENTA_BG: &str = "\x1b[45m";
pub const CYAN_BG: &str = "\x1b[46m";
pub const WHITE_BG: &str = "\x1b[47m";
pub const DEFAULT_BG: &str = "\x1b[49m";
pub const BRIGHT_BLACK_FG: &str = "\x1b[90m";
pub const BRIGHT_RED_FG: &str = "\x1b[91m";
pub const BRIGHT_GREEN_FG: &str = "\x1b[92m";
pub const BRIGHT_YELLOW_FG: &str = "\x1b[93m";
pub const BRIGHT_BLUE_FG: &str = "\x1b[94m";
pub const BRIGHT_MAGENTA_FG: &str = "\x1b[95m";
pub const BRIGHT_CYAN_FG: &str = "\x1b[96m";
pub const BRIGHT_WHITE_FG: &str = "\x1b[97m";
pub const BRIGHT_BLACK_BG: &str = "\x1b[100m";
pub const BRIGHT_RED_BG: &str = "\x1b[101m";
pub const BRIGHT_GREEN_BG: &str = "\x1b[102m";
pub const BRIGHT_YELLOW_BG: &str = "\x1b[103m";
pub const BRIGHT_BLUE_BG: &str = "\x1b[104m";
pub const BRIGHT_MAGENTA_BG: &str = "\x1b[105m";
pub const BRIGHT_CYAN_BG: &str = "\x1b[106m";
pub const BRIGHT_WHITE_BG: &str = "\x1b[107m";
pub const CURSOR_INVISIBLE: &str = "\x1b[?25l";
pub const CURSOR_VISIBLE: &str = "\x1b[?25h";
pub const SCREEN_RESTORE: &str = "\x1b[?47l";
pub const SCREEN_SAVE: &str = "\x1b[?47h";
pub const ALT_BUFFER_ENABLE: &str = "\x1b[?1049h";
pub const ALT_BUFFER_DISABLE: &str = "\x1b[?1049l";
pub const ENABLE_LINE_WRAP: &str = "\x1b[=7h";
pub const MODE_40X25_MONO_TEXT: &str = "\x1b[=0h";
pub const MODE_40X25_COLOR_TEXT: &str = "\x1b[=1h";
pub const MODE_80X25_MONO_TEXT: &str = "\x1b[=2h";
pub const MODE_80X25_COLOR_TEXT: &str = "\x1b[=3h";
pub const MODE_320X200_4COLOR: &str = "\x1b[=4h";
pub const MODE_320X200_MONO: &str = "\x1b[=5h";
pub const MODE_640X200_MONO: &str = "\x1b[=6h";
pub const MODE_320X200_COLOR: &str = "\x1b[=13h";
pub const MODE_640X200_16COLOR: &str = "\x1b[=14h";
pub const MODE_640X350_MONO: &str = "\x1b[=15h";
pub const MODE_640X350_16COLOR: &str = "\x1b[=16h";
pub const MODE_640X480_MONO: &str = "\x1b[=17h";
pub const MODE_640X480_16COLOR: &str = "\x1b[=18h";
pub const MODE_320X200_256COLOR: &str = "\x1b[=19h";

#[macro_export]
macro_rules! color_id_fg {
    ($n:literal) => {{
        const _: () = assert!($n > 0);
        concat!("\x1b[38;5;", $n, "m")
    }};
}
#[macro_export]
macro_rules! color_id_bg {
    ($n:literal) => {{
        const _: () = assert!($n > 0);
        concat!("\x1b[48;5;", $n, "m")
    }};
}
#[macro_export]
macro_rules! rgb_fg {
    ($r:literal, $g:literal, $b: literal) => {{
        const _: () = assert!($r > 0);
        concat!("\x1b[38;2;", $r, ";", $g, ";", $b, "m")
    }};
}
#[macro_export]
macro_rules! rgb_bg {
    ($r:literal, $g:literal, $b: literal) => {{
        const _: () = assert!($r > 0);
        concat!("\x1b[48;2;", $r, ";", $g, ";", $b, "m")
    }};
}
#[macro_export]
macro_rules! cursor_up {
    ($n:literal) => {{
        const _: () = assert!($n > 0);
        concat!("\x1b[", $n, "A")
    }};
}
#[macro_export]
macro_rules! cursor_down {
    ($n:literal) => {{
        const _: () = assert!($n > 0);
        concat!("\x1b[", $n, "B")
    }};
}
#[macro_export]
macro_rules! cursor_right {
    ($n:literal) => {{
        const _: () = assert!($n > 0);
        concat!("\x1b[", $n, "C")
    }};
}
#[macro_export]
macro_rules! cursor_left {
    ($n:literal) => {{
        const _: () = assert!($n > 0);
        concat!("\x1b[", $n, "D")
    }};
}
#[macro_export]
macro_rules! cursor_next {
    ($n:literal) => {{
        const _: () = assert!($n > 0);
        concat!("\x1b[", $n, "E")
    }};
}
#[macro_export]
macro_rules! cursor_last {
    ($n:literal) => {{
        const _: () = assert!($n > 0);
        concat!("\x1b[", $n, "F")
    }};
}
#[macro_export]
macro_rules! cursor_col {
    ($n:literal) => {{
        const _: () = assert!($n > 0);
        concat!("\x1b[", $n, "G")
    }};
}
#[macro_export]
macro_rules! cursor_move {
    ($l:literal, $c:literal) => {{
        const _: () = assert!($l > 0);
        const _: () = assert!($c > 0);
        concat!("\x1b[", $n, ";", $c, "f")
    }};
}

//
// The following was converted from the constants by an LLM.
// Scrutinized and verified by a certified meat bag, blame me if breaks.
//

/// All ANSI escape sequences.
pub enum EscapeCode {
    Esc,
    Csi,
    Dcs,
    Osc,
    Bel,
    Bs,
    Ht,
    Lf,
    Vt,
    Ff,
    Cr,
    Del,
    Home,
    ReqCursorPos,
    OneUp,
    SaveDec,
    RestoreDec,
    SaveSco,
    RestoreSco,
    CursorUp(u16),
    CursorDown(u16),
    CursorRight(u16),
    CursorLeft(u16),
    CursorNextLine(u16),
    CursorLastLine(u16),
    CursorCol(u16),
    CursorMove(u16, u16),
    CursorVisible,
    CursorInvisible,
    ClearToEnd,
    ClearToBegin,
    ClearAll,
    ClearSaved,
    ClearToLineEnd,
    ClearToLineStart,
    ClearLine,
    Reset,
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hide,
    Strikethrough,
    BoldReset,
    DimReset,
    ItalicReset,
    UnderlineReset,
    BlinkReset,
    ReverseReset,
    HideReset,
    StrikethroughReset,
    BlackFg,
    RedFg,
    GreenFg,
    YellowFg,
    BlueFg,
    MagentaFg,
    CyanFg,
    WhiteFg,
    DefaultFg,
    BlackBg,
    RedBg,
    GreenBg,
    YellowBg,
    BlueBg,
    MagentaBg,
    CyanBg,
    WhiteBg,
    DefaultBg,
    BrightBlackFg,
    BrightRedFg,
    BrightGreenFg,
    BrightYellowFg,
    BrightBlueFg,
    BrightMagentaFg,
    BrightCyanFg,
    BrightWhiteFg,
    BrightBlackBg,
    BrightRedBg,
    BrightGreenBg,
    BrightYellowBg,
    BrightBlueBg,
    BrightMagentaBg,
    BrightCyanBg,
    BrightWhiteBg,
    ColorIdFg(u8),
    ColorIdBg(u8),
    RgbFg(u8, u8, u8),
    RgbBg(u8, u8, u8),
    ScreenSave,
    ScreenRestore,
    AltBufferEnable,
    AltBufferDisable,
    EnableLineWrap,
    Mode40x25MonoText,
    Mode40x25ColorText,
    Mode80x25MonoText,
    Mode80x25ColorText,
    Mode320x200_4Color,
    Mode320x200Mono,
    Mode640x200Mono,
    Mode320x200Color,
    Mode640x200_16Color,
    Mode640x350Mono,
    Mode640x350_16Color,
    Mode640x480Mono,
    Mode640x480_16Color,
    Mode320x200_256Color,
}

impl fmt::Display for EscapeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Esc => f.write_str(ESC),
            Self::Csi => f.write_str(CSI),
            Self::Dcs => f.write_str(DCS),
            Self::Osc => f.write_str(OSC),
            Self::Bel => f.write_str(BEL),
            Self::Bs => f.write_str(BS),
            Self::Ht => f.write_str(HT),
            Self::Lf => f.write_str(LF),
            Self::Vt => f.write_str(VT),
            Self::Ff => f.write_str(FF),
            Self::Cr => f.write_str(CR),
            Self::Del => f.write_str(DEL),
            Self::Home => f.write_str(HOME),
            Self::ReqCursorPos => f.write_str(REQCP),
            Self::OneUp => f.write_str(ONE_UP),
            Self::SaveDec => f.write_str(SV_DEC),
            Self::RestoreDec => f.write_str(RSTR_DEC),
            Self::SaveSco => f.write_str(SV_SCO),
            Self::RestoreSco => f.write_str(RSTR_SCO),
            Self::CursorUp(n) => write!(f, "\x1b[{n}A"),
            Self::CursorDown(n) => write!(f, "\x1b[{n}B"),
            Self::CursorRight(n) => write!(f, "\x1b[{n}C"),
            Self::CursorLeft(n) => write!(f, "\x1b[{n}D"),
            Self::CursorNextLine(n) => write!(f, "\x1b[{n}E"),
            Self::CursorLastLine(n) => write!(f, "\x1b[{n}F"),
            Self::CursorCol(n) => write!(f, "\x1b[{n}G"),
            Self::CursorMove(l, c) => write!(f, "\x1b[{l};{c}f"),
            Self::CursorVisible => f.write_str(CURSOR_VISIBLE),
            Self::CursorInvisible => f.write_str(CURSOR_INVISIBLE),
            Self::ClearToEnd => f.write_str(CL_CTE),
            Self::ClearToBegin => f.write_str(CL_CTB),
            Self::ClearAll => f.write_str(CL_ALL),
            Self::ClearSaved => f.write_str(CL_SVD),
            Self::ClearToLineEnd => f.write_str(CL_CTN),
            Self::ClearToLineStart => f.write_str(CL_STC),
            Self::ClearLine => f.write_str(CL_LNE),
            Self::Reset => f.write_str(RESET),
            Self::Bold => f.write_str(BOLD),
            Self::Dim => f.write_str(DIM),
            Self::Italic => f.write_str(ITALIC),
            Self::Underline => f.write_str(UNDER),
            Self::Blink => f.write_str(BLINK),
            Self::Reverse => f.write_str(REV),
            Self::Hide => f.write_str(HIDE),
            Self::Strikethrough => f.write_str(STRIKE),
            Self::BoldReset => f.write_str(BOLD_R),
            Self::DimReset => f.write_str(DIM_R),
            Self::ItalicReset => f.write_str(ITALIC_R),
            Self::UnderlineReset => f.write_str(UNDER_R),
            Self::BlinkReset => f.write_str(BLINK_R),
            Self::ReverseReset => f.write_str(REV_R),
            Self::HideReset => f.write_str(HIDE_R),
            Self::StrikethroughReset => f.write_str(STRIKE_R),
            Self::BlackFg => f.write_str(BLACK_FG),
            Self::RedFg => f.write_str(RED_FG),
            Self::GreenFg => f.write_str(GREEN_FG),
            Self::YellowFg => f.write_str(YELLOW_FG),
            Self::BlueFg => f.write_str(BLUE_FG),
            Self::MagentaFg => f.write_str(MAGENTA_FG),
            Self::CyanFg => f.write_str(CYAN_FG),
            Self::WhiteFg => f.write_str(WHITE_FG),
            Self::DefaultFg => f.write_str(DEFAULT_FG),
            Self::BlackBg => f.write_str(BLACK_BG),
            Self::RedBg => f.write_str(RED_BG),
            Self::GreenBg => f.write_str(GREEN_BG),
            Self::YellowBg => f.write_str(YELLOW_BG),
            Self::BlueBg => f.write_str(BLUE_BG),
            Self::MagentaBg => f.write_str(MAGENTA_BG),
            Self::CyanBg => f.write_str(CYAN_BG),
            Self::WhiteBg => f.write_str(WHITE_BG),
            Self::DefaultBg => f.write_str(DEFAULT_BG),
            Self::BrightBlackFg => f.write_str(BRIGHT_BLACK_FG),
            Self::BrightRedFg => f.write_str(BRIGHT_RED_FG),
            Self::BrightGreenFg => f.write_str(BRIGHT_GREEN_FG),
            Self::BrightYellowFg => f.write_str(BRIGHT_YELLOW_FG),
            Self::BrightBlueFg => f.write_str(BRIGHT_BLUE_FG),
            Self::BrightMagentaFg => f.write_str(BRIGHT_MAGENTA_FG),
            Self::BrightCyanFg => f.write_str(BRIGHT_CYAN_FG),
            Self::BrightWhiteFg => f.write_str(BRIGHT_WHITE_FG),
            Self::BrightBlackBg => f.write_str(BRIGHT_BLACK_BG),
            Self::BrightRedBg => f.write_str(BRIGHT_RED_BG),
            Self::BrightGreenBg => f.write_str(BRIGHT_GREEN_BG),
            Self::BrightYellowBg => f.write_str(BRIGHT_YELLOW_BG),
            Self::BrightBlueBg => f.write_str(BRIGHT_BLUE_BG),
            Self::BrightMagentaBg => f.write_str(BRIGHT_MAGENTA_BG),
            Self::BrightCyanBg => f.write_str(BRIGHT_CYAN_BG),
            Self::BrightWhiteBg => f.write_str(BRIGHT_WHITE_BG),
            Self::ColorIdFg(n) => write!(f, "\x1b[38;5;{n}m"),
            Self::ColorIdBg(n) => write!(f, "\x1b[48;5;{n}m"),
            Self::RgbFg(r, g, b) => write!(f, "\x1b[38;2;{r};{g};{b}m"),
            Self::RgbBg(r, g, b) => write!(f, "\x1b[48;2;{r};{g};{b}m"),
            Self::ScreenSave => f.write_str(SCREEN_SAVE),
            Self::ScreenRestore => f.write_str(SCREEN_RESTORE),
            Self::AltBufferEnable => f.write_str(ALT_BUFFER_ENABLE),
            Self::AltBufferDisable => f.write_str(ALT_BUFFER_DISABLE),
            Self::EnableLineWrap => f.write_str(ENABLE_LINE_WRAP),
            Self::Mode40x25MonoText => f.write_str(MODE_40X25_MONO_TEXT),
            Self::Mode40x25ColorText => f.write_str(MODE_40X25_COLOR_TEXT),
            Self::Mode80x25MonoText => f.write_str(MODE_80X25_MONO_TEXT),
            Self::Mode80x25ColorText => f.write_str(MODE_80X25_COLOR_TEXT),
            Self::Mode320x200_4Color => f.write_str(MODE_320X200_4COLOR),
            Self::Mode320x200Mono => f.write_str(MODE_320X200_MONO),
            Self::Mode640x200Mono => f.write_str(MODE_640X200_MONO),
            Self::Mode320x200Color => f.write_str(MODE_320X200_COLOR),
            Self::Mode640x200_16Color => f.write_str(MODE_640X200_16COLOR),
            Self::Mode640x350Mono => f.write_str(MODE_640X350_MONO),
            Self::Mode640x350_16Color => f.write_str(MODE_640X350_16COLOR),
            Self::Mode640x480Mono => f.write_str(MODE_640X480_MONO),
            Self::Mode640x480_16Color => f.write_str(MODE_640X480_16COLOR),
            Self::Mode320x200_256Color => f.write_str(MODE_320X200_256COLOR),
        }
    }
}
