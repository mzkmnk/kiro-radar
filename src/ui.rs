use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Gauge, List, ListItem, Paragraph},
};

pub fn render(frame: &mut Frame) {
    let area = frame.area();

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(5),
        ])
        .split(area);

    let header =
        Paragraph::new("kiro-radar Dashboard").style(Style::default().fg(Color::Cyan).bold());
    frame.render_widget(header, main_chunks[0]);

    let gauge = Gauge::default()
        .block(
            Block::bordered()
                .title("Progress")
                .border_type(BorderType::Rounded),
        )
        .gauge_style(Style::default().fg(Color::Green))
        .percent(60)
        .label("60%");

    frame.render_widget(gauge, main_chunks[1]);

    let items = vec![
        ListItem::new("requirements.md"),
        ListItem::new("design.md"),
        ListItem::new("tasks.md"),
    ];

    let list = List::new(items)
        .block(
            Block::bordered()
                .title("Spec Files")
                .border_type(BorderType::Rounded),
        )
        .highlight_style(Style::default().bg(Color::Blue));
    frame.render_widget(list, main_chunks[2]);
}
