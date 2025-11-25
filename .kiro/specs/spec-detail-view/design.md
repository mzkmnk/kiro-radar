# Design Document

## Overview

Spec 詳細ビュー機能は、既存の kiro-radar TUI アプリケーションに新しい画面モードを追加します。現在のリスト表示（ListView）に加えて、選択された Spec の詳細情報を表示する詳細ビュー（DetailView）を実装します。

この機能により、ユーザーは requirements.md、design.md、tasks.md の内容を TUI 内で直接閲覧でき、外部エディタを開くことなく Spec の全体像を把握できるようになります。

## Architecture

### 状態管理の拡張

現在の `App` 構造体を拡張し、ビューモードとタブ状態の概念を導入します：

```rust
pub enum ViewMode {
    List,
    Detail { spec_index: usize },
}

pub enum DetailTab {
    Requirements,
    Design,
    Tasks,
}

pub struct App {
    pub running: bool,
    pub spec_sets: Vec<SpecSet>,
    pub list_state: ListState,
    pub view_mode: ViewMode,           // 新規追加
    pub detail_scroll: usize,          // 新規追加
    pub active_tab: DetailTab,         // 新規追加
}
```

### モジュール構成

既存のモジュール構成を維持しつつ、以下を拡張：

- **app.rs**: ViewMode の管理、詳細ビューへの遷移ロジック
- **events.rs**: ViewMode に応じたキーイベント処理の分岐
- **ui.rs**: ViewMode に応じたレンダリング処理の分岐
- **spec/reader.rs** (新規): Spec ファイルの読み込みロジック

### イベントフロー

```mermaid
stateDiagram-v2
    [*] --> ListView
    ListView --> DetailView: Enter キー
    DetailView --> ListView: Esc キー
    DetailView --> [*]: q / Ctrl+C
    ListView --> [*]: q / Ctrl+C

    state DetailView {
        [*] --> RequirementsTab
        RequirementsTab --> DesignTab: Tab キー
        DesignTab --> TasksTab: Tab キー
        TasksTab --> RequirementsTab: Tab キー

        RequirementsTab --> RequirementsTab: j/k, ↑↓
        DesignTab --> DesignTab: j/k, ↑↓
        TasksTab --> TasksTab: j/k, ↑↓
    }
```

## Components and Interfaces

### ViewMode 列挙型

ビューの状態を表現する列挙型：

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ViewMode {
    List,
    Detail { spec_index: usize },
}
```

### DetailTab 列挙型（新規）

詳細ビュー内のタブ状態を表現する列挙型：

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum DetailTab {
    Requirements,
    Design,
    Tasks,
}
```

### App 構造体の拡張

```rust
pub struct App {
    pub running: bool,
    pub spec_sets: Vec<SpecSet>,
    pub list_state: ListState,
    pub view_mode: ViewMode,
    pub detail_scroll: usize,
    pub active_tab: DetailTab,
}
```

新規フィールド：

- `view_mode`: 現在のビューモード
- `detail_scroll`: 詳細ビュー内のスクロール位置（行単位）
- `active_tab`: 詳細ビューで現在アクティブなタブ

新規メソッド：

- `enter_detail_view()`: 詳細ビューに遷移（デフォルトタブ: Requirements）
- `exit_detail_view()`: リストビューに戻る
- `scroll_down()`: 詳細ビューを下にスクロール
- `scroll_up()`: 詳細ビューを上にスクロール
- `next_tab()`: 次のタブに切り替え（スクロール位置をリセット）

### SpecContent 構造体（新規）

Spec ファイルの内容を保持する構造体：

```rust
pub struct SpecContent {
    pub requirements: Option<String>,
    pub design: Option<String>,
    pub tasks: Option<String>,
}
```

各フィールドは `Option<String>` 型で、ファイルが正常に読み込めた場合は `Some(内容)`、読み込めなかった場合は `None` を保持します。エラーの詳細はログに記録されます。

### spec/reader.rs（新規モジュール）

Spec ファイルを読み込む責務を持つモジュール：

```rust
pub fn read_spec_content(spec_set: &SpecSet) -> SpecContent
```

- 各ファイル（requirements.md、design.md、tasks.md）の読み込みを試行
- ファイルが存在しない場合やエラーの場合は `None` を返す
- IO エラーはログに記録し、`None` を返す

## Data Models

### ViewMode

```rust
pub enum ViewMode {
    List,
    Detail { spec_index: usize },
}
```

### DetailTab

```rust
pub enum DetailTab {
    Requirements,
    Design,
    Tasks,
}
```

### SpecContent

```rust
pub struct SpecContent {
    pub requirements: Option<String>,
    pub design: Option<String>,
    pub tasks: Option<String>,
}
```

### App（拡張後）

```rust
pub struct App {
    pub running: bool,
    pub spec_sets: Vec<SpecSet>,
    pub list_state: ListState,
    pub view_mode: ViewMode,
    pub detail_scroll: usize,
    pub active_tab: DetailTab,
}
```

