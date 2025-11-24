use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Padding, Paragraph},
};

use crate::app::App;

// Define a palette based on Charm's aesthetics
const COLOR_PRIMARY: Color = Color::Magenta;
const COLOR_SECONDARY: Color = Color::Cyan;
const COLOR_TEXT: Color = Color::White;
const COLOR_SUBTEXT: Color = Color::DarkGray;

pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.area();

    // Add margins to create a "floating window" feel
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Top margin
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Bottom margin
        ])
        .split(area);

    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(2), // Left margin
            Constraint::Min(0),    // Content
            Constraint::Length(2), // Right margin
        ])
        .split(outer_layout[1]);

    let content_area = horizontal_layout[1];

    // Calculate overall progress
    let total_tasks: usize = app
        .spec_sets
        .iter()
        .map(|s| s.total_tasks.unwrap_or(0))
        .sum();
    let completed_tasks: usize = app
        .spec_sets
        .iter()
        .map(|s| s.completed_tasks.unwrap_or(0))
        .sum();

    let progress_ratio = if total_tasks > 0 {
        completed_tasks as f64 / total_tasks as f64
    } else {
        0.0
    };

    // Main Layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header
            Constraint::Length(3), // Progress
            Constraint::Min(0),    // List
            Constraint::Length(1), // Footer
        ])
        .split(content_area);

    let header = Paragraph::new("[ KIRO RADAR - 0.1.0 ]")
        .alignment(Alignment::Right)
        .style(
            Style::default()
                .fg(COLOR_SECONDARY)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(header, chunks[0]);

    // Progress
    let label = Span::styled(
        format!(
            "{:.0}% ({}/{})",
            progress_ratio * 100.0,
            completed_tasks,
            total_tasks
        ),
        Style::default().fg(COLOR_TEXT).add_modifier(Modifier::BOLD),
    );

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title("Overall Progress")
                .title_style(Style::default().fg(COLOR_SECONDARY))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(COLOR_SUBTEXT))
                .padding(Padding::horizontal(1)),
        )
        .gauge_style(
            Style::default()
                .fg(COLOR_SECONDARY)
                .bg(Color::Rgb(60, 60, 60)),
        )
        .ratio(progress_ratio)
        .label(label)
        .use_unicode(true);

    frame.render_widget(gauge, chunks[1]);

    // Spec List
    let items: Vec<ListItem> = app
        .spec_sets
        .iter()
        .flat_map(|spec| {
            let t = spec.total_tasks.unwrap_or(0);
            let c = spec.completed_tasks.unwrap_or(0);
            let p = if t > 0 { c as f64 / t as f64 } else { 0.0 };
            let percent = (p * 100.0) as u16;

            let name_style = Style::default().fg(COLOR_TEXT).add_modifier(Modifier::BOLD);
            let info_style = Style::default().fg(COLOR_SUBTEXT);

            let line = Line::from(vec![
                Span::styled(format!("{:<20}", spec.name), name_style),
                Span::styled(format!("  {:>3}% ({}/{})", percent, c, t), info_style),
            ]);

            // メインアイテムと空行を返す
            vec![ListItem::new(line)]
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Specs")
                .title_style(Style::default().fg(COLOR_PRIMARY))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(COLOR_SUBTEXT))
                .padding(Padding::new(1, 1, 1, 1)),
        )
        .highlight_style(
            Style::default()
                .bg(COLOR_PRIMARY)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_stateful_widget(list, chunks[2], &mut app.list_state);

    let footer = Paragraph::new("[ ↑↓/jk -> navigate ] [ q  -> quit ]")
        .alignment(Alignment::Right)
        .style(
            Style::default()
                .fg(COLOR_SUBTEXT)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(footer, chunks[3]);
}
