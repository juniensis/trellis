use std::time::{Duration, Instant};

use trellis_core::terminal::style::{Color, Style};
use trellis_graphics::{
    primitives::{Primitive, Renderable},
    shell::Shell,
};
use trellis_terminal::event::{Event, KeyCode};

pub enum Message {
    Exit,
    Save,
}

pub struct TextEditor {
    line: u32,
    column: u32,
    contents: Vec<String>,
    width: u32,
    height: u32,
}

impl TextEditor {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            line: 1,
            column: 1,
            contents: Vec::new(),
            width,
            height,
        }
    }
    pub fn handle(&mut self, event: &Event) -> Option<Message> {
        if let Event::Key { code, modifiers } = event {
            match code {
                KeyCode::Up => self.line = self.line.saturating_sub(1).max(1),
                KeyCode::Down => self.line = (self.line + 1).min(self.contents.len() as u32),
                KeyCode::Left => self.column = self.column.saturating_sub(1).max(1),
                KeyCode::Right => {
                    self.column =
                        (self.column + 1).min(self.contents[self.line as usize - 1].len() as u32)
                }
                KeyCode::Enter => {
                    if self.line >= self.contents.len() as u32 {
                        self.contents.push(String::new());
                    } else {
                        self.contents.insert(self.line as usize, String::new());
                    }

                    self.line = (self.line + 1).min(self.contents.len() as u32);
                }
                KeyCode::Backspace => {
                    self.column = self.column.saturating_sub(1).max(1);
                    if self.column < self.contents[(self.line - 1) as usize].len() as u32 {
                        self.contents[(self.line - 1) as usize].remove(self.column as usize - 1);
                    } else {
                        self.contents[(self.line - 1) as usize].pop();
                    }
                }
                _ => {
                    if modifiers.is_ctrl() {
                        match code {
                            KeyCode::Char('x') => return Some(Message::Exit),
                            KeyCode::Char('s') => return Some(Message::Save),
                            _ => {}
                        }
                    } else if let KeyCode::Char(ch) = code {
                        if self.contents.len() < self.line as usize {
                            self.contents.push(String::new());
                        }
                        if self.contents[self.line as usize - 1].len() < self.column as usize {
                            self.contents[self.line as usize - 1].push(*ch);
                        } else {
                            self.contents[self.line as usize - 1]
                                .insert(self.column as usize - 1, *ch);
                        }
                        self.column += 1;
                    }
                }
            }
        } else if let Event::Resized(nw, nh) = event {
            self.width = *nw as u32;
            self.height = *nh as u32;
        }
        None
    }
}

pub struct StatusBar {
    x: u32,
    y: u32,
    w: u32,
}

impl Renderable for StatusBar {
    fn render(&self, _ctx: trellis_graphics::primitives::RenderCtx) -> Primitive {
        let mut status_bar = Primitive::new_region(2);
        status_bar.write_str_styled(
            (0, 0),
            "| CTRL + X: EXIT | CTRL + S: SAVE |",
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

impl Renderable for TextEditor {
    fn render(
        &self,
        _ctx: trellis_graphics::primitives::RenderCtx,
    ) -> trellis_graphics::primitives::Primitive {
        let mut ret = Primitive::new_region(1);

        for line in self.contents.iter().enumerate() {
            ret.write_str((0, line.0), line.1);
        }

        ret
    }
}

fn main() {
    let mut shell = Shell::new_crossterm();
    let mut editor = TextEditor::new(shell.width(), shell.height());
    let mut status = StatusBar {
        x: 1,
        y: 1,
        w: shell.width(),
    };
    editor.contents.push("Hello".to_string());
    editor.contents.push("World".to_string());
    editor.contents.push("!".to_string());

    'running: while let Some(tick) = shell.tick() {
        let now = Instant::now();
        for event in tick.events() {
            if let Some(cmd) = editor.handle(event) {
                match cmd {
                    Message::Exit => break 'running,
                    Message::Save => todo!(),
                }
            }
        }

        status.x = editor.column;
        status.y = editor.line;
        let height = shell.height();
        shell
            .viewport()
            .move_cursor_to((status.x - 1, status.y - 1));

        let mut frame = shell.start_frame();
        frame.draw((0, height - 1), &status);
        frame.draw((0, 0), &editor);
        frame.submit();

        while now.elapsed() < Duration::from_secs_f32(1.0 / 240.0) {}
    }
}
