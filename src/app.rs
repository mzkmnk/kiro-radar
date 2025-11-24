use std::path::Path;

use crate::events::handle_crossterm_events;
use crate::spec::finder::{SpecSet, find_all_specs};
use crate::ui::render;
use color_eyre::Result;
use ratatui::DefaultTerminal;
use ratatui::widgets::ListState;

#[derive(Debug, Clone, PartialEq)]
pub enum ViewMode {
    List,
    Detail { spec_index: usize },
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub spec_sets: Vec<SpecSet>,
    pub list_state: ListState,
    pub view_mode: ViewMode,
    pub detail_scroll: usize,
}

impl App {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let mut app = Self {
            running: false,
            spec_sets: Vec::new(),
            list_state: ListState::default(),
            view_mode: ViewMode::List,
            detail_scroll: 0,
        };

        if let Ok(specs) = find_all_specs(path.as_ref()) {
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

    pub fn enter_detail_view(&mut self) {
        if let Some(selected_index) = self.list_state.selected() {
            self.view_mode = ViewMode::Detail {
                spec_index: selected_index,
            };
            self.detail_scroll = 0;
        }
    }

    pub fn exit_detail_view(&mut self) {
        self.view_mode = ViewMode::List;
        self.detail_scroll = 0;
    }

    pub fn scroll_down(&mut self, max_scroll: usize) {
        if self.detail_scroll < max_scroll {
            self.detail_scroll += 1;
        }
    }

    pub fn scroll_up(&mut self) {
        if self.detail_scroll > 0 {
            self.detail_scroll = self.detail_scroll.saturating_sub(1);
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_enter_detail_view() {
        let temp_dir = TempDir::new().unwrap();
        let mut app = App::new(temp_dir.path());

        // 初期状態はリストビュー
        assert_eq!(app.view_mode, ViewMode::List);
        assert_eq!(app.detail_scroll, 0);

        // Spec がない場合は何も起こらない
        app.enter_detail_view();
        assert_eq!(app.view_mode, ViewMode::List);

        // Spec を追加して選択状態にする
        app.spec_sets.push(SpecSet {
            name: "test-spec".to_string(),
            requirements: None,
            design: None,
            tasks: None,
            total_tasks: None,
            completed_tasks: None,
        });
        app.list_state.select(Some(0));

        // 詳細ビューに遷移
        app.enter_detail_view();
        assert_eq!(app.view_mode, ViewMode::Detail { spec_index: 0 });
        assert_eq!(app.detail_scroll, 0);
    }

    #[test]
    fn test_enter_detail_view_resets_scroll() {
        let temp_dir = TempDir::new().unwrap();
        let mut app = App::new(temp_dir.path());

        // Spec を追加
        app.spec_sets.push(SpecSet {
            name: "test-spec".to_string(),
            requirements: None,
            design: None,
            tasks: None,
            total_tasks: None,
            completed_tasks: None,
        });
        app.list_state.select(Some(0));

        // スクロール位置を変更
        app.detail_scroll = 10;

        // 詳細ビューに遷移するとスクロール位置が初期化される
        app.enter_detail_view();
        assert_eq!(app.detail_scroll, 0);
    }

    #[test]
    fn test_exit_detail_view() {
        let temp_dir = TempDir::new().unwrap();
        let mut app = App::new(temp_dir.path());

        // Spec を追加して詳細ビューに遷移
        app.spec_sets.push(SpecSet {
            name: "test-spec".to_string(),
            requirements: None,
            design: None,
            tasks: None,
            total_tasks: None,
            completed_tasks: None,
        });
        app.list_state.select(Some(0));
        app.enter_detail_view();

        // スクロール位置を変更
        app.detail_scroll = 5;

        // リストビューに戻る
        app.exit_detail_view();
        assert_eq!(app.view_mode, ViewMode::List);
        assert_eq!(app.detail_scroll, 0);
    }

    #[test]
    fn test_exit_detail_view_clears_scroll() {
        let temp_dir = TempDir::new().unwrap();
        let mut app = App::new(temp_dir.path());

        // 詳細ビュー状態を設定
        app.view_mode = ViewMode::Detail { spec_index: 0 };
        app.detail_scroll = 20;

        // リストビューに戻るとスクロール位置がクリアされる
        app.exit_detail_view();
        assert_eq!(app.view_mode, ViewMode::List);
        assert_eq!(app.detail_scroll, 0);
    }

    #[test]
    fn test_scroll_down() {
        let temp_dir = TempDir::new().unwrap();
        let mut app = App::new(temp_dir.path());

        // 初期状態
        assert_eq!(app.detail_scroll, 0);

        // 下にスクロール
        app.scroll_down(10);
        assert_eq!(app.detail_scroll, 1);

        app.scroll_down(10);
        assert_eq!(app.detail_scroll, 2);
    }

    #[test]
    fn test_scroll_down_at_bottom() {
        let temp_dir = TempDir::new().unwrap();
        let mut app = App::new(temp_dir.path());

        // 最下部に設定
        let max_scroll = 5;
        app.detail_scroll = max_scroll;

        // 最下部で下スクロールしても変化しない
        app.scroll_down(max_scroll);
        assert_eq!(app.detail_scroll, max_scroll);
    }

    #[test]
    fn test_scroll_up() {
        let temp_dir = TempDir::new().unwrap();
        let mut app = App::new(temp_dir.path());

        // スクロール位置を設定
        app.detail_scroll = 5;

        // 上にスクロール
        app.scroll_up();
        assert_eq!(app.detail_scroll, 4);

        app.scroll_up();
        assert_eq!(app.detail_scroll, 3);
    }

    #[test]
    fn test_scroll_up_at_top() {
        let temp_dir = TempDir::new().unwrap();
        let mut app = App::new(temp_dir.path());

        // 最上部（0）に設定
        app.detail_scroll = 0;

        // 最上部で上スクロールしても変化しない
        app.scroll_up();
        assert_eq!(app.detail_scroll, 0);
    }
}
