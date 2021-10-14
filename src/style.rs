use super::{AnsiStyled, Style};
use std::borrow::{Cow, ToOwned};

impl Default for Style {
    fn default() -> Self {
        Self {
            is_bold: false,
            is_italic: false,
            fg: None,
        }
    }
}

/// A style where the text will be printed in italic
pub static ITALIC: Style = Style {
    is_bold: false,
    is_italic: true,
    fg: None,
};

static RESET: &str = "\x1B[0m";
static HEAD: &str = "\x1B[";

impl Style {
    /// Builds a new instance of the style
    pub fn new() -> Self {
        Style::default()
    }

    /// Sets the text to bold
    pub fn bold(&self) -> Self {
        Self {
            is_bold: true,
            ..*self
        }
    }

    /// Sets the text to italic
    pub fn italic(&self) -> Self {
        Self {
            is_italic: true,
            ..*self
        }
    }

    /// Takes an input and gets the ansi styled back.
    pub fn to_ansi<'a, I, S: 'a + ToOwned + ?Sized>(self, input: I) -> AnsiStyled<'a, S>
    where
        I: Into<Cow<'a, S>>,
    {
        AnsiStyled {
            input: input.into(),
            style: self,
        }
    }

    fn is_plain(&self) -> bool {
        if self.is_bold {
            return false;
        }

        if self.is_italic {
            return false;
        }

        if self.fg.is_some() {
            return false;
        }

        true
    }

    pub(crate) fn write_suffix(&self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
        if self.is_plain() {
            Ok(())
        } else {
            write!(w, "{}", RESET)
        }
    }

    pub(crate) fn write_prefix(&self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
        if self.is_plain() {
            return Ok(());
        }

        write!(w, "{}", HEAD)?;

        let mut written_anything = false;

        {
            let mut write_char = |c| {
                if written_anything {
                    write!(w, ";")?;
                }
                written_anything = true;
                write!(w, "{}", c)?;
                Ok(())
            };

            if self.is_bold {
                write_char('1')?
            }

            if self.is_italic {
                write_char('3')?
            }

            if let Some(fg) = &self.fg {
                if written_anything {
                    write!(w, ";")?;
                }
                fg.write_foreground(w)?;
            }

            // All the codes end with an `m`, because reasons.
            write!(w, "m")?;
        }

        Ok(())
    }
}
