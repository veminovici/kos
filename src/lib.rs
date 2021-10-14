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
}

/// The color of the foreground or background
#[derive(Clone, Copy, Debug)]
pub enum Color {
    /// Red color
    Red,
    /// Blue color
    Blue,
}

/// An Ansi style.
pub struct AnsiStyled<'a, S: 'a + ToOwned + ?Sized> {
    style: Style,
    input: Cow<'a, S>,
}

pub use crate::color::{BLUE, RED};
pub use crate::style::ITALIC;
