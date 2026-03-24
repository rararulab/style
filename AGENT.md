# rara — Agent Guidelines

## Purpose

Unified design token system for all rararulab projects. One `palette.json` generates platform-specific theme files for web, CLI, TUI, and desktop. The visual identity is soft pink, cute, and feminine.

**This is the single source of truth for all visual decisions across rararulab.**

## Architecture

```
palette.json          ← THE canonical source. Edit ONLY this file for color/token changes.
scripts/build.ts      ← Reads palette.json, generates all platform outputs.
docs/index.html       ← Generated style guide (GitHub Pages).
platforms/            ← Generated output. NEVER edit directly — always regenerate.
  web/tokens.css      ← CSS custom properties
  terminal/ansi.json  ← ANSI 16-color mapping
  terminal/alacritty.toml
  tui/theme.rs        ← Ratatui color constants
  cli/theme.rs        ← ANSI escape code constants for CLI output
```

## How to Use in Your Project

### Web (HTML/CSS)

Link or copy `platforms/web/tokens.css`, then use variables:

```css
color: var(--color-ink);
background: var(--color-bg);
border: 1px solid var(--color-line);
font-family: var(--font-sans);
```

### CLI (Rust)

Copy `platforms/cli/theme.rs` into your project:

```rust
mod style;
println!("{}Error:{} something broke", style::ERR, style::RESET);
```

### TUI / Ratatui (Rust)

Copy `platforms/tui/theme.rs` into your project:

```rust
mod theme;
let style = Style::default().fg(theme::POP);
```

### Terminal Emulator

Import `platforms/terminal/alacritty.toml` into your Alacritty config, or read `platforms/terminal/ansi.json` for other emulators.

## Commands

```bash
npm run build          # Regenerate all platform outputs from palette.json
npm run format         # Format source files with Prettier
npm run format:check   # Check formatting (used in pre-commit)
```

## Critical Invariants

- `palette.json` is the ONLY file you edit for visual changes. Everything in `platforms/` is generated.
- After changing `palette.json`, run `npm run build` to regenerate outputs.
- Colors use role-based names (`ink`, `muted`, `pop`, `ok`, `err`), NEVER platform-specific names.
- ANSI mappings must stay in sync with the semantic palette — `ansi.blue` = `color.pop`.
- All generated files include a "do not edit" header.

## What NOT To Do

- Do NOT edit files in `platforms/` — they will be overwritten on next build.
- Do NOT add platform-specific color names to `palette.json` (no `css-bg`, `ansi-red`).
- Do NOT add new colors without semantic justification — the palette is intentionally minimal.
- Do NOT remove the ANSI section — CLI/TUI projects depend on it.

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
