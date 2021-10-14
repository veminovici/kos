use super::{AnsiStyled, Style};
use std::borrow::{Cow, ToOwned};
use std::fmt::{Debug, Display};

impl<'a, S: 'a + ToOwned + ?Sized> AnsiStyled<'a, S> {
    /// Creates a new ANSI styled input
    pub fn new<I>(style: Style, input: I) -> Self
    where
        I: Into<Cow<'a, S>>,
    {
        AnsiStyled {
            style,
            input: input.into(),
        }
    }
}

//
// Display
//

impl<'a, S: 'a + ToOwned + Display + ?Sized> Display for AnsiStyled<'a, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w: &mut dyn std::fmt::Write = f;
        self.write_display(w)
    }
}

impl<'a, S: 'a + ToOwned + Display + ?Sized> AnsiStyled<'a, S> {
    fn write_display(&self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.style.write_prefix(w)?;
        write!(w, "{}", self.input.as_ref())?;
        self.style.write_suffix(w)
    }
}

//
// Debug
//

impl<'a, S: 'a + ToOwned + Debug + ?Sized> Debug for AnsiStyled<'a, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w: &mut dyn std::fmt::Write = f;
        self.write_debug(w)
    }
}

impl<'a, S: 'a + ToOwned + Debug + ?Sized> AnsiStyled<'a, S> {
    fn write_debug(&self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.style.write_prefix(w)?;
        write!(w, "{:?}", self.input.as_ref())?;
        self.style.write_suffix(w)
    }
}
