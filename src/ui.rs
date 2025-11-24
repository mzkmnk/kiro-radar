use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.area();

    let header =
        Paragraph::new("kiro-radar Dashboard").style(Style::default().fg(Color::Cyan).bold());

    let items: Vec<ListItem> = app
        .spec_sets
        .iter()
        .map(|spec| {
            let text = format!("{}", spec.name);
            ListItem::new(text)
        })
        .collect();

    let items_count = items.len() as u16;

    let list = List::new(items).block(
        Block::bordered()
            .title("Specs")
            .border_type(BorderType::Rounded),
    );

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(items_count + 2)])
        .split(area);

    frame.render_widget(header, main_chunks[0]);
    frame.render_widget(list, main_chunks[1]);
}
