use std::time::Instant;

use trellis_core::{
    terminal::{Cell, style::Color},
    utils::rand::{psuedo_random_u32, psuedo_random_u64},
};
use trellis_graphics::{
    primitives::{Region, Scatter},
    shell::Shell,
};
use trellis_terminal::event::{Event, KeyCode};

struct State {
    width: u32,
    height: u32,
}

const CHARS: &[u8] = b"!@#$%^&*()_+ ,.<>?:";

impl State {
    fn new(w: u32, h: u32) -> Self {
        Self {
            width: w,
            height: h,
        }
    }
    fn random_region(&self) -> (u32, u32, Region) {
        let mut ret = Region::new(psuedo_random_u64() as u32 % 128);
        let w = psuedo_random_u32() % self.width / 2;
        let h = psuedo_random_u32() % self.height / 2;
        let xc = psuedo_random_u32() % (self.width - w);
        let yc = psuedo_random_u32() % (self.height - h);
        let r = psuedo_random_u32() % 256;
        let g = psuedo_random_u32() % 256;
        let b = psuedo_random_u32() % 256;
        let ch = psuedo_random_u32() as usize % CHARS.len();
        for y in 0..h {
            for x in 0..w {
                ret.set_cell(
                    x,
                    y,
                    Cell::new(CHARS[ch] as char).with_fg(Color::RGB(r as u8, g as u8, b as u8)),
                );
            }
        }
        (xc, yc, ret)
    }
    fn random_scatter(&self) -> Scatter {
        let mut ret = Scatter::new(psuedo_random_u32() % 128);
        let points = psuedo_random_u32() % 64;

        for _ in 0..points {
            let x = psuedo_random_u32() % self.width;
            let y = psuedo_random_u32() % self.height;
            let r = psuedo_random_u32() % 256;
            let g = psuedo_random_u32() % 256;
            let b = psuedo_random_u32() % 256;
            let ch = psuedo_random_u32() as usize % CHARS.len();
            ret.insert(
                (x, y),
                Cell::new(CHARS[ch] as char).with_fg(Color::RGB(r as u8, g as u8, b as u8)),
            )
        }
        ret
    }
}

fn main() {
    let mut shell = Shell::new_crossterm();
    let state = State::new(shell.width(), shell.height());
    let mut samples = Vec::with_capacity(1024);

    let mut now = Instant::now();
    'running: while let Some(tick) = shell.tick() {
        for event in tick.events() {
            if let Event::Key {
                code: KeyCode::Escape,
                modifiers: _,
            } = event
            {
                break 'running;
            }
        }

        let mut frame = shell.start_frame();
        for _ in 0..8 {
            let (x, y, r) = state.random_region();
            frame.draw((x, y), &r.into_primitive());
        }

        for _ in 0..8 {
            let s = state.random_scatter();
            frame.draw((0, 0), &s.into_primitive());
        }

        frame.submit_with_hidden_cursor();

        samples.push(now.elapsed().as_secs_f64());
        if samples.len() == 1024 {
            break;
        }
        now = Instant::now();
    }

    drop(shell);

    samples.sort_by(|a, b| a.total_cmp(b));

    let max = *samples.last().unwrap();
    let min = *samples.first().unwrap();
    let sum = samples.iter().sum::<f64>();
    let avg = sum / 1024.0;

    println!(
        "frametimes:\n  max: {}\n  min: {}\n  avg: {}",
        max, min, avg
    );
    println!(
        "fps:\n  max: {}\n  min: {}\n  avg: {}",
        1.0 / max,
        1.0 / min,
        1.0 / avg
    );
}
