use super::{AnsiStyled, Color, Style};
use std::borrow::{Cow, ToOwned};
use std::fmt::Debug;

impl Default for Style {
    fn default() -> Self {
        Self {
            is_bold: false,
            is_italic: false,
            is_underline: false,
            is_dimmed: false,
            is_blink: false,
            is_hidden: false,
            is_reverse: false,
            is_strikethrough: false,
            fg: None,
            bg: None,
        }
    }
}

impl Debug for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "b={} i={} u={} fg={:?} bg={:?}",
            self.is_bold, self.is_italic, self.is_underline, self.fg, self.bg
        )
    }
}

/// A style where the text will be printed in italic
pub static ITALIC: Style = Style {
    is_bold: false,
    is_italic: true,
    is_underline: false,
    is_blink: false,
    is_dimmed: false,
    is_hidden: false,
    is_reverse: false,
    is_strikethrough: false,
    fg: None,
    bg: None,
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

    /// Sets the text to underlined
    pub fn underline(&self) -> Self {
        Self {
            is_underline: true,
            ..*self
        }
    }

    /// Sets the text to blink
    pub fn blink(&self) -> Self {
        Self {
            is_blink: true,
            ..*self
        }
    }

    /// Sets the text to dimmed
    pub fn dimmed(&self) -> Self {
        Self {
            is_dimmed: true,
            ..*self
        }
    }

    /// Sets the text to hidden
    pub fn hidden(&self) -> Self {
        Self {
            is_hidden: true,
            ..*self
        }
    }

    /// Sets the text to reverse
    pub fn reverse(&self) -> Self {
        Self {
            is_reverse: true,
            ..*self
        }
    }

    /// Sets the text to reverse
    pub fn strikethrough(&self) -> Self {
        Self {
            is_strikethrough: true,
            ..*self
        }
    }

    /// Sets the foreground color
    pub fn fg(&self, color: Color) -> Self {
        Self {
            fg: Some(color),
            ..*self
        }
    }

    /// Sets the background color
    pub fn bg(&self, color: Color) -> Self {
        Self {
            bg: Some(color),
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

        if self.is_blink {
            return false;
        }

        if self.is_dimmed {
            return false;
        }

        if self.is_hidden {
            return false;
        }

        if self.is_reverse {
            return false;
        }

        if self.is_strikethrough {
            return false;
        }

        if self.is_underline {
            return false;
        }

        if self.fg.is_some() {
            return false;
        }

        if self.bg.is_some() {
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

            if self.is_dimmed {
                write_char('2')?
            }

            if self.is_italic {
                write_char('3')?
            }

            if self.is_underline {
                write_char('4')?
            }

            if self.is_blink {
                write_char('5')?
            }

            if self.is_reverse {
                write_char('7')?
            }

            if self.is_hidden {
                write_char('8')?
            }

            if self.is_strikethrough {
                write_char('9')?
            }

            if let Some(bg) = &self.bg {
                if written_anything {
                    write!(w, ";")?;
                }
                written_anything = true;
                bg.write_background(w)?;
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
