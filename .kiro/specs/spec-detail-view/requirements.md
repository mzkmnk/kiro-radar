# Requirements Document

## Introduction

Spec 詳細ビュー機能は、kiro-radar TUI アプリケーションにおいて、選択された Spec セットの詳細情報を表示する機能です。現在のリスト表示に加えて、ユーザーが特定の Spec の内容を深く確認できるようにし、requirements.md、design.md、tasks.md の内容をスクロール可能な形式で閲覧できるようにします。

## Glossary

- **Spec セット**: requirements.md、design.md、tasks.md の 3 つのファイルで構成される仕様ドキュメント群
- **リストビュー**: Spec セットの一覧を表示するメイン画面
- **詳細ビュー**: 選択された Spec セットの詳細情報を表示する画面
- **タブ**: 詳細ビュー内で requirements.md、design.md、tasks.md を切り替えるための UI 要素
- **アクティブタブ**: 現在表示されているファイルに対応するタブ
- **App**: アプリケーションの状態を管理する構造体
- **ViewMode**: アプリケーションの表示モード（リストビューまたは詳細ビュー）
- **スクロール状態**: 詳細ビュー内でのスクロール位置を管理する状態

## Requirements

### Requirement 1

**User Story:** ユーザーとして、選択した Spec の詳細情報を確認したいので、リストから詳細ビューに遷移できるようにしたい

#### Acceptance Criteria

1. WHEN ユーザーがリストビューで Spec を選択して Enter キーを押す THEN システムは詳細ビューに遷移する
2. WHEN 詳細ビューが表示される THEN システムは選択された Spec の名前をヘッダーに表示する
3. WHEN 詳細ビューに遷移する THEN システムはスクロール位置を初期化する

### Requirement 2

**User Story:** ユーザーとして、Spec の各ファイル内容を確認したいので、タブで切り替えて閲覧できるようにしたい

#### Acceptance Criteria

1. WHEN 詳細ビューが表示される THEN システムは requirements.md の内容をデフォルトで表示する
2. WHEN アクティブタブのファイルが存在しない THEN システムは「File not found」というメッセージを表示する
3. WHEN ファイル内容が表示領域を超える THEN システムはスクロール可能な状態で内容を表示する
4. WHEN タスクファイルが表示される THEN システムは完了タスクと未完了タスクを視覚的に区別して表示する

### Requirement 4

**User Story:** ユーザーとして、詳細ビュー内で長いドキュメントを閲覧したいので、スクロール操作ができるようにしたい

#### Acceptance Criteria

1. WHEN ユーザーが j キーまたは ↓ キーを押す THEN システムは表示内容を下方向にスクロールする
2. WHEN ユーザーが k キーまたは ↑ キーを押す THEN システムは表示内容を上方向にスクロールする
3. WHEN スクロール位置が最上部にある状態で上方向にスクロールしようとする THEN システムはスクロール位置を変更しない
4. WHEN スクロール位置が最下部にある状態で下方向にスクロールしようとする THEN システムはスクロール位置を変更しない
5. WHEN 表示内容が表示領域内に収まる THEN システムはスクロール操作を無効化する

### Requirement 5

**User Story:** ユーザーとして、詳細ビューからリストビューに戻りたいので、Esc キーで画面遷移できるようにしたい

#### Acceptance Criteria

1. WHEN ユーザーが詳細ビューで Esc キーを押す THEN システムはリストビューに遷移する
2. WHEN リストビューに戻る THEN システムは以前選択していた Spec の選択状態を保持する
3. WHEN リストビューに戻る THEN システムは詳細ビューのスクロール状態をクリアする

### Requirement 6

**User Story:** ユーザーとして、現在の操作方法を確認したいので、詳細ビューでも利用可能なキーバインドを表示してほしい

#### Acceptance Criteria

1. WHEN 詳細ビューが表示される THEN システムはフッターにキーバインド情報を表示する
2. WHEN フッターが表示される THEN システムは「Tab: Switch, ↑/k: Up, ↓/j: Down, Esc: Back, q: Quit」を表示する
3. WHEN ユーザーが q キーまたは Ctrl+C を押す THEN システムはアプリケーションを終了する

### Requirement 7

**User Story:** 開発者として、ビューモードの状態管理を適切に行いたいので、App 構造体に ViewMode を追加したい

#### Acceptance Criteria

1. WHEN App 構造体が初期化される THEN システムは ViewMode をリストビューに設定する
2. WHEN ViewMode が変更される THEN システムは適切なビューをレンダリングする
3. WHEN 詳細ビューに遷移する THEN システムは選択された Spec のインデックスを保持する

### Requirement 8

**User Story:** 開発者として、ファイル読み込みエラーを適切に処理したいので、エラーハンドリング機構を実装したい

#### Acceptance Criteria

1. WHEN ファイル読み込みに失敗する THEN システムはエラーメッセージを詳細ビューに表示する
2. WHEN ファイルが存在しない THEN システムは「File not found」メッセージを表示する
3. WHEN ファイル読み込み中に IO エラーが発生する THEN システムはエラー内容を含むメッセージを表示する

### Requirement 9

**User Story:** ユーザーとして、Spec の異なるファイルを素早く切り替えたいので、Tab キーでタブを順次切り替えられるようにしたい

#### Acceptance Criteria

1. WHEN ユーザーが詳細ビューで Tab キーを押す THEN システムは次のタブに切り替える
2. WHEN アクティブタブが tasks.md の状態で Tab キーを押す THEN システムは requirements.md タブに切り替える
3. WHEN タブが切り替わる THEN システムはスクロール位置を 0 にリセットする
4. WHEN タブが切り替わる THEN システムは新しいタブに対応するファイル内容を表示する
5. WHEN 詳細ビューが表示される THEN システムは右上にタブ一覧を表示する
6. WHEN タブ一覧が表示される THEN システムはアクティブタブを視覚的に強調表示する
