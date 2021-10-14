use crate::AnsiStyled;
use std::borrow::{Cow, ToOwned};

use super::{Color, Style};

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
            //Color::Black      => write!(f, "30"),
            Color::Red => write!(w, "31"),
            //Colour::Green      => write!(f, "32"),
            //Colour::Yellow     => write!(f, "33"),
            Color::Blue => write!(w, "34"),
            //Colour::Purple     => write!(f, "35"),
            //Colour::Cyan       => write!(f, "36"),
            //Colour::White      => write!(f, "37"),
            //Colour::Fixed(num) => write!(f, "38;5;{}", &num),
            //Colour::RGB(r,g,b) => write!(f, "38;2;{};{};{}", &r, &g, &b),
        }
    }
}

/// The blue color.
pub const BLUE: Color = Color::Blue;

/// The red color.
pub const RED: Color = Color::Red;
