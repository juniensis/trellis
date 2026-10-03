#![allow(unused)]

use std::collections::HashSet;

use trellis_graphics::frame::Frame;
use trellis_terminal::event::Event;

use crate::{
    commands::Command,
    file::TrellisFile,
    input::{InputStateMachine, State},
    ui::status::StatusBar,
    world::{board::Board, id::Id, viewport::Viewport},
};

pub mod block;
pub mod commands;
pub mod connector;
pub mod file;
pub mod input;
pub mod ui;
pub mod world;

pub struct Trellis {
    board: Board,
    handler: InputStateMachine,
    state: State,
    viewport: Viewport,
    status: StatusBar,
    cursor_x: i64,
    cursor_y: i64,
    focused: HashSet<Id>,
}

impl Trellis {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            board: Board::default(),
            handler: InputStateMachine::default(),
            state: State::Normal,
            viewport: Viewport::new(width, height),
            status: StatusBar::new(0, 0, width),
            cursor_x: 0,
            cursor_y: 0,
            focused: HashSet::new(),
        }
    }
    pub fn handle_event(&mut self, event: &Event) -> Option<Command> {
        self.handler.handle(event)
    }
    pub fn handle_command(&mut self, command: Command) {
        match command {
            Command::NormalCursorLeft(x) => self.cursor_x -= x as i64,
            Command::NormalCursorDown(x) => self.cursor_y -= x as i64,
            Command::NormalCursorUp(x) => self.cursor_y += x as i64,
            Command::NormalCursorRight(x) => self.cursor_x += x as i64,
            Command::NormalInsert => {
                if let Some(hovered) = self
                    .board
                    .all_within_viewport(&self.viewport)
                    .find(|&x| x.contains(self.cursor_x, self.cursor_y))
                {}
            }
            Command::NormalEnter => {}
            _ => {}
        }
    }
    pub fn update_status(&mut self) {
        self.status.set_state(self.state);
        self.status.set_max_width(self.viewport.w as u32);
        self.status.set_coords(self.cursor_x, self.cursor_y);
    }
    pub fn resize(&mut self, nw: u32, nh: u32) {
        self.viewport.w = nw as i64;
        self.viewport.h = nh as i64;
        self.status.set_max_width(nw);
    }
    pub fn draw<'a>(&'a self, mut frame: Frame<'a>) -> Frame<'a> {
        frame.draw((0, self.viewport.h - 1), &self.status);

        for contained in self.board.all_within_viewport(&self.viewport) {
            if let Some(coords) = self
                .viewport
                .translate_world_coords(contained.x(), contained.y())
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
