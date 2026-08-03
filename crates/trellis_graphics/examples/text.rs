use std::time::Instant;

use trellis_graphics::{shell::Shell, text::Text};
use trellis_terminal::event::{Event, KeyCode};

fn main() {
    let mut shell = Shell::new_crossterm();

    let mut text = Text::new("", 0);

    let mut start = Instant::now();
    'running: while let Some(tick) = shell.tick() {
        for event in tick.events() {
            if let &Event::Key {
                code: KeyCode::Escape,
                modifiers: _,
            } = event
            {
                break 'running;
            }
        }

        let frametime = start.elapsed().as_secs_f32();

        text.set_text(format!("{}fps", 1.0 / frametime));
        shell.start_frame().draw((0, 0), &text).submit();

        start = Instant::now();
    }
}
