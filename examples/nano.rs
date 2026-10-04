use std::{
    env::{self},
    fs,
    path::PathBuf,
    time::{Duration, Instant},
};

use trellis::{
    Primitive, Renderable, Shell,
    event::{Event, KeyCode},
};
use trellis_core::terminal::style::{Color, Style};
use trellis_graphics::text;

enum Message {
    Exit,
    Save,
}

struct Status {
    x: usize,
    y: usize,
    w: u32,
}

impl Renderable for Status {
    fn render(&self, _ctx: trellis::RenderCtx) -> Primitive {
        let mut status_bar = Primitive::new_region(2);
        status_bar.write_str_styled(
            (0, 0),
            "| CTRL + X: EXIT | CTRL + W: WRITE |",
            Style::new()
                .with_bg(Color::BrightWhite)
                .with_fg(Color::RGB(0, 0, 0)),
        );
        let coords = format!("({}:{})", self.y, self.x);
        status_bar.write_str_styled(
            (self.w - coords.len() as u32, 0),
            coords,
            Style::new()
                .with_bg(Color::BrightWhite)
                .with_fg(Color::RGB(0, 0, 0)),
        );

        status_bar
    }
}

struct Text {
    contents: Vec<String>,
    line: usize,
    column: usize,
    x_offset: usize,
    y_offset: usize,
}

impl Text {
    fn height(&self) -> usize {
        self.contents.len()
    }
    fn current_line_mut(&mut self) -> &mut String {
        if self.contents.len() < self.line {
            self.contents.push(String::new());
        }

        &mut self.contents[self.line.saturating_sub(1)]
    }
    fn current_line_length(&self) -> usize {
        self.contents
            .get(self.line - 1)
            .map(|x| x.len())
            .unwrap_or(0)
    }
    fn cursor_up(&mut self) {
        if self.line > 1 {
            self.line -= 1;
            self.column = self.column.min(self.current_line_length() + 1)
        }
    }
    fn cursor_down(&mut self) {
        if self.line < self.height() {
            self.line += 1;
            self.column = self.column.min(self.current_line_length() + 1)
        }
    }
    fn cursor_left(&mut self) {
        if self.column > 1 {
            self.column -= 1;
        }
    }
    fn cursor_right(&mut self) {
        if self.column <= self.current_line_length() {
            self.column += 1;
        }
    }
    fn insert_char(&mut self, ch: char) {
        let col = self.column;
        if col >= self.current_line_length() {
            self.current_line_mut().push(ch);
        } else {
            self.current_line_mut().insert(col - 1, ch);
        }
        self.cursor_right();
    }
    fn remove_char(&mut self) {
        self.cursor_left();
        if self.current_line_length() > self.column - 1 {
            let col = self.column;
            self.current_line_mut().remove(col - 1);
        }
    }
    fn insert_newline(&mut self) {
        if self.line >= self.contents.len() {
            self.contents.push(String::new());
            self.cursor_down();
        } else {
            let col = self.column;
            let rem = self.current_line_mut().split_at(col - 1);
            let (l, r) = (rem.0.to_string(), rem.1.to_string());
            *self.current_line_mut() = l;
            self.contents.insert(self.line, r);
            self.cursor_down();
            self.column = 1;
        }
    }
    fn text(&self) -> String {
        let mut lines = self.contents.join("\n");
        lines.push('\n');
        lines
    }
    fn from_string(contents: String) -> Self {
        Self {
            contents: contents.lines().map(|x| x.to_string()).collect(),
            line: 1,
            column: 1,
            x_offset: 0,
            y_offset: 0,
        }
    }
    fn x(&self) -> u32 {
        self.column.saturating_sub(self.x_offset) as u32
    }
    fn y(&self) -> u32 {
        self.line.saturating_sub(self.y_offset) as u32
    }
    fn lines_offset(&self) -> impl Iterator<Item = &str> {
        self.contents.iter().skip(self.y_offset).map(|x| {
            if x.len() > self.x_offset {
                &x[self.x_offset..]
            } else {
                x
            }
        })
    }
    fn scroll_down(&mut self, n: usize) {
        self.y_offset = (self.y_offset + n).min(self.contents.len());
    }
    fn scroll_up(&mut self, n: usize) {
        self.y_offset = self.y_offset.saturating_sub(n);
    }
    fn scroll_right(&mut self, n: usize) {
        self.x_offset = (self.x_offset + n).min(self.current_line_length());
    }
    fn scroll_left(&mut self, n: usize) {
        self.x_offset = self.x_offset.saturating_sub(n);
    }
    fn scroll_to_fit(&mut self, width: u32, height: u32) {
        while self.x() > width.saturating_sub(5) {
            self.scroll_right(1);
        }

        while self.x() < 5 && self.x_offset > 0 {
            self.scroll_left(1);
        }

        while self.y() > height.saturating_sub(5) {
            self.scroll_down(1);
        }

        while self.y() < 5 && self.y_offset > 0 {
            self.scroll_up(1);
        }
    }
}

