use crate::AnsiStyled;
use std::borrow::{Cow, ToOwned};
use std::fmt::Debug;

use super::{Color, Style};

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Black => write!(f, "Black"),
            Self::Red => write!(f, "Red"),
            Self::Green => write!(f, "Green"),
            Self::Yellow => write!(f, "Yellow"),
            Self::Blue => write!(f, "Blue"),
            Self::Purple => write!(f, "Purple"),
            Self::Cyan => write!(f, "Cyan"),
            Self::White => write!(f, "White"),
            Self::Fixed(arg0) => f.debug_tuple("Fixed").field(arg0).finish(),
            Self::RGB(arg0, arg1, arg2) => f
                .debug_tuple("RGB")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
        }
    }
}

impl Color {
    /// Create a stle with the current color.
    fn normal(self) -> Style {
        Style {
            fg: Some(self),
            ..Style::default()
        }
    }

    /// Creates a style with the current color as foreground color and with bold text.
    pub fn bold(self) -> Style {
        Style {
            fg: Some(self),
            is_bold: true,
            ..Style::default()
        }
    }

    /// Creates a style using the current color as foregorund color and with italic text.
    pub fn italic(self) -> Style {
        Style {
            fg: Some(self),
            is_italic: true,
            ..Style::default()
        }
    }

    /// Creates a style using the current color as foregorund color and with underline text.
    pub fn underline(self) -> Style {
        Style {
            fg: Some(self),
            is_underline: true,
            ..Style::default()
        }
    }

    /// Creates a style using the current color as foregorund color and with blink text.
    pub fn blink(self) -> Style {
        Style {
            fg: Some(self),
            is_blink: true,
            ..Style::default()
        }
    }

    /// Creates a style using the current color as foregorund color and with dimmed text.
    pub fn dimmed(self) -> Style {
        Style {
            fg: Some(self),
            is_dimmed: true,
            ..Style::default()
        }
    }

    /// Creates a style using the current color as foregorund color and with reverse text.
    pub fn reverse(self) -> Style {
        Style {
            fg: Some(self),
            is_reverse: true,
            ..Style::default()
        }
    }

    /// Creates a style using the current color as foregorund color and with reverse text.
    pub fn hidden(self) -> Style {
        Style {
            fg: Some(self),
            is_hidden: true,
            ..Style::default()
        }
    }

    /// Creates a style using the current color as foregorund color and with reverse text.
    pub fn strikethrough(self) -> Style {
        Style {
            fg: Some(self),
            is_strikethrough: true,
            ..Style::default()
        }
    }

    /// Creates a style using the current color as foreground color and the other color as background color.
    pub fn bg(self, color: Color) -> Style {
        Style {
            fg: Some(self),
            bg: Some(color),
            ..Style::default()
        }
    }

    /// Creates a style using the current color as background color and the other color as foreground color.
    pub fn fg(self, color: Color) -> Style {
        Style {
            fg: Some(color),
            bg: Some(self),
            ..Style::default()
        }
    }

    /// Takes an input and gets the ansi styled back.
    pub fn to_ansi<'a, I, S: 'a + ToOwned + ?Sized>(self, input: I) -> AnsiStyled<'a, S>
    where
        I: Into<Cow<'a, S>>,
    {
        AnsiStyled {
            input: input.into(),
            style: self.normal(),
        }
    }

    pub(crate) fn write_foreground(&self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
        match *self {
            Color::Black => write!(w, "30"),
            Color::Red => write!(w, "31"),
            Color::Green => write!(w, "32"),
            Color::Yellow => write!(w, "33"),
            Color::Blue => write!(w, "34"),
            Color::Purple => write!(w, "35"),
            Color::Cyan => write!(w, "36"),
            Color::White => write!(w, "37"),
            Color::Fixed(num) => write!(w, "38;5;{}", &num),
            Color::RGB(r, g, b) => write!(w, "38;2;{};{};{}", &r, &g, &b),
        }
    }

    pub(crate) fn write_background(&self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
        match *self {
            Color::Black => write!(w, "40"),
            Color::Red => write!(w, "41"),
            Color::Green => write!(w, "42"),
            Color::Yellow => write!(w, "43"),
            Color::Blue => write!(w, "44"),
            Color::Purple => write!(w, "45"),
            Color::Cyan => write!(w, "46"),
            Color::White => write!(w, "47"),
            Color::Fixed(num) => write!(w, "48;5;{}", &num),
            Color::RGB(r, g, b) => write!(w, "48;2;{};{};{}", &r, &g, &b),
        }
    }
}

/// The black color.
pub const BLACK: Color = Color::Black;

/// The blue color.
pub const BLUE: Color = Color::Blue;

/// The red color.
pub const RED: Color = Color::Red;

/// The green color.
pub const GREEN: Color = Color::Green;

/// The yellow color.
pub const YELLOW: Color = Color::Yellow;

/// The purple color.
pub const PURPLE: Color = Color::Purple;

/// The cyan color.
pub const CYAN: Color = Color::Cyan;

/// The white color.
pub const WHITE: Color = Color::White;
