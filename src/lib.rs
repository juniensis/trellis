#![allow(dead_code, unused_imports, unused)]
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use std::{collections::HashMap, thread::sleep, time::Duration};

use crate::{components::Component, viewport::Viewport};

pub mod cell;
pub mod components;
pub mod viewport;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    KeyPressed {
        code: KeyCode,
        modifiers: KeyModifiers,
    },
    KeyReleased {
        code: KeyCode,
        modifiers: KeyModifiers,
    },
    KeyRepeat {
        code: KeyCode,
        modifiers: KeyModifiers,
    },
    MoveCursor(u16, u16),
    DisplaceCursor(i16, i16),
    Leave,
    MoveTo(u16, u16),
    MoveBy(i16, i16),
}

impl Event {
    pub fn try_from_crossterm_event(e: crossterm::event::Event) -> Option<Self> {
        match e {
            crossterm::event::Event::Key(KeyEvent {
                code,
                modifiers,
                kind,
                state: _,
            }) => match kind {
                KeyEventKind::Press => Some(Event::KeyPressed { code, modifiers }),
                KeyEventKind::Release => Some(Event::KeyReleased { code, modifiers }),
                KeyEventKind::Repeat => Some(Event::KeyRepeat { code, modifiers }),
            },
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct Trellis {
    viewport: Viewport,
    components: Vec<Box<dyn Component>>,
    event_queue: Vec<Event>,
    current_component: Option<usize>,
}

impl Trellis {
    pub fn new() -> Self {
        Self {
            viewport: Viewport::new(),
            components: Vec::new(),
            event_queue: Vec::new(),
            current_component: None,
        }
    }
    pub fn with_component<C: Component + 'static>(mut self, component: C) -> Self {
        self.components.push(Box::new(component));
        self
    }
    pub fn push_component<C: Component + 'static>(&mut self, component: C) {
        self.components.push(Box::new(component));
    }
    pub fn run(mut self) {
        'outer: loop {
            if let Ok(true) = crossterm::event::poll(Duration::from_secs(0))
                && let Some(e) = crossterm::event::read()
                    .ok()
                    .and_then(Event::try_from_crossterm_event)
            {
                self.event_queue.push(e)
            }

            while let Some(event) = self.event_queue.pop() {
                if let Some(component) = self.current_component {
                    if let Some(tail) = self.components[component].update(event) {
                        if let Event::Leave = tail {
                            self.current_component = None;
                        } else if let Event::DisplaceCursor(dx, dy) = tail {
                            self.viewport.displace_cursor(dx, dy);
                        } else {
                            self.event_queue.push(tail);
                        }
                    }
                    self.components[component].draw(&mut self.viewport);
                } else {
                    match event {
                        Event::KeyPressed {
                            code: KeyCode::Esc,
                            modifiers: _,
                        } => break 'outer,
                        Event::KeyPressed {
                            code: KeyCode::Enter,
                            modifiers: _,
                        } => {
                            for (i, c) in self.components.iter().enumerate() {
                                let (x, y) = self.viewport.pos();
                                if c.is_inside(x, y) {
                                    self.current_component = Some(i);
                                    break;
                                }
                            }
                        }
                        Event::KeyPressed {
                            code: KeyCode::Char(ch),
                            modifiers: _,
                        } => match ch {
                            'h' => self.viewport.displace_cursor(-1, 0),
                            'j' => self.viewport.displace_cursor(0, 1),
                            'k' => self.viewport.displace_cursor(0, -1),
                            'l' => self.viewport.displace_cursor(1, 0),
                            'H' => self.viewport.displace_cursor(-5, 0),
                            'J' => self.viewport.displace_cursor(0, 5),
                            'K' => self.viewport.displace_cursor(0, -5),
                            'L' => self.viewport.displace_cursor(5, 0),
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }

            for component in &self.components {
                component.draw(&mut self.viewport);
            }
            sleep(Duration::from_secs_f64(1.0 / 240.0));
        }
    }
}
