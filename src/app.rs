use std::path::Path;

use crate::events::handle_crossterm_events;
use crate::spec::finder::{SpecSet, find_all_specs};
use crate::ui::render;
use color_eyre::Result;
use ratatui::DefaultTerminal;

#[derive(Debug, Default)]
pub struct App {
    pub running: bool,
    pub spec_sets: Vec<SpecSet>,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self::default();

        if let Ok(specs) = find_all_specs(Path::new(".")) {
            app.spec_sets = specs;
        }

        app
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