struct Nano {
    text: Text,
    status: Status,
    width: u32,
    height: u32,
    path: PathBuf,
}

impl Nano {
    fn new(width: u32, height: u32, path: PathBuf, initial: String) -> Self {
        Self {
            text: Text::from_string(initial),
            status: Status {
                x: 1,
                y: 1,
                w: width,
            },
            width,
            height,
            path,
        }
    }
    fn update(&mut self) {
        self.status.x = self.text.column;
        self.status.y = self.text.line;
        self.status.w = self.width;
        self.text.scroll_to_fit(self.width, self.height);
    }
    fn handle(&mut self, e: &Event) -> Option<Message> {
        if let Event::Key { code, modifiers } = e {
            match code {
                KeyCode::Up => self.text.cursor_up(),
                KeyCode::Down => self.text.cursor_down(),
                KeyCode::Left => self.text.cursor_left(),
                KeyCode::Right => self.text.cursor_right(),
                KeyCode::Enter => self.text.insert_newline(),
                KeyCode::Backspace => self.text.remove_char(),
                KeyCode::Char(ch) => {
                    if modifiers.is_ctrl() {
                        match ch {
                            'x' => return Some(Message::Exit),
                            'w' => return Some(Message::Save),
                            _ => {}
                        }
                    } else {
                        self.text.insert_char(*ch);
                    }
                }
                KeyCode::Tab => {
                    for _ in 0..4 {
                        self.text.insert_char(' ');
                    }
                }
                _ => {}
            }
        } else if let Event::Resized(nw, nh) = e {
            self.width = *nw as u32;
            self.height = *nh as u32;
        }
        None
    }
}

impl Renderable for Text {
    fn render(&self, _ctx: trellis::RenderCtx) -> trellis::Primitive {
        let mut ret = Primitive::new_region(1);

        for (idx, line) in self.lines_offset().enumerate() {
            ret.write_str((0, idx), line);
        }

        ret
    }
}

fn main() {
    let args = env::args().nth(1).expect("Provide path to edit or create.");

    let pth = PathBuf::from(args);
    if !pth.exists() {
        fs::write(&pth, b"").expect("Invalid path given.");
    }

    let initial = fs::read_to_string(&pth).expect("Invalid path given.");
    let mut shell = Shell::new_crossterm();
    let mut nano = Nano::new(shell.width(), shell.height(), pth, initial);

    let mut cooldown = 0;
    'running: while let Some(tick) = shell.tick() {
        let now = Instant::now();
        for event in tick.events() {
            if let Some(msg) = nano.handle(event) {
                match msg {
                    Message::Exit => break 'running,
                    Message::Save => {
                        fs::write(&nano.path, nano.text.text().as_bytes())
                            .expect("Failed to save.");
                        cooldown = 240;
                    }
                }
            }
        }

        let height = shell.height();

        nano.update();
        shell.show_cursor();
        shell
            .viewport()
            .move_cursor_to((nano.text.x() - 1, nano.text.y() - 1));

        let mut frame = shell
            .start_frame()
            .with_draw((0, 0), &nano.text)
            .with_draw((0, height - 1), &nano.status);

        if cooldown > 0 {
            frame.draw(
                (0, height - 2),
                &text::Text::new(format!("Saved to {:?}", nano.path.as_path()), 3),
            );
            cooldown -= 1;
        }
        frame.submit();

        while now.elapsed() < Duration::from_secs_f32(1.0 / 240.0) {}
    }
}
