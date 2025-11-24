# 技術スタック

## 言語・エディション

- **Rust**: Edition 2024
- **パッケージ名**: kiro-radar
- **バージョン**: 0.1.0
- **ライセンス**: MIT

## 主要ライブラリ

### UI フレームワーク

- **ratatui** (v0.29.0): TUI（Terminal User Interface）フレームワーク
  - ターミナルベースの UI コンポーネント（Block、List、Gauge、Paragraph など）
  - レイアウトシステム（Constraint、Direction）
  - スタイリング機能（Color、Modifier、Style）
  - ドキュメント: <https://docs.rs/ratatui/latest/ratatui/>

### ターミナル制御

- **crossterm** (v0.28.1): クロスプラットフォームなターミナル操作ライブラリ
  - キーボード・マウスイベントの処理（Event、KeyCode、KeyModifiers）
  - ターミナルの初期化と復元
  - イベント駆動アーキテクチャのベース

### エラーハンドリング

- **color-eyre** (v0.6.3): 人間が読みやすいエラーレポート
  - スタックトレースの可視化
  - エラーコンテキストの提供
  - Result 型との統合

### 開発依存関係

- **tempfile** (v3.23.0): テスト用の一時ファイル・ディレクトリ作成

## ビルドシステム

### 基本コマンド

```bash
# ビルド
cargo build

# リリースビルド（最適化あり）
cargo build --release

# 実行
cargo run

# テスト実行（全テスト）
cargo test

# 特定のテストを実行
cargo test test_name

# テスト出力を表示
cargo test -- --nocapture

# コードフォーマット
cargo fmt

# フォーマットチェック（CI用）
cargo fmt -- --check

# リント
cargo clippy

# より厳格なリント
cargo clippy -- -D warnings

# 依存関係の更新確認
cargo outdated

# ドキュメント生成
cargo doc --open
```

## コーディング規約

### Rust スタイル

- Rust 標準のフォーマット（`cargo fmt`）に従う
- `cargo clippy` の警告をゼロに保つ
- 未使用の警告（dead_code）は将来の実装予定でも一時的に `#[allow(dead_code)]` で抑制しない
  - 代わりに、実装時に追加する

### エラーハンドリング

- `color_eyre::Result<T>` を使用
- `?` 演算子でエラー伝播
- パニックは避け、Result で処理

### テスト

- ユニットテストは各モジュールの `#[cfg(test)]` 内に配置
- テストには `tempfile` を使用して一時ディレクトリを作成
- テスト関数名は `test_` プレフィックスを使用
