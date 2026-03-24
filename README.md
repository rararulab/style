# rara

Soft, cute, unified design tokens for all rararulab projects — web, CLI, TUI, desktop.

One `palette.json` generates platform-specific theme files.

**[Live Style Guide →](https://rararulab.github.io/style/)**

## Quick Start

```bash
npm install
npm run build
```

## Usage

### Web

```html
<link rel="stylesheet" href="platforms/web/tokens.css" />
```

```css
color: var(--color-ink);
background: var(--color-bg);
font-family: var(--font-sans);
```

### CLI (Rust)

Copy `platforms/cli/theme.rs` into your project:

```rust
mod style;
println!("{}ok{}", style::OK, style::RESET);
```

### TUI / Ratatui

Copy `platforms/tui/theme.rs` into your project:

```rust
mod theme;
let s = Style::default().fg(theme::POP);
```

### Terminal Emulator

Import `platforms/terminal/alacritty.toml` or read `ansi.json` for other emulators.

## Editing

1. Edit **only** `palette.json`
2. Run `npm run build`
3. Commit — pre-commit hook validates formatting and rebuilds

## Structure

```
palette.json            ← single source of truth (W3C DTCG)
scripts/build.ts        ← generates all platform outputs
platforms/              ← generated (gitignored)
docs/index.html         ← generated style guide (GitHub Pages)
```
