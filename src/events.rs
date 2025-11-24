use crate::app::{App, ViewMode};
use crate::ui::calculate_max_scroll;
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
    // コンテンツ領域の高さを推定（ターミナルサイズに依存するが、一般的な値を使用）
    // 実際のレンダリング時にはより正確な値が使用される
    const ESTIMATED_CONTENT_HEIGHT: usize = 20;

    match (key.modifiers, key.code) {
        // 終了操作
        (_, KeyCode::Char('q'))
        | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => quit(app),
        // リストビューへの復帰
        (_, KeyCode::Esc) => app.exit_detail_view(),
        // タブ切り替え
        (_, KeyCode::Tab) => app.next_tab(),
        // スクロール操作
        (_, KeyCode::Down | KeyCode::Char('j')) => {
            let max_scroll = calculate_max_scroll(app, ESTIMATED_CONTENT_HEIGHT);
            app.scroll_down(max_scroll);
        }
        (_, KeyCode::Up | KeyCode::Char('k')) => app.scroll_up(),
        _ => {}
    }
}

/// Set running to false to quit the application.
fn quit(app: &mut App) {
    app.running = false;
}
