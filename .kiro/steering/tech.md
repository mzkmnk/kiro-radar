# 技術スタック

## 言語・エディション

- **Rust**: Edition 2024

## 主要ライブラリ

### UI フレームワーク

- **ratatui** (v0.29.0): TUI（Terminal User Interface）フレームワーク
  - ターミナルベースの UI コンポーネントを提供
  - ウィジェット、レイアウト、スタイリング機能
  - ドキュメント: <https://docs.rs/ratatui/latest/ratatui/>

### ターミナル制御

- **crossterm** (v0.28.1): クロスプラットフォームなターミナル操作ライブラリ
  - キーボード・マウスイベントの処理
  - ターミナルの初期化と復元

### エラーハンドリング

- **color-eyre** (v0.6.3): 人間が読みやすいエラーレポート
  - スタックトレースの可視化
  - エラーコンテキストの提供

## ビルドシステム

### Cargo コマンド

```bash
# ビルド
cargo build

# リリースビルド
cargo build --release

# 実行
cargo run

# テスト実行
cargo test

# コードフォーマット
cargo fmt

# リント
cargo clippy

# 依存関係の更新確認
cargo outdated
```

## プロジェクト設定

- **パッケージ名**: kiro-radar
- **バージョン**: 0.1.0
- **ライセンス**: MIT
