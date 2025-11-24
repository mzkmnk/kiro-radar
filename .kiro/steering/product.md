# プロダクト概要

## kiro-radar

AWS Kiro IDE における Spec-Driven Development の進捗を追跡するための TUI（Terminal User Interface）ダッシュボード。

### 目的

Spec ベースの開発プロセスを可視化し、要件定義、設計、実装タスクの進捗状況をターミナル上でリアルタイムに監視できるようにする。

### 主な機能

- `.kiro/specs` ディレクトリ配下の Spec セット自動検出
- Spec ファイル（requirements.md、design.md、tasks.md）の解析
- Markdown チェックリスト形式のタスク進捗トラッキング
- 全体進捗と Spec 別進捗の可視化
- キーボードナビゲーション（j/k、↑↓）による直感的な操作

### アーキテクチャ

- **イベント駆動**: crossterm によるキーボード・マウスイベント処理
- **モジュール分離**: app（状態管理）、ui（レンダリング）、events（イベント処理）、spec（解析ロジック）
- **エラーハンドリング**: color-eyre による人間が読みやすいエラー表示
