//! Stylized text.

use crate::{cell::CellFormat, color::Color};
use bitflags::bitflags;
use std::sync::Arc;

bitflags! {
    /// Attributes that can be applied to drawn text.
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Attributes: u8 {
        const BOLD          = 0b_00000001;
        const ITALIC        = 0b_00000010;
        const UNDERLINED    = 0b_00000100;
        const HIDDEN        = 0b_00001000;
        const NO_FG_COLOR   = 0b_00010000;
        const NO_BG_COLOR   = 0b_00100000;
    }
}

/// Stylized text representation.
///
/// Bundles together text, foreground color, background color and attributes.
///
/// # Conversions
/// `RichText` can be created from the following types:
/// - `String`
/// - `&str`
#[derive(Clone)]
pub struct RichText {
    pub text: Arc<String>,
    pub fg: Color,
    pub bg: Color,
    pub attributes: Attributes,
    pub(crate) cell_format: CellFormat,
}

impl RichText {
    /// Creates a new `RichText` with default styling.
    ///
    /// To customize the style, use the following builder methods:
    /// - [`RichText::withg_fg()`]
    /// - [`RichText::with_bg()`]
    /// - [`RichText::with_attributes()`]
    ///
    /// `&str` and `String` types can be turned `into()`, which are converted into [`RichText`].
    #[inline]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: Arc::new(text.into()),
            fg: Color::WHITE,
            bg: Color::CLEAR,
            attributes: Attributes::empty(),
            cell_format: CellFormat::Standard,
        }
    }

    #[inline]
    pub fn with_fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    #[inline]
    pub fn with_bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    #[inline]
    pub fn with_attributes(mut self, attributes: Attributes) -> Self {
        self.attributes = attributes;
        self
    }

    #[inline]
    pub(crate) fn with_cell_format(mut self, format: CellFormat) -> Self {
        self.cell_format = format;
        self
    }
}

impl From<String> for RichText {
    #[inline]
    fn from(s: String) -> Self {
        RichText::new(s)
    }
}

impl<'a> From<&'a str> for RichText {
    #[inline]
    fn from(s: &'a str) -> Self {
        RichText::new(s)
    }
}