## Correctness Properties

_A property is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees._

### Property Reflection

以下のプロパティの冗長性を分析しました：

**統合可能なプロパティ:**

- 2.1, 2.3, 3.1（各ファイルの内容表示）→ 単一のプロパティ「ファイル内容表示」に統合
- 2.5, 3.4（スクロール可能状態）→ 重複のため 3.4 を削除
- 4.1, 4.2（スクロール動作）→ 単一のプロパティ「スクロール操作」に統合

**エッジケースとして扱うもの:**

- 4.3, 4.4（境界条件）→ プロパティテストのジェネレータで処理

**残すプロパティ:**
上記の統合後、以下の独立したプロパティを定義します。

### Correctness Properties

Property 1: ビューモード遷移の正確性
_For any_ 有効な Spec インデックスに対して、リストビューで Enter キーイベントを処理すると、ViewMode が Detail { spec_index } に変更される
**Validates: Requirements 1.1, 7.3**

Property 2: ビューモード復帰の正確性
_For any_ 詳細ビュー状態に対して、Esc キーイベントを処理すると、ViewMode が List に変更される
**Validates: Requirements 5.1**

Property 3: 選択状態の保持（Round-trip）
_For any_ 選択インデックスに対して、詳細ビューへの遷移と復帰を行っても、リストの選択インデックスは変更されない
**Validates: Requirements 5.2**

Property 4: スクロール位置の初期化
_For any_ スクロール位置に対して、詳細ビューに遷移すると、detail_scroll が 0 にリセットされ、active_tab が Requirements に設定される
**Validates: Requirements 1.3, 2.1**

Property 5: スクロール位置のクリア
_For any_ スクロール位置に対して、リストビューに戻ると、detail_scroll が 0 にリセットされる
**Validates: Requirements 5.3**

Property 13: タブ切り替えの正確性
_For any_ DetailTab 状態に対して、Tab キーイベントを処理すると、次のタブに遷移する（Requirements → Design → Tasks → Requirements）
**Validates: Requirements 9.1, 9.2**

Property 14: タブ切り替え時のスクロールリセット
_For any_ スクロール位置に対して、タブを切り替えると、detail_scroll が 0 にリセットされる
**Validates: Requirements 9.3**

Property 6: スクロール操作の動作
_For any_ 有効なスクロール位置（0 < position < max）に対して、下キーイベントでスクロール位置が増加し、上キーイベントでスクロール位置が減少する
**Validates: Requirements 4.1, 4.2**

Property 7: ファイル内容の表示
_For any_ 存在する Spec ファイルとアクティブタブに対して、詳細ビューのレンダリング結果にそのタブに対応するファイルの内容が含まれる
**Validates: Requirements 2.1, 9.4**

Property 8: エラーメッセージの表示
_For any_ 読み込みに失敗した Spec ファイルに対して、詳細ビューのレンダリング結果にエラーメッセージが含まれる
**Validates: Requirements 8.1, 8.3**

Property 9: Spec 名の表示
_For any_ Spec に対して、詳細ビューのレンダリング結果にその Spec の名前が含まれる
**Validates: Requirements 1.2**

Property 10: キーバインド情報の表示
_For any_ 詳細ビュー状態に対して、レンダリング結果にキーバインド情報（Tab キーを含む）が含まれる
**Validates: Requirements 6.1, 6.2**

Property 15: タブ一覧の表示
_For any_ 詳細ビュー状態に対して、レンダリング結果に 3 つのタブ（Requirements、Design、Tasks）が表示される
**Validates: Requirements 9.5**

Property 16: アクティブタブの強調表示
_For any_ DetailTab 状態に対して、レンダリング結果でアクティブタブが視覚的に強調表示される
**Validates: Requirements 9.6**

Property 11: 終了操作の動作
_For any_ ビューモード（List または Detail）に対して、q キーまたは Ctrl+C イベントを処理すると、running フラグが false になる
**Validates: Requirements 6.3**

Property 12: ViewMode に応じたレンダリング分岐
_For any_ App 状態に対して、ViewMode が List の場合はリストビューがレンダリングされ、ViewMode が Detail の場合は詳細ビューがレンダリングされる
**Validates: Requirements 7.2**

## Error Handling

### ファイル読み込みエラー

Spec ファイルの読み込み時に発生する可能性のあるエラー：

1. **ファイル不在**: `Option<PathBuf>` が `None` の場合
   - 対応: "File not found" メッセージを表示
2. **IO エラー**: ファイル読み込み時の権限エラーなど
   - 対応: "Failed to read file" メッセージを表示
3. **UTF-8 デコードエラー**: ファイルが有効な UTF-8 でない場合
   - 対応: "Failed to read file" メッセージを表示

すべてのエラーは `Option::None` として扱われ、UI では統一されたエラーメッセージを表示します。詳細なエラー情報が必要な場合は、開発時にログ出力を追加できます。

### エラーハンドリング戦略

