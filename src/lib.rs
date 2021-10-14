//! A rust library for distance functions.
#![deny(missing_docs)]
#![deny(unreachable_pub)]

use std::borrow::{Cow, ToOwned};

mod ansi;
mod color;
mod style;

/// The style of the text
#[derive(Clone, Copy)]
pub struct Style {
    is_bold: bool,
    is_italic: bool,
    fg: Option<Color>,
    bg: Option<Color>,
}

/// The color of the foreground or background
#[derive(Clone, Copy)]
pub enum Color {
    /// Black color
    Black,
    /// Red color
    Red,
    /// Green color
    Green,
    /// Yellow color
    Yellow,
    /// Blue color
    Blue,
    /// Purple color
    Purple,
    /// Cyan color
    Cyan,
    /// White color
    White,
    /// A colour number from 0 to 255, for use in 256-colour terminal environments.
    Fixed(u8),
    /// A 24-bit RGB color, as specified by ISO-8613-3.
    RGB(u8, u8, u8),
}

/// An Ansi style.
pub struct AnsiStyled<'a, S: 'a + ToOwned + ?Sized> {
    style: Style,
    input: Cow<'a, S>,
}

pub use crate::color::{BLACK, BLUE, CYAN, GREEN, PURPLE, RED, WHITE, YELLOW};
pub use crate::style::ITALIC;
