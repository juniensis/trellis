#![allow(unused)]

use trellis_core::terminal::Cell;
use trellis_graphics::frame::{self, Frame};
use trellis_terminal::event::Event;

use crate::{
    board::Board,
    command::Command,
    entities::{Entity, text_box::TextBox},
    id::Id,
    input::InputStateMachine,
    status::StatusBar,
    viewport::Viewport,
};

pub mod board;
pub mod command;
pub mod entities;
pub mod file;
pub mod id;
pub mod input;
pub mod status;
pub mod viewport;

pub enum State {
    Normal,
    Insert,
}

pub struct Trellis {
    board: Board,
    handler: InputStateMachine,
    state: State,
    viewport: Viewport,
    status: StatusBar,
    cursor_x: i64,
    cursor_y: i64,
}

impl Trellis {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            board: Board::default(),
            handler: InputStateMachine::default(),
            state: State::Normal,
            viewport: Viewport::new(width, height),
            status: {
                let mut bar = StatusBar::new();
                bar.set_max_width(width);
                bar
            },
            cursor_x: 0,
            cursor_y: 0,
        }
    }
    pub fn resize(&mut self, nw: usize, nh: usize) {
        self.viewport.resize(nw as i64, nh as i64);
        self.status.set_max_width(nw as u32);
    }
    pub fn handle_event(&mut self, event: &Event) -> Option<Command> {
        self.status.handle_key(event);
        match self.state {
            State::Normal => self.handler.handle_normal(event),
            State::Insert => todo!(),
        }
    }
    pub fn debug<S: ToString>(&mut self, message: S) {
        self.status.send_message(message);
    }
    pub fn handle_command(&mut self, command: Command) {
        match command {
            Command::NormalCursorLeft(x) => self.cursor_x -= x as i64,
            Command::NormalCursorUp(x) => self.cursor_y += x as i64,
            Command::NormalCursorDown(x) => self.cursor_y -= x as i64,
            Command::NormalCursorRight(x) => self.cursor_x += x as i64,
            Command::NormalInsert => {
                self.debug("insert");
                if let Some(at_cursor) =
                    self.board
                        .try_get_at_cursor(&self.viewport, self.cursor_x, self.cursor_y)
                {
                    self.debug(format!("Found: {}", at_cursor.id));
                } else {
                    self.board.create_textbox(self.cursor_x, self.cursor_y);
                }
            }
            _ => {}
        }
    }
    pub fn draw<'a>(&'a self, mut frame: Frame<'a>) -> Frame<'a> {
        frame.draw((0, self.viewport.h - 1), &self.status);

        for contained in self.board.all_within_viewport(&self.viewport) {
            if let Some(coords) = self
                .viewport
                .translate_world_coords(contained.x, contained.y)
            {
                frame.draw(coords, contained);
            }
        }

        frame
    }
    pub fn transformed_cursor_position(&mut self) -> (u32, u32) {
        self.viewport.follow_cursor(self.cursor_x, self.cursor_y, 1);
        self.viewport
            .translate_world_coords(self.cursor_x, self.cursor_y)
            .unwrap_or((0, 0))
    }
}
