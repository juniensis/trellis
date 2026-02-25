use std::{collections::HashMap, rc::Rc};

use trellis_terminal::{Terminal, error::TerminalResult, event::EventQueue};
use trellis_ui::{
    Buffer, Style,
    event::UiEvent,
    widgets::{Widget, WidgetId},
};

pub struct Viewport {
    front: Buffer,
    pub back: Buffer,
    terminal: Terminal,
    ui_widgets: HashMap<WidgetId, Box<dyn Widget<UiEvent>>>,
    ui_events: EventQueue<UiEvent>,
}

impl Viewport {
    pub fn new() -> TerminalResult<Self> {
        let terminal = Terminal::new()?;
        let (width, height) = terminal.size()?;
        Ok(Self {
            front: Buffer::new(width, height),
            back: Buffer::new(width, height),
            terminal,
            ui_widgets: HashMap::new(),
            ui_events: EventQueue::new(),
        })
    }
    pub fn flush(&mut self) -> TerminalResult<()> {
        self.terminal.save_cursor()?;
        self.terminal.hide_cursor()?;

        let (mut last_x, mut last_y) = (0, 0);
        let mut last_style = Style::default();

        for (((x, y), next), current) in self
            .back
            .positioned_iter()
            .zip(self.front.iter_mut())
            .filter(|((_, lhs), rhs)| lhs != rhs)
        {
            if x != last_x + 1 || y != last_y {
                self.terminal.move_cursor(x, y)?;
            }

            if next.style() != last_style {
                self.terminal.write_str(next.to_string())?;
                last_style = next.style();
            } else {
                self.terminal.write_char(next.char())?;
            }
            (last_x, last_y) = (x, y);
            *current = *next;
        }

        self.terminal.reset_sgr()?;
        self.terminal.restore_cursor()?;
        self.terminal.show_cursor()?;
        self.terminal.flush()
    }
    pub fn render(&mut self) {
        for widget in self.ui_widgets.values() {
            widget.draw(&mut self.back);
        }
    }
    pub fn update(&mut self, targets: &[WidgetId]) {
        if let Some(e) = self.ui_events.try_recv() {
            for &idx in targets {
                let mut tail = self
                    .ui_widgets
                    .get_mut(&idx)
                    .and_then(|x| x.update(e.clone()));
                while let Some(t) = tail {
                    tail = self.ui_widgets.get_mut(&idx).and_then(|x| x.update(t));
                }
            }
        }
    }
    pub fn add_widget(&mut self, widget: Box<dyn Widget<UiEvent>>) -> WidgetId {
        let ret = widget.identify();
        self.ui_widgets.insert(ret, widget);
        ret
    }
    pub fn get_widget(&self, id: WidgetId) -> Option<&dyn Widget<UiEvent>> {
        self.ui_widgets.get(&id).map(|x| x.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;
    #[test]
    fn labels() {
        let mut ui = Viewport::new().unwrap();
        ui.flush().unwrap();
        sleep(Duration::from_secs(5));
    }
}
