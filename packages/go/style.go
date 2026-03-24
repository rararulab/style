// Package rarastyle provides the rararulab design tokens for Go TUI applications.
//
// Uses charmbracelet/lipgloss colors. All semantic colors from the
// rara palette are available as package-level variables.
//
// Usage:
//
//	import rarastyle "github.com/rararulab/style/packages/go"
//
//	style := lipgloss.NewStyle().Foreground(rarastyle.Pop).Bold(true)
//	fmt.Println(style.Render("hello rara"))
package rarastyle

import "github.com/charmbracelet/lipgloss"

// ── Semantic colors (rararulab palette) ──
//
// Values from palette.json — keep in sync when palette changes.

var (
	// Primary text, headings — warm dark brown
	Ink = lipgloss.Color("#3D2B2E")
	// Secondary text, captions — dusty rose gray
	Muted = lipgloss.Color("#8C7478")
	// Tertiary text, placeholders — soft mauve
	Subtle = lipgloss.Color("#B8A0A4")
	// Borders, dividers — blush pink
	Line = lipgloss.Color("#F0DEE0")
	// Cards, panels — white
	Surface = lipgloss.Color("#ffffff")
	// Page background — clean white
	Bg = lipgloss.Color("#fafafa")
	// Accent — CTAs, highlights — coral pink
	Pop = lipgloss.Color("#E8788A")
	// Accent hover — deeper rose
	PopHover = lipgloss.Color("#D4606F")
	// Success, positive — soft mint
	Ok = lipgloss.Color("#7BC4A0")
	// Warning, caution — warm peach
	Warn = lipgloss.Color("#E8B86D")
	// Error, failure — muted red
	Err = lipgloss.Color("#D46A6A")
)

// ── ANSI 16-color mapping ──

var (
	AnsiBlack          = lipgloss.Color("#3D2B2E")
	AnsiRed            = lipgloss.Color("#D46A6A")
	AnsiGreen          = lipgloss.Color("#7BC4A0")
	AnsiYellow         = lipgloss.Color("#E8B86D")
	AnsiBlue           = lipgloss.Color("#9BAFD9")
	AnsiMagenta        = lipgloss.Color("#E8788A")
	AnsiCyan           = lipgloss.Color("#85C7C9")
	AnsiWhite          = lipgloss.Color("#F0DEE0")
	AnsiBrightBlack    = lipgloss.Color("#8C7478")
	AnsiBrightRed      = lipgloss.Color("#E89090")
	AnsiBrightGreen    = lipgloss.Color("#A0D9BF")
	AnsiBrightYellow   = lipgloss.Color("#F0D090")
	AnsiBrightBlue     = lipgloss.Color("#B8CCE8")
	AnsiBrightMagenta  = lipgloss.Color("#F0A0B0")
	AnsiBrightCyan     = lipgloss.Color("#A8D8D8")
	AnsiBrightWhite    = lipgloss.Color("#fafafa")
)
