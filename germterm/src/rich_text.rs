//! Stylized text.

use crate::color::Color;
use bitflags::bitflags;
use std::sync::Arc;

bitflags! {
    /// Attributes that can be applied to drawn text.
    ///
    /// Some attributes are **internal to the renderer** and **not part of the
    /// public API**. They may change or be removed at any time.
    ///
    /// Internal (do-not-use) attributes:
    /// - `TWOXEL`
    /// - `OCTAD`
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Attributes: u8 {
        // Standard terminal flags
        const BOLD          = 0b_00000001;
        const ITALIC        = 0b_00000010;
        const UNDERLINED    = 0b_00000100;
        const HIDDEN        = 0b_00001000;
        // Internal flags
        /// # WARNING
        /// This flag is **not part of the public API**.
        /// Using it may cause rendering glitches.
        ///
        /// Incompatible with:
        /// - [`Attributes::OCTAD`]
        /// - [`Attributes::BLOCKTAD`]
        const TWOXEL        = 0b_00010000;
        /// # WARNING
        /// This flag is **not part of the public API**.
        /// Using it may cause rendering glitches.
        ///
        /// Incompatible with:
        /// - [`Attributes::TWOXEL`]
        /// - [`Attributes::BLOCKTAD`]
        const OCTAD         = 0b_00100000;
        /// # WARNING
        /// This flag is **not part of the public API**.
        /// Using it may cause rendering glitches.
        ///
        /// Incompatible with:
        /// - [`Attributes::TWOXEL`]
        /// - [`Attributes::OCTAD`]
        const BLOCKTAD      = 0b_01000000;


    }
}

/// Styled text representation.
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
}

impl RichText {
    /// Creates a new `RichText` with default styling.
    ///
    /// To customize the style, use the following builder methods:
    /// - [`RichText::fg`]
    /// - [`RichText::bg`]
    /// - [`RichText::attributes`]
    ///
    /// `&str` and `String` types can be turned `into()`, which are converted into [`RichText`].
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: Arc::new(text.into()),
            fg: Color::WHITE,
            bg: Color::CLEAR,
            attributes: Attributes::empty(),
        }
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    pub fn attributes(mut self, attributes: Attributes) -> Self {
        self.attributes = attributes;
        self
    }
}

impl From<String> for RichText {
    fn from(s: String) -> Self {
        RichText::new(s)
    }
}

impl<'a> From<&'a str> for RichText {
    fn from(s: &'a str) -> Self {
        RichText::new(s)
    }
}
