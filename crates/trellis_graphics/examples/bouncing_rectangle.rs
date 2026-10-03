use std::time::{Duration, Instant};

use trellis_graphics::{shapes::Rectangle, shell::Shell};
use trellis_terminal::event::{Event, KeyCode};

fn main() {
    let mut shell = Shell::new_crossterm();
    let mut x = 0i32;
    let mut y = 0i32;
    let mut dx = 1i32;
    let mut dy = 1i32;

    let rect = Rectangle::new(10, 3, 0);

    'running: while let Some(tick) = shell.tick() {
        let start = Instant::now();
        for event in tick.events() {
            if let &Event::Key {
                code: KeyCode::Escape,
                modifiers: _,
            } = event
            {
                break 'running;
            }
        }

        shell.start_frame().with_draw((x, y), &rect).submit();

        if x + 10 >= shell.width() as i32 {
            dx = -1;
        }

        if x <= 0 {
            dx = 1;
        }

        if y + 3 >= shell.height() as i32 {
            dy = -1;
        }

        if y <= 0 {
            dy = 1;
        }

        x += dx;
        y += dy;

        while start.elapsed() < Duration::from_secs_f32(1.0 / 30.0) {}
    }
}
