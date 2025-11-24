use crate::app::App;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub fn handle_crossterm_events(app: &mut App) -> Result<()> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => on_key_event(app, key),
        Event::Mouse(_) => {}
        Event::Resize(_, _) => {}
        _ => {}
    }
    Ok(())
}

fn on_key_event(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
        (_, KeyCode::Esc | KeyCode::Char('q'))
        | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => quit(app),
        (_, KeyCode::Down | KeyCode::Char('j')) => app.next_item(),
        (_, KeyCode::Up | KeyCode::Char('k')) => app.previous_item(),
        _ => {}
    }
}

/// Set running to false to quit the application.
fn quit(app: &mut App) {
    app.running = false;
}
