use crate::events::handle_crossterm_events;
use crate::ui::render;
use color_eyre::Result;
use ratatui::DefaultTerminal;

#[derive(Debug, Default)]
pub struct App {
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(render)?;
            handle_crossterm_events(&mut self)?;
        }
        Ok(())
    }
}
