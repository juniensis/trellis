use std::time::Instant;

use trellis_core::collections::Queue;
use trellis_terminal::{Terminal, backend::Backend, event::Event};

use crate::{frame::Frame, primitives::RenderCtx, viewport::Viewport};

pub struct Shell {
    viewport: Viewport,
    frame_idx: u32,
    init: Instant,
    last: Instant,
    ctx: RenderCtx,
    events: Queue<Event>,
}

#[derive(Debug)]
pub struct Tick {
    frame: u32,
    frame_time: f32,
    frame_delta: f32,
    events: Vec<Event>,
}

impl Tick {
    pub fn events(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }
}

impl Shell {
    pub fn new<B: Backend + 'static>(backend: B) -> Self {
        let mut backend = Box::new(backend);
        let mut events = Queue::new();
        backend.link(events.clone(), 1000);
        backend.enter_alternate_screen();
        backend.enable_raw_mode();

        let viewport = Viewport::new(backend).expect("Failed to build viewport.");

        Self {
            viewport,
            frame_idx: 0,
            init: Instant::now(),
            last: Instant::now(),
            ctx: RenderCtx::default(),
            events,
        }
    }
    pub fn new_crossterm() -> Self {
        let backend = Terminal::new();
        Self::new(backend)
    }
    pub fn tick(&mut self) -> Option<Tick> {
        let mut events = Vec::new();

        while let Some(event) = self.events.pop() {
            events.push(event);
        }
        let frame = self.frame_idx;
        let frame_time = self.init.elapsed().as_secs_f32();
        let frame_delta = self.last.elapsed().as_secs_f32();

        self.ctx = RenderCtx {
            viewport_width: self.viewport.width(),
            viewport_height: self.viewport.height(),
            time: frame_time,
            delta: frame_delta,
        };

        self.last = Instant::now();
        self.frame_idx += 1;

        Some(Tick {
            frame,
            frame_time,
            frame_delta,
            events,
        })
    }
    pub fn backend(&mut self) -> &mut Box<dyn Backend> {
        &mut self.viewport.backend
    }
    pub fn viewport(&mut self) -> &mut Viewport {
        &mut self.viewport
    }
    pub fn width(&self) -> u32 {
        self.viewport.width()
    }
    pub fn height(&self) -> u32 {
        self.viewport.height()
    }
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
    pub fn start_frame<'a>(&'a mut self) -> Frame<'a> {
        Frame::new(&mut self.viewport, self.ctx)
    }
    pub fn end_frame(&mut self) {
        self.viewport().flush();
    }
    pub fn hide_cursor(&mut self) {
        let _ = self.backend().hide_cursor();
    }
    pub fn show_cursor(&mut self) {
        let _ = self.backend().show_cursor();
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        self.backend().disable_raw_mode();
        self.backend().leave_alternate_screen();
        self.backend().show_cursor();
    }
}

#[cfg(test)]
mod tests {
    use trellis_core::terminal::Cell;
    use trellis_terminal::event::KeyCode;

    use crate::primitives::{Primitive, Region};

    use super::*;

    #[test]
    fn draw_rectangle() {
        let mut shell = Shell::new_crossterm();
        let mut x = 0u32;
        let mut y = 0u32;

        'running: while let Some(tick) = shell.tick() {
            for event in tick.events() {
                if let Event::Key { code, modifiers } = event {
                    match code {
                        KeyCode::Escape => break 'running,
                        KeyCode::Up => y = y.saturating_sub(1),
                        KeyCode::Down => y += 1,
                        KeyCode::Left => x = x.saturating_sub(1),
                        KeyCode::Right => x += 1,
                        _ => {}
                    }
                }
            }

            let rect = Region::build(
                vec![
                    &[Cell::new('x'); 3],
                    &[Cell::new('x'), Cell::new(' '), Cell::new('x')],
                    &[Cell::new('x'); 3],
                ],
                0,
            );

            let fps = 1.0 / tick.frame_delta;
            print!("{fps}");

            shell.viewport().composite((x, y).into(), rect);
            shell.viewport().flush();
        }
    }
}
