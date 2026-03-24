# rara — Agent Guidelines

## Purpose

Unified design token system for all rararulab projects. The visual identity is soft pink, cute, and feminine.

**This is the single source of truth for all visual decisions across rararulab.**

## Architecture

```
palette.json              ← THE canonical source for all color/token values.
scripts/build.ts          ← Generates web CSS, terminal configs, docs from palette.json.
docs/index.html           ← Generated style guide (GitHub Pages).
platforms/                ← Generated output (gitignored). NEVER edit directly.
  web/tokens.css          ← CSS custom properties
  terminal/ansi.json      ← ANSI 16-color mapping
  terminal/alacritty.toml

crates/rara-style/        ← Rust crate (published to crates.io). Hand-maintained source.
  Cargo.toml
  src/lib.rs

packages/go/              ← Go module (tagged for go get). Hand-maintained source.
  go.mod
  style.go
```

### What is generated vs hand-maintained

| Path                 | Maintained by            | Gitignored |
| -------------------- | ------------------------ | ---------- |
| `platforms/*`        | `build.ts` (generated)   | Yes        |
| `docs/index.html`    | `build.ts` (generated)   | No         |
| `crates/rara-style/` | Hand-written Rust source | No         |
| `packages/go/`       | Hand-written Go source   | No         |

When `palette.json` values change, update **both** the crate and Go package to match.

## How to Use in Your Project

### Rust CLI / TUI

```bash
cargo add rara-style                      # CLI (zero deps)
cargo add rara-style --features ratatui   # TUI (ratatui colors)
```

```rust
use rara_style::{OK, ERR, BOLD, RESET};

// Auto-reset via paint()
eprintln!("{}", OK.paint("build passed"));

// Manual reset
eprintln!("{}heading{}", BOLD, RESET);

// Ratatui (with feature)
use rara_style::tui;
let style = ratatui::style::Style::default().fg(tui::POP).bg(tui::BG);

// Raw hex values
assert_eq!(rara_style::hex::POP, "#E8788A");
```

Respects `NO_COLOR` / `CLICOLOR` / `CLICOLOR_FORCE` per <https://no-color.org/>.

### Go TUI (lipgloss)

```bash
go get github.com/rararulab/style/packages/go
```

```go
import rarastyle "github.com/rararulab/style/packages/go"

style := lipgloss.NewStyle().Foreground(rarastyle.Pop).Bold(true)
```

### Web (HTML/CSS)

Link or copy `platforms/web/tokens.css`, then use variables:

```css
color: var(--color-ink);
background: var(--color-bg);
border: 1px solid var(--color-line);
font-family: var(--font-sans);
```

### Terminal Emulator

Import `platforms/terminal/alacritty.toml` into your Alacritty config, or read `platforms/terminal/ansi.json` for other emulators.

## Commands

```bash
npm run build          # Regenerate platforms/ and docs from palette.json
npm run format         # Format source files with Prettier
npm run format:check   # Check formatting (used in pre-commit)
```

## Publishing

- **Rust**: `cd crates/rara-style && cargo publish`
- **Go**: Tag with `packages/go/vX.Y.Z` for Go module proxy
- **Web/Terminal**: Generated on build, no publishing needed

## Critical Invariants

- `palette.json` is the canonical source for color values.
- `crates/rara-style/` and `packages/go/` must stay in sync with palette.json.
- Colors use role-based names (`ink`, `muted`, `pop`, `ok`, `err`), NEVER platform-specific names.
- All generated files (`platforms/`) include a "do not edit" header.

## What NOT To Do

- Do NOT edit files in `platforms/` — they will be overwritten on next build.
- Do NOT add platform-specific color names to `palette.json` (no `css-bg`, `ansi-red`).
- Do NOT add new colors without semantic justification — the palette is intentionally minimal.

## Token Naming Convention

| Layer  | Pattern              | Example              |
| ------ | -------------------- | -------------------- |
| Color  | `color.{role}`       | `color.ink`          |
| ANSI   | `ansi.{name}`        | `ansi.bright-blue`   |
| Type   | `typography.{scale}` | `typography.size.lg` |
| Space  | `spacing.{step}`     | `spacing.4` (16px)   |
| Radius | `radius.{size}`      | `radius.md` (8px)    |
| Motion | `motion.{speed}`     | `motion.fast`        |

## Dependencies

- **Upstream**: none (this is the root of the design system)
- **Downstream**: rararulab-site, rara, kotoba, tenki, cli-template, and all future projects
