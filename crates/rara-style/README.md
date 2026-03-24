# rara-style

rararulab unified CLI/TUI style — soft pink, cute, feminine palette.

Zero dependencies by default. Respects `NO_COLOR` / `CLICOLOR` / `CLICOLOR_FORCE`.

## Usage

```rust
use rara_style::{OK, ERR, BOLD, RESET};

// Auto-reset via paint()
eprintln!("{}", OK.paint("build passed"));
eprintln!("{}", ERR.paint("error:"));

// Manual reset
eprintln!("{}important{}", BOLD, RESET);
```

## Ratatui support

```toml
rara-style = { version = "0.1", features = ["ratatui"] }
```

```rust
use rara_style::tui;

let style = ratatui::style::Style::default().fg(tui::POP).bg(tui::BG);
```

## Hex values

```rust
use rara_style::hex;

println!("accent color: {}", hex::POP); // "#E8788A"
```
