#![doc = include_str!("../README.md")]

use std::fmt;
use std::sync::OnceLock;

/// Whether color output is enabled (checked once, cached).
///
/// Respects `NO_COLOR` / `CLICOLOR` / `CLICOLOR_FORCE` environment variables.
/// See <https://no-color.org/> and <https://bixense.com/clicolors/>.
pub fn color_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        if std::env::var("CLICOLOR_FORCE").is_ok_and(|v| v != "0") {
            return true;
        }
        if std::env::var("NO_COLOR").is_ok() {
            return false;
        }
        if std::env::var("CLICOLOR").is_ok_and(|v| v == "0") {
            return false;
        }
        std::io::IsTerminal::is_terminal(&std::io::stderr())
    })
}

/// An ANSI escape sequence that respects `NO_COLOR`.
///
/// ```rust
/// use rara_style::{OK, ERR, BOLD, RESET};
///
/// // Manual reset
/// eprintln!("{}build passed{}", OK, RESET);
///
/// // Auto-reset via paint()
/// eprintln!("{}", OK.paint("build passed"));
/// eprintln!("{}", BOLD.paint("important"));
/// ```
#[derive(Clone, Copy)]
pub struct Color {
    code: &'static str,
}

impl Color {
    /// Create a color from a raw ANSI escape sequence.
    pub const fn new(code: &'static str) -> Self {
        Self { code }
    }

    /// Wrap text with this color + automatic reset.
    pub const fn paint(self, text: &str) -> Painted<'_> {
        Painted { color: self, text }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if color_enabled() {
            f.write_str(self.code)
        } else {
            Ok(())
        }
    }
}

/// Text wrapped with a color + automatic reset.
pub struct Painted<'a> {
    color: Color,
    text:  &'a str,
}

impl fmt::Display for Painted<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if color_enabled() {
            write!(f, "{}{}\x1b[0m", self.color.code, self.text)
        } else {
            f.write_str(self.text)
        }
    }
}

// ── Modifiers ──

pub const RESET: Color = Color::new("\x1b[0m");
pub const BOLD: Color  = Color::new("\x1b[1m");
pub const DIM: Color   = Color::new("\x1b[2m");

// ── Foreground colors ──
//
// Values from palette.json — keep in sync when palette changes.

/// Primary text, headings — warm dark brown `#3D2B2E`
pub const INK: Color    = Color::new("\x1b[38;2;61;43;46m");
/// Secondary text, captions — dusty rose gray `#8C7478`
pub const MUTED: Color  = Color::new("\x1b[38;2;140;116;120m");
/// Tertiary text, placeholders — soft mauve `#B8A0A4`
pub const SUBTLE: Color = Color::new("\x1b[38;2;184;160;164m");
/// Accent — CTAs, highlights — coral pink `#E8788A`
pub const POP: Color    = Color::new("\x1b[38;2;232;120;138m");
/// Success, positive — soft mint `#7BC4A0`
pub const OK: Color     = Color::new("\x1b[38;2;123;196;160m");
/// Warning, caution — warm peach `#E8B86D`
pub const WARN: Color   = Color::new("\x1b[38;2;232;184;109m");
/// Error, failure — muted red `#D46A6A`
pub const ERR: Color    = Color::new("\x1b[38;2;212;106;106m");

// ── Background colors ──

pub const BG_POP: Color  = Color::new("\x1b[48;2;232;120;138m");
pub const BG_OK: Color   = Color::new("\x1b[48;2;123;196;160m");
pub const BG_WARN: Color = Color::new("\x1b[48;2;232;184;109m");
pub const BG_ERR: Color  = Color::new("\x1b[48;2;212;106;106m");

/// Raw hex values for config files, color pickers, etc.
pub mod hex {
    pub const INK: &str      = "#3D2B2E";
    pub const MUTED: &str    = "#8C7478";
    pub const SUBTLE: &str   = "#B8A0A4";
    pub const LINE: &str     = "#F0DEE0";
    pub const SURFACE: &str  = "#ffffff";
    pub const BG: &str       = "#fafafa";
    pub const POP: &str      = "#E8788A";
    pub const POP_HOVER: &str = "#D4606F";
    pub const OK: &str       = "#7BC4A0";
    pub const WARN: &str     = "#E8B86D";
    pub const ERR: &str      = "#D46A6A";
}

/// Ratatui color constants.
///
/// ```toml
/// rara-style = { version = "0.1", features = ["ratatui"] }
/// ```
///
/// ```rust,ignore
/// use rara_style::tui;
/// let style = ratatui::style::Style::default().fg(tui::POP).bg(tui::BG);
/// ```
#[cfg(feature = "ratatui")]
pub mod tui {
    use ratatui::style::Color as RatatuiColor;

    pub const INK: RatatuiColor     = RatatuiColor::Rgb(61, 43, 46);
    pub const MUTED: RatatuiColor   = RatatuiColor::Rgb(140, 116, 120);
    pub const SUBTLE: RatatuiColor  = RatatuiColor::Rgb(184, 160, 164);
    pub const LINE: RatatuiColor    = RatatuiColor::Rgb(240, 222, 224);
    pub const SURFACE: RatatuiColor = RatatuiColor::Rgb(255, 255, 255);
    pub const BG: RatatuiColor      = RatatuiColor::Rgb(250, 250, 250);
    pub const POP: RatatuiColor     = RatatuiColor::Rgb(232, 120, 138);
    pub const OK: RatatuiColor      = RatatuiColor::Rgb(123, 196, 160);
    pub const WARN: RatatuiColor    = RatatuiColor::Rgb(232, 184, 109);
    pub const ERR: RatatuiColor     = RatatuiColor::Rgb(212, 106, 106);
}
