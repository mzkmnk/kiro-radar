use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Padding, Paragraph, Tabs},
};

use crate::app::{App, DetailTab, ViewMode};
use crate::spec::reader::read_spec_content;

// Define a palette based on Charm's aesthetics
const COLOR_PRIMARY: Color = Color::Magenta;
const COLOR_SECONDARY: Color = Color::Cyan;
const COLOR_TEXT: Color = Color::White;
const COLOR_SUBTEXT: Color = Color::DarkGray;

pub fn render(app: &mut App, frame: &mut Frame) {
    match &app.view_mode {
        ViewMode::List => render_list_view(app, frame),
        ViewMode::Detail { spec_index } => render_detail_view(app, frame, *spec_index),
    }
}

/// タブ UI をレンダリングする
///
/// # 引数
/// * `active_tab` - 現在アクティブなタブ
/// * `area` - レンダリング領域
/// * `frame` - フレーム
fn render_tabs(active_tab: &DetailTab, area: Rect, frame: &mut Frame) {
    let tab_titles = vec!["Requirements", "Design", "Tasks"];
    let selected_index = match active_tab {
        DetailTab::Requirements => 0,
        DetailTab::Design => 1,
        DetailTab::Tasks => 2,
    };

    let tabs = Tabs::new(tab_titles)
        .select(selected_index)
        .style(Style::default().fg(COLOR_SUBTEXT))
        .highlight_style(
            Style::default()
                .fg(COLOR_SECONDARY)
                .add_modifier(Modifier::BOLD | Modifier::REVERSED),
        )
        .divider(" | ");

    frame.render_widget(tabs, area);
}

/// 詳細ビューをレンダリングする
///
/// # 引数
/// * `app` - アプリケーション状態
/// * `frame` - フレーム
/// * `spec_index` - 表示する Spec のインデックス
fn render_detail_view(app: &mut App, frame: &mut Frame, spec_index: usize) {
    let area = frame.area();

    // マージンを追加
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

    // メインレイアウト
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header
            Constraint::Length(1), // Spec name + Tabs
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Footer
        ])
        .split(content_area);

    // ヘッダー
    let version = env!("CARGO_PKG_VERSION");
    let header = Paragraph::new(format!("[ KIRO RADAR - {} ]", version))
        .alignment(Alignment::Right)
        .style(
            Style::default()
                .fg(COLOR_SECONDARY)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_widget(header, chunks[0]);

    // Spec 名とタブ
    let spec_name = app
        .spec_sets
        .get(spec_index)
        .map(|s| s.name.as_str())
        .unwrap_or("Unknown");

    // Spec 名とタブを横に並べる
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),     // Spec name
            Constraint::Length(40), // Tabs
        ])
        .split(chunks[1]);

    let spec_header = Paragraph::new(format!("Spec: {}", spec_name)).style(
        Style::default()
            .fg(COLOR_PRIMARY)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(spec_header, header_chunks[0]);

    // タブ UI
    render_tabs(&app.active_tab, header_chunks[1], frame);

    // コンテンツ領域
    let spec_set = app.spec_sets.get(spec_index);
    let content_text = if let Some(spec) = spec_set {
        let spec_content = read_spec_content(spec);
        match app.active_tab {
            DetailTab::Requirements => spec_content.requirements,
            DetailTab::Design => spec_content.design,
            DetailTab::Tasks => spec_content.tasks,
        }
    } else {
        None
    };

    let display_text = content_text.unwrap_or_else(|| "File not found".to_string());
    let lines: Vec<&str> = display_text.lines().collect();
    let total_lines = lines.len();

    // 表示可能な行数を計算
    let content_height = chunks[2].height.saturating_sub(2) as usize; // ボーダー分を引く
    let max_scroll = total_lines.saturating_sub(content_height);

    // スクロール位置を調整
    let scroll_pos = app.detail_scroll.min(max_scroll);

    // 表示する行を取得
    let visible_lines: Vec<Line> = lines
        .iter()
        .skip(scroll_pos)
        .take(content_height)
        .map(|&line| Line::from(line))
        .collect();

    let tab_name = match app.active_tab {
        DetailTab::Requirements => "requirements.md",
        DetailTab::Design => "design.md",
        DetailTab::Tasks => "tasks.md",
    };

    let content_block = Block::default()
        .title(tab_name)
        .title_style(Style::default().fg(COLOR_SECONDARY))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_SUBTEXT))
        .padding(Padding::horizontal(1));

    let content_paragraph = Paragraph::new(visible_lines).block(content_block);
    frame.render_widget(content_paragraph, chunks[2]);

    // フッター
    let footer = Paragraph::new("[ Tab: Switch, ↑/k: Up, ↓/j: Down, Esc: Back, q: Quit ]")
        .alignment(Alignment::Right)
        .style(
            Style::default()
                .fg(COLOR_SUBTEXT)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_widget(footer, chunks[3]);
}

/// リストビューをレンダリングする
fn render_list_view(app: &mut App, frame: &mut Frame) {
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

    let version = env!("CARGO_PKG_VERSION");
    let header = Paragraph::new(format!("[ KIRO RADAR - {} ]", version))
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

/// 詳細ビューの最大スクロール位置を計算する
pub fn calculate_max_scroll(app: &App, content_height: usize) -> usize {
    if let ViewMode::Detail { spec_index } = &app.view_mode {
        if let Some(spec) = app.spec_sets.get(*spec_index) {
            let spec_content = read_spec_content(spec);
            let content_text = match app.active_tab {
                DetailTab::Requirements => spec_content.requirements,
                DetailTab::Design => spec_content.design,
                DetailTab::Tasks => spec_content.tasks,
            };

            if let Some(text) = content_text {
                let total_lines = text.lines().count();
                return total_lines.saturating_sub(content_height);
            }
        }
    }
    0
}
