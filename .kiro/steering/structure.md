# プロジェクト構造

## ディレクトリ構成

```
kiro-radar/
├── src/
│   ├── main.rs          # エントリーポイント
│   ├── app.rs           # App 構造体とメインロジック
│   ├── events.rs        # イベントハンドリング
│   ├── ui.rs            # UI レンダリング
│   └── spec/            # Spec ファイル解析
│       ├── mod.rs       # モジュール定義
│       ├── finder.rs    # Spec セット検出
│       └── parser.rs    # Markdown パーサー
├── target/              # ビルド成果物（Git 管理外）
├── .kiro/               # Kiro IDE 設定
│   ├── specs/           # Spec ファイル格納ディレクトリ
│   │   └── {spec-name}/ # 各Specセット
│   │       ├── requirements.md
│   │       ├── design.md
│   │       └── tasks.md
│   └── steering/        # AI アシスタント用ガイドライン
├── Cargo.toml           # プロジェクト設定と依存関係
├── Cargo.lock           # 依存関係のロックファイル
├── README.md            # プロジェクト概要
└── LICENSE              # MIT ライセンス
```

## モジュール構成

### src/main.rs

エントリーポイント。最小限の責任のみ：

- `color_eyre` の初期化
- ターミナルの初期化と復元
- `App::new().run()` の呼び出し

### src/app.rs

アプリケーションの状態管理とメインループ：

- **App 構造体**:
  - `running: bool` - アプリケーション実行状態
  - `spec_sets: Vec<SpecSet>` - 検出された Spec セット
  - `list_state: ListState` - リスト選択状態（ratatui）
- **メソッド**:
  - `new()` - 初期化時に `.kiro/specs` をスキャン
  - `run()` - メインループ（描画 → イベント処理）
  - `next_item()` / `previous_item()` - リストナビゲーション

### src/events.rs

イベント処理ロジック：

- `handle_crossterm_events()` - crossterm イベントの読み取りと振り分け
- `on_key_event()` - キーボード入力の処理
  - `q` / `Esc` / `Ctrl+C` - 終了
  - `j` / `↓` - 次の項目
  - `k` / `↑` - 前の項目
- `quit()` - アプリケーション終了

### src/ui.rs

UI レンダリングロジック：

- `render()` - メイン描画関数
- レイアウト構成:
  - ヘッダー（バージョン表示）
  - 全体進捗ゲージ
  - Spec リスト
  - フッター（キーバインド表示）
- カラーパレット定義（Charm スタイル）

### src/spec/

Spec ファイル解析モジュール：

#### finder.rs

- **SpecSet 構造体**:
  - `name: String` - Spec 名
  - `requirements: Option<PathBuf>` - requirements.md パス
  - `design: Option<PathBuf>` - design.md パス
  - `tasks: Option<PathBuf>` - tasks.md パス
  - `total_tasks: Option<usize>` - 総タスク数
  - `completed_tasks: Option<usize>` - 完了タスク数
- `find_all_specs()` - `.kiro/specs` 配下の全 Spec を検出

#### parser.rs

- `parse_tasks_file()` - tasks.md のチェックリストを解析
  - `- [ ]` - 未完了タスク
  - `- [x]` - 完了タスク
  - 戻り値: `(total, completed)`

## アーキテクチャパターン

### イベント駆動アーキテクチャ

1. crossterm によるイベント読み取り（`event::read()`）
2. イベントに応じた状態更新（`App` の変更）
3. 状態変更後の UI 再描画（`terminal.draw()`）

### 単一責任の原則

- **app.rs**: 状態管理とライフサイクル
- **events.rs**: イベント処理のみ
- **ui.rs**: レンダリングロジックのみ
- **spec/**: Spec ファイル解析のみ

### モジュール間の依存関係

```
main.rs
  └─> app.rs
       ├─> events.rs (イベント処理)
       ├─> ui.rs (レンダリング)
       └─> spec/
            ├─> finder.rs (Spec 検出)
            └─> parser.rs (タスク解析)
```

## テスト構成

各モジュールに `#[cfg(test)]` でユニットテストを配置：

- **spec/finder.rs**: Spec 検出ロジックのテスト
  - `test_find_all_specs_empty()` - 空ディレクトリ
  - `test_find_all_specs_multiple()` - 複数 Spec
- **spec/parser.rs**: タスク解析ロジックのテスト
  - `test_parse_tasks_file_not_exist()` - ファイル不在
  - `test_parse_tasks_file_empty()` - 空ファイル
  - `test_parse_tasks_file()` - 実際のチェックリスト

テストには `tempfile` クレートを使用して一時ディレクトリを作成。

## 拡張時の指針

新機能追加時は以下の原則に従う：

1. **モジュール分離**: 新機能は新しいモジュールとして追加
2. **単一責任**: 各モジュールは一つの責任のみ
3. **テストファースト**: 実装前にテストを書く
4. **エラーハンドリング**: `Result` 型を使用し、パニックを避ける
