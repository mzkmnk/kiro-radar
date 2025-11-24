# Implementation Plan

- [x] 1. ViewMode と DetailTab 列挙型の実装

  - app.rs に ViewMode enum を追加（List と Detail バリアント）
  - app.rs に DetailTab enum を追加（Requirements、Design、Tasks バリアント）
  - Clone と PartialEq を derive
  - _Requirements: 7.1, 7.2, 7.3, 9.1_

- [x] 2. App 構造体の拡張

  - view_mode フィールドを追加（初期値: ViewMode::List）
  - detail_scroll フィールドを追加（初期値: 0）
  - active_tab フィールドを追加（初期値: DetailTab::Requirements）
  - _Requirements: 7.1, 1.3, 2.1_

- [x] 3. ビューモード遷移メソッドの実装

  - enter_detail_view() メソッドを実装（active_tab を Requirements に初期化）
  - exit_detail_view() メソッドを実装
  - スクロール位置の初期化とクリアを含む
  - _Requirements: 1.1, 5.1, 1.3, 5.3, 2.1_

- [x] 3.1 タブ切り替えメソッドの実装

  - next_tab() メソッドを実装（Requirements → Design → Tasks → Requirements）
  - タブ切り替え時にスクロール位置を 0 にリセット
  - _Requirements: 9.1, 9.2, 9.3_

- [ ]\* 3.2 ビューモード遷移のプロパティテストを実装

  - **Property 1: ビューモード遷移の正確性**
  - **Property 2: ビューモード復帰の正確性**
  - **Property 3: 選択状態の保持（Round-trip）**
  - **Property 4: スクロール位置の初期化**
  - **Property 5: スクロール位置のクリア**
  - **Validates: Requirements 1.1, 5.1, 5.2, 1.3, 5.3**

- [ ]\* 3.3 タブ切り替えのプロパティテストを実装

  - **Property 13: タブ切り替えの正確性**
  - **Property 14: タブ切り替え時のスクロールリセット**
  - **Validates: Requirements 9.1, 9.2, 9.3**

- [x] 4. スクロール操作メソッドの実装

  - scroll_down() メソッドを実装（境界チェック付き）
  - scroll_up() メソッドを実装（境界チェック付き）
  - _Requirements: 4.1, 4.2, 4.3, 4.4_

- [ ]\* 4.1 スクロール操作のプロパティテストを実装

  - **Property 6: スクロール操作の動作**
  - **Validates: Requirements 4.1, 4.2**

- [ ]\* 4.2 スクロール境界条件のユニットテストを実装

  - 最上部での上スクロールテスト
  - 最下部での下スクロールテスト
  - _Requirements: 4.3, 4.4_

- [x] 5. spec/reader.rs モジュールの作成

  - SpecContent 構造体を定義
  - read_spec_content() 関数を実装
  - read_spec_file() ヘルパー関数を実装
  - エラーハンドリング（ファイル不在、IO エラー）
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 3.1, 3.2, 8.1, 8.2, 8.3_

- [ ]\* 5.1 ファイル読み込みのプロパティテストを実装

  - **Property 7: ファイル内容の表示**
  - **Property 8: エラーメッセージの表示**
  - **Validates: Requirements 2.1, 2.3, 3.1, 8.1, 8.3**

- [ ]\* 5.2 エラーケースのユニットテストを実装

  - ファイル不在時のエラーメッセージテスト
  - 特定のエラーメッセージ文字列の検証
  - _Requirements: 2.2, 2.4, 3.2, 8.2_

- [x] 6. イベント処理の拡張（events.rs）

  - on_key_event() を ViewMode に応じて分岐
  - リストビューでの Enter キー処理を追加
  - 詳細ビューでの Esc キー処理を追加
  - 詳細ビューでの j/k、↑↓ キー処理を追加
  - 詳細ビューでの Tab キー処理を追加（next_tab() 呼び出し）
  - _Requirements: 1.1, 4.1, 4.2, 5.1, 6.3, 9.1_

- [ ]\* 6.1 イベント処理のプロパティテストを実装

  - **Property 11: 終了操作の動作**
  - **Validates: Requirements 6.3**

- [x] 7. タブ UI の実装（ui.rs）

  - render_tabs() ヘルパー関数を作成
  - 右上にタブ一覧を表示（Requirements、Design、Tasks）
  - アクティブタブを視覚的に強調表示（反転表示または色付き）
  - _Requirements: 9.5, 9.6_

- [ ]\* 7.1 タブ UI のプロパティテストを実装

  - **Property 15: タブ一覧の表示**
  - **Property 16: アクティブタブの強調表示**
  - **Validates: Requirements 9.5, 9.6**

- [x] 7.2 詳細ビュー UI の実装（ui.rs）

  - render_detail_view() 関数を作成
  - Spec 名のヘッダー表示
  - タブ UI の統合（render_tabs() 呼び出し）
  - アクティブタブに応じたファイル内容の表示
  - スクロール処理の実装
  - _Requirements: 1.2, 2.1, 2.3, 9.4_

- [ ]\* 7.3 詳細ビュー UI のプロパティテストを実装

  - **Property 9: Spec 名の表示**
  - **Property 12: ViewMode に応じたレンダリング分岐**
  - **Validates: Requirements 1.2, 7.2**

- [ ] 8. 詳細ビューのフッター実装

  - キーバインド情報の表示（Tab キーを含む）
  - _Requirements: 6.1, 6.2_

- [ ]\* 8.1 フッター表示のプロパティテストとユニットテストを実装

  - **Property 10: キーバインド情報の表示**
  - 特定のキーバインド文字列の検証（Tab キーを含む）
  - **Validates: Requirements 6.1, 6.2**

- [ ] 9. render() 関数の分岐実装

  - ViewMode に応じて render_list_view() または render_detail_view() を呼び出し
  - 既存のリストビューロジックを render_list_view() に抽出
  - _Requirements: 7.2_

- [ ] 10. 依存関係の追加

  - Cargo.toml に quickcheck クレート（v1.0）を追加
  - _Requirements: Testing Strategy_

- [ ] 11. Checkpoint - すべてのテストが通ることを確認
  - すべてのテストを実行し、合格を確認
  - 質問があればユーザーに確認
