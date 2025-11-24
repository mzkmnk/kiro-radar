use std::path::Path;

use crate::events::handle_crossterm_events;
use crate::spec::finder::{SpecSet, find_all_specs};
use crate::ui::render;
use color_eyre::Result;
use ratatui::DefaultTerminal;
use ratatui::widgets::ListState;

#[derive(Debug, Default)]
pub struct App {
    pub running: bool,
    pub spec_sets: Vec<SpecSet>,
    pub list_state: ListState,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            running: false,
            spec_sets: Vec::new(),
            list_state: ListState::default(),
        };

        if let Ok(specs) = find_all_specs(Path::new(".")) {
            app.spec_sets = specs;

            if !app.spec_sets.is_empty() {
                app.list_state.select(Some(0));
            }
        }

        app
    }

    pub fn next_item(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.spec_sets.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
    pub fn previous_item(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.spec_sets.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| render(&mut self, frame))?;
            handle_crossterm_events(&mut self)?;
        }
        Ok(())
    }
}
