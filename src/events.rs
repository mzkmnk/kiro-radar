use crate::app::{App, ViewMode};
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
    // ViewMode に応じて処理を分岐
    match &app.view_mode {
        ViewMode::List => handle_list_view_keys(app, key),
        ViewMode::Detail { .. } => handle_detail_view_keys(app, key),
    }
}

/// リストビューでのキーイベント処理
fn handle_list_view_keys(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
        // 終了操作
        (_, KeyCode::Char('q'))
        | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => quit(app),
        // リストナビゲーション
        (_, KeyCode::Down | KeyCode::Char('j')) => app.next_item(),
        (_, KeyCode::Up | KeyCode::Char('k')) => app.previous_item(),
        // 詳細ビューへの遷移
        (_, KeyCode::Enter) => app.enter_detail_view(),
        _ => {}
    }
}

/// 詳細ビューでのキーイベント処理
fn handle_detail_view_keys(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
        // 終了操作
        (_, KeyCode::Char('q'))
        | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => quit(app),
        // リストビューへの復帰
        (_, KeyCode::Esc) => app.exit_detail_view(),
        // スクロール操作（max_scroll は仮に大きな値を設定、実際の UI レンダリング時に調整）
        (_, KeyCode::Down | KeyCode::Char('j')) => app.scroll_down(usize::MAX),
        (_, KeyCode::Up | KeyCode::Char('k')) => app.scroll_up(),
        _ => {}
    }
}

/// Set running to false to quit the application.
fn quit(app: &mut App) {
    app.running = false;
}
