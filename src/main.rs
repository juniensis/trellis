#![allow(dead_code, unused)]

use std::time::{Duration, Instant};

use trellis::Trellis;
use trellis_core::terminal::Cell;
use trellis_graphics::{
    primitives::{Primitive, Region, Scatter},
    shell::Shell,
};
use trellis_terminal::event::{Event, KeyCode};

fn main() {
    let mut shell = Shell::new_crossterm();
    let mut trellis = Trellis::new(shell.width(), shell.height());
    shell.show_cursor();

    'running: while let Some(tick) = shell.tick() {
        let mut now = Instant::now();
        let mut commands = Vec::new();
        for event in tick.events() {
            match event {
                Event::Key {
                    code: KeyCode::Escape,
                    modifiers,
                } => {
                    break 'running;
                }
                Event::Resized(nw, nh) => {
                    shell.viewport().resize(*nw as u32, *nh as u32);
                    trellis.resize(*nw as u32, *nh as u32);
                }
                Event::Quit => break 'running,
                other => {
                    if let Some(cmd) = trellis.handle_event(other) {
                        commands.push(cmd);
                    }
                }
            }
        }

        for cmd in commands {
            trellis.handle_command(cmd);
        }

        trellis.update_status();

        let (x, y) = trellis.transformed_cursor_position();
        shell.viewport().move_cursor_to((x, y));

        let mut frame = shell.start_frame();
        trellis.draw(frame).submit();

        while now.elapsed() < Duration::from_secs_f32(1.0 / 240.0) {}
    }
}