```rust
pub fn read_spec_file(path: &Option<PathBuf>) -> Option<String> {
    match path {
        None => None,
        Some(p) => {
            std::fs::read_to_string(p).ok()
        }
    }
}
```

エラーは `Option<String>` で表現し、`None` の場合は UI レンダリング時に適切なメッセージを表示します。詳細なエラー情報が必要な場合は、ログに記録します。

### パニックの回避

- すべてのファイル操作は `Option` 型で処理し、エラーは `ok()` で変換
- 配列アクセスは境界チェック済みのインデックスのみ使用
- `unwrap()` の使用を避け、適切なエラーハンドリングを実施
- 詳細なエラー情報が必要な場合は `color_eyre::Result<T>` を使用

## Testing Strategy

### ユニットテスト

以下の具体的なケースをユニットテストで検証：

1. **初期状態のテスト**

   - App::new() が ViewMode::List で初期化されること
   - detail_scroll が 0 で初期化されること

2. **エッジケースのテスト**

   - スクロール位置が 0 の時に上スクロールしても変化しないこと
   - スクロール位置が最大値の時に下スクロールしても変化しないこと
   - コンテンツが短い場合にスクロールが無効化されること

3. **エラーメッセージのテスト**
   - ファイルが存在しない場合に "File not found" が表示されること
   - 特定のキーバインド文字列が表示されること

### プロパティベーステスト

プロパティベーステストには **quickcheck** クレート（v1.0）を使用します。

各プロパティテストは以下の形式で実装：

```rust
#[quickcheck]
fn property_name(input: ArbitraryType) -> bool {
    // テストロジック
}
```

**設定:**

- 各プロパティテストは最低 100 回の反復を実行
- テスト失敗時は反例を出力

**タグ付け規則:**
各プロパティベーステストには、対応する設計ドキュメントのプロパティを明示するコメントを付与：

```rust
// **Feature: spec-detail-view, Property 1: ビューモード遷移の正確性**
#[quickcheck]
fn test_view_mode_transition(spec_index: usize) -> bool {
    // テストロジック
}
```

**実装するプロパティテスト:**

1. Property 1: ビューモード遷移の正確性
2. Property 2: ビューモード復帰の正確性
3. Property 3: 選択状態の保持（Round-trip）
4. Property 4: スクロール位置の初期化
5. Property 5: スクロール位置のクリア
6. Property 6: スクロール操作の動作
7. Property 7: ファイル内容の表示
8. Property 8: エラーメッセージの表示
9. Property 9: Spec 名の表示
10. Property 10: キーバインド情報の表示
11. Property 11: 終了操作の動作
12. Property 12: ViewMode に応じたレンダリング分岐
13. Property 13: タブ切り替えの正確性
14. Property 14: タブ切り替え時のスクロールリセット
15. Property 15: タブ一覧の表示
16. Property 16: アクティブタブの強調表示

### テスト対象モジュール

- **app.rs**: 状態遷移、スクロール操作のロジック
- **events.rs**: キーイベント処理の分岐
- **ui.rs**: レンダリング結果の検証（文字列マッチング）
- **spec/reader.rs**: ファイル読み込みとエラーハンドリング

## Implementation Notes

### UI レイアウト設計

詳細ビューは以下のレイアウトで構成：

```
┌─────────────────────────────────────────────────────────┐
│ [ KIRO RADAR - 0.1.0 ]                                  │ ← ヘッダー
├─────────────────────────────────────────────────────────┤
│ Spec: {spec_name}    [ Requirements | Design | Tasks ]  │ ← Spec 名 + タブ
│                       ^^^^^^^^^^^^                       │   (アクティブタブは強調)
├─────────────────────────────────────────────────────────┤
│                                                          │
│ {アクティブタブのファイル内容}                           │
│ ...                                                      │ ← スクロール可能
│ ...                                                      │
│                                                          │
├─────────────────────────────────────────────────────────┤
│ [ Tab: Switch, ↑/k: Up, ↓/j: Down, Esc: Back, q: Quit ] │ ← フッター
└─────────────────────────────────────────────────────────┘
```

タブの表示例：

- アクティブ: `[ Requirements ]` （反転表示または色付き）
- 非アクティブ: `Design` `Tasks` （通常表示）

### スクロール実装

- `detail_scroll` フィールドで現在のスクロール位置（行番号）を管理
- レンダリング時に `detail_scroll` から表示可能行数分のコンテンツを切り出し
- スクロール可能な最大位置 = 総行数 - 表示可能行数

### パフォーマンス考慮事項

- ファイル読み込みは詳細ビューへの遷移時に一度だけ実行
- 大きなファイルの場合、スクロール時の再レンダリングコストを最小化
- 文字列の分割は `lines()` イテレータを使用して効率的に処理

### 将来の拡張性

- 検索機能（ファイル内容の全文検索）
- タスクの編集機能（チェックボックスの切り替え）
- シンタックスハイライト（Markdown のレンダリング）
- マウスクリックによるタブ切り替え
