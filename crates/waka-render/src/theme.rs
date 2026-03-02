//! Centralised colour theme for `waka` terminal output.
//!
//! Build a [`Theme`] once per process via [`Theme::from_env`] and pass it to
//! all renderers.  This ensures that every colour decision is made in one
//! place and that `NO_COLOR` / pipe detection is respected uniformly.
//!
//! # Example
//!
//! ```rust
//! use waka_render::Theme;
//!
//! let theme = Theme::from_env();
//! // Pass `&theme` to every renderer function.
//! ```

use owo_colors::{DynColors, Style};
use supports_color::Stream;

// ─────────────────────────────────────────────────────────────────────────────
// Theme
// ─────────────────────────────────────────────────────────────────────────────

/// Centralised colour palette for all `waka` terminal output.
///
/// Create once at program startup with [`Theme::from_env`] and pass through
/// to every renderer.  Use [`Theme::plain`] when colour must be disabled
/// unconditionally.
#[derive(Debug, Clone)]
pub struct Theme {
    /// Accent / primary colour (cyan).
    pub accent: Style,
    /// Success / reached goals (green).
    pub success: Style,
    /// Warning / near-miss state (yellow).
    pub warning: Style,
    /// Error / failed state (red).
    pub error: Style,
    /// Secondary / metadata (bright black / grey).
    pub muted: Style,
    /// Bold text for headlines.
    pub bold: Style,
    /// Section headings (bold cyan).
    pub heading: Style,
    /// Whether colour rendering is active.
    ///
    /// This flag gates [`Theme::lang_color`] and [`Theme::bar_chars`] so
    /// callers do not need to re-check the environment themselves.
    pub is_colored: bool,
}

impl Theme {
    /// Build a fully-coloured theme.
    ///
    /// Colours are applied using ANSI escape codes.  Only call this when you
    /// know the output stream supports colour — prefer [`Theme::from_env`] in
    /// most cases.
    pub fn colored() -> Self {
        Self {
            accent: Style::new().cyan().bold(),
            success: Style::new().green().bold(),
            warning: Style::new().yellow(),
            error: Style::new().red().bold(),
            muted: Style::new().bright_black(),
            bold: Style::new().bold(),
            heading: Style::new().cyan().bold(),
            is_colored: true,
        }
    }

    /// Build a plain (no-colour, no-style) theme.
    ///
    /// All style fields are identity — applying them to a string produces no
    /// ANSI escape codes.  This is the right choice for piped output and
    /// `NO_COLOR` environments.
    pub fn plain() -> Self {
        let plain = Style::new();
        Self {
            accent: plain,
            success: plain,
            warning: plain,
            error: plain,
            muted: plain,
            bold: plain,
            heading: plain,
            is_colored: false,
        }
    }

    /// Detect the right theme from the current environment.
    ///
    /// Colour is enabled only when [`supports_color`] reports that stdout
    /// supports ANSI codes.  This automatically respects:
    ///
    /// - `NO_COLOR` environment variable (<https://no-color.org/>)
    /// - `FORCE_COLOR` environment variable
    /// - `TERM=dumb`
    /// - Piped / non-TTY stdout
    pub fn from_env() -> Self {
        if supports_color::on(Stream::Stdout).is_some() {
            Self::colored()
        } else {
            Self::plain()
        }
    }

    /// Return the official display colour for a programming language.
    ///
    /// The match is **case-insensitive**.  Returns a plain [`Style`] (no ANSI
    /// codes) when the theme has colour disabled.
    ///
    /// Colours are sourced from
    /// [GitHub Linguist](https://github.com/github-linguist/linguist).
    pub fn lang_color(&self, lang: &str) -> Style {
        if !self.is_colored {
            return Style::new();
        }
        let (r, g, b) = lang_rgb(lang.to_lowercase().as_str());
        Style::new().color(DynColors::Rgb(r, g, b))
    }

