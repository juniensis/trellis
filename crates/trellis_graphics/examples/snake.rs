use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use trellis_core::{primitives::Pos, terminal::Cell};
use trellis_graphics::{
    primitives::{RenderCtx, Renderable, Scatter},
    shell::Shell,
};
use trellis_terminal::event::{Event, KeyCode};

struct SnakeState {
    x: i16,
    y: i16,
    positions: VecDeque<Pos>,
    dx: i16,
    dy: i16,
    length: u16,
    width: u32,
    height: u32,
}

impl SnakeState {
    pub fn new(initial: Pos, width: u32, height: u32) -> Self {
        let mut positions = VecDeque::new();
        positions.push_front(initial);
        Self {
            x: 10,
            y: 10,
            positions,
            dx: 0,
            dy: 0,
            length: 16,
            width,
            height,
        }
    }
    pub fn handle_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up => {
                self.dy = -1;
                self.dx = 0;
            }
            KeyCode::Down => {
                self.dy = 1;
                self.dx = 0;
            }
            KeyCode::Left => {
                self.dx = -1;
                self.dy = 0;
            }
            KeyCode::Right => {
                self.dx = 1;
                self.dy = 0;
            }
            _ => {}
        }
    }
    pub fn update(&mut self) {
        let new_x = self.x + self.dx;
        let new_y = self.y + self.dy;

        self.x = new_x;
        self.y = new_y;

        if new_x < 0 {
            self.x = self.width as i16 - 1;
        }
        if new_y < 0 {
            self.y = self.height as i16 - 1;
        }
        if new_x == self.width as i16 {
            self.x = 0;
        }
        if new_y == self.height as i16 {
            self.y = 0;
        }

        self.positions.push_front((self.x, self.y).into());

        while self.positions.len() > self.length as usize {
            self.positions.pop_back();
        }
    }
}

impl Renderable for SnakeState {
    fn render(&self, _ctx: RenderCtx) -> trellis_graphics::primitives::Primitive {
        let mut scatter = Scatter::new(0);

        for pos in self.positions.iter() {
            scatter.insert(*pos, Cell::new('*'));
        }

        scatter.into()
    }
}

fn main() {
    let mut shell = Shell::new_crossterm();
    shell.hide_cursor();
    let mut state = SnakeState::new((10, 10).into(), shell.width(), shell.height());

    'running: while let Some(tick) = shell.tick() {
        let now = Instant::now();
        for event in tick.events() {
            match event {
                Event::Key { code, modifiers: _ } => match code {
                    KeyCode::Escape => break 'running,
                    other => state.handle_key(*other),
                },
                Event::Resized(nw, nh) => shell.viewport().resize(*nw as u32, *nh as u32),
                Event::Quit => break 'running,
            }
        }

        state.update();
        shell.start_frame().with_draw((0, 0), &state).submit();

        while now.elapsed() < Duration::from_secs_f32(1.0 / 240.0) {}
    }
}
