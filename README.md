# kiro-radar

A TUI (Terminal User Interface) dashboard for tracking Spec-Driven Development progress in AWS Kiro IDE.

![Rust](https://img.shields.io/badge/rust-2024-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Vibe Engineering](https://img.shields.io/badge/vibe%20engineering-active-blueviolet.svg)

## Overview

kiro-radar is a tool that visualizes the Spec-based development process, allowing you to monitor the progress of requirements definition, design, and implementation tasks in real-time from your terminal.

It automatically detects Spec sets under the `.kiro/specs` directory and tracks task progress in Markdown checklist format.

## Screenshot

![kiro-radar dashboard](.github/images/kiro-radar-image.png)

## Features

- Automatic detection of Spec sets under the `.kiro/specs` directory
- Parsing of Spec files (requirements.md, design.md, tasks.md)
- Task progress tracking in Markdown checklist format
- Visualization of overall progress and per-Spec progress
- Intuitive keyboard navigation (j/k, ↑↓)

## Installation

### From crates.io (Recommended)

```bash
cargo install kiro-radar
```

### From Source

#### Prerequisites

- Rust 1.75 or higher (Edition 2024 support)

#### Build

```bash
# Clone the repository
git clone https://github.com/mzkmnk/kiro-radar.git
cd kiro-radar

# Build and install
cargo install --path .
```

## Usage

```bash
# Run in development mode
cargo run
```

### Key Bindings

| Key                    | Action                |
| ---------------------- | --------------------- |
| `j` / `↓`              | Move to next item     |
| `k` / `↑`              | Move to previous item |
| `q` / `Esc` / `Ctrl+C` | Exit application      |

### Spec File Structure

kiro-radar expects the following directory structure:

```
.kiro/
└── specs/
    └── {spec-name}/
        ├── requirements.md  # Requirements definition
        ├── design.md        # Design document
        └── tasks.md         # Implementation tasks
```

`tasks.md` should be written in Markdown checklist format:

```markdown
# Implementation Plan

- [ ] 1. Task 1
- [x] 2. Task 2 (completed)
- [ ] 3. Task 3
```

## Tech Stack

- **Language**: Rust Edition 2024
- **TUI Framework**: [ratatui](https://ratatui.rs) v0.29.0
- **Terminal Control**: [crossterm](https://docs.rs/crossterm) v0.28.1
- **Error Handling**: [color-eyre](https://docs.rs/color-eyre) v0.6.3

## Architecture

kiro-radar adopts an event-driven architecture and consists of the following modules:

- **app**: Application state management and main loop
- **events**: Keyboard and mouse event handling
- **ui**: UI rendering logic
- **spec**: Spec file parsing (finder, parser)

For details, see [.kiro/steering/structure.md](.kiro/steering/structure.md).

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Display test output
cargo test -- --nocapture
```

### Code Formatting

```bash
# Apply formatting
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting

```bash
# Run Clippy
cargo clippy

# Stricter linting
cargo clippy -- -D warnings
```

## License

Copyright (c) mzkmnk <mzk.mnk.dev@gmail.com>

This project is licensed under the MIT license ([LICENSE](./LICENSE))