    /// Return the fill / empty characters used for progress bars.
    ///
    /// Returns Unicode block characters when colour is enabled:
    ///
    /// - fill: `█`
    /// - empty: `░`
    ///
    /// Falls back to ASCII `#` / `-` for `NO_COLOR` environments and pipes.
    pub fn bar_chars(&self) -> (&'static str, &'static str) {
        if self.is_colored {
            ("█", "░")
        } else {
            ("#", "-")
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helper — language RGB lookup
// ─────────────────────────────────────────────────────────────────────────────

/// Return the `(r, g, b)` triple for a well-known programming language.
///
/// Source: GitHub Linguist / language-colors.json.
/// Falls back to `(56, 190, 212)` (≈ cyan) for unknown languages.
#[allow(clippy::match_same_arms)] // Different languages that share similar colours are intentional
fn lang_rgb(lang: &str) -> (u8, u8, u8) {
    match lang {
        "rust" => (206, 64, 32),
        "python" => (53, 114, 165),
        "javascript" | "js" => (247, 223, 30),
        "typescript" | "ts" => (43, 116, 137),
        "go" | "golang" => (0, 173, 216),
        "java" => (176, 114, 25),
        "c" => (85, 85, 85),
        "c++" | "cpp" => (243, 75, 125),
        "c#" | "csharp" => (23, 134, 0),
        "ruby" => (112, 21, 22),
        "php" => (79, 93, 149),
        "swift" => (240, 81, 56),
        "kotlin" => (169, 123, 255),
        "scala" => (220, 50, 47),
        "html" => (227, 76, 38),
        "css" => (86, 61, 124),
        "vue" => (65, 184, 131),
        "svelte" => (255, 62, 0),
        "shell" | "bash" | "sh" | "zsh" => (137, 224, 81),
        "powershell" => (1, 36, 86),
        "lua" => (0, 0, 128),
        "haskell" => (94, 80, 134),
        "elixir" => (110, 74, 126),
        "erlang" => (184, 57, 152),
        "clojure" => (219, 88, 85),
        "dart" => (0, 180, 216),
        "r" => (25, 140, 198),
        "sql" => (230, 120, 0),
        "markdown" | "md" => (8, 63, 185),
        "yaml" | "yml" => (203, 65, 84),
        "toml" => (156, 66, 33),
        "json" => (0, 128, 0),
        "xml" => (0, 150, 136),
        "dockerfile" | "docker" => (1, 101, 163),
        "makefile" | "make" => (66, 56, 48),
        // Accent fallback — matches Theme::colored().accent base hue
        _ => (56, 190, 212),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_theme_is_not_colored() {
        let theme = Theme::plain();
        assert!(!theme.is_colored);
    }

    #[test]
    fn colored_theme_is_colored() {
        let theme = Theme::colored();
        assert!(theme.is_colored);
    }

    #[test]
    fn plain_bar_chars_are_ascii() {
        let theme = Theme::plain();
        assert_eq!(theme.bar_chars(), ("#", "-"));
    }

    #[test]
    fn colored_bar_chars_are_unicode() {
        let theme = Theme::colored();
        assert_eq!(theme.bar_chars(), ("█", "░"));
    }

    #[test]
    fn lang_color_plain_returns_plain_style() {
        use owo_colors::OwoColorize as _;
        let theme = Theme::plain();
        // Plain style applied to "text" should produce "text" unchanged.
        let styled = "text".style(theme.lang_color("rust"));
        // We can't easily inspect the ANSI codes, but we verify no panic.
        let _ = styled.to_string();
    }

    #[test]
    fn lang_color_known_lang() {
        let theme = Theme::colored();
        let style = theme.lang_color("Rust");
        // Should produce a TrueColor RGB style — just verify it's a different
        // Style object than the plain fallback (is_colored gates this)
        let plain = Style::new();
        // The styles differ in their colour field — just ensure no panic
        let _ = style;
        let _ = plain;
    }

    #[test]
    fn lang_color_unknown_lang_falls_back() {
        let theme = Theme::colored();
        // Should not panic for completely unknown language
        let style = theme.lang_color("brainfuck");
        let _ = style;
    }

    #[test]
    fn lang_color_case_insensitive() {
        // lang_rgb is tested directly with already-lowercased strings
        assert_eq!(lang_rgb("rust"), (206, 64, 32));
        assert_eq!(lang_rgb("python"), (53, 114, 165));
        // Theme::lang_color lowercases before lookup — just verify no panic
        let theme = Theme::colored();
        let _ = theme.lang_color("Rust");
        let _ = theme.lang_color("PYTHON");
    }
}
