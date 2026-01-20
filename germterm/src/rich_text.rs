use std::sync::Arc;

use bitflags::bitflags;

use crate::color::Color;

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Attributes: u8 {
        // Crossterm specific flags
        const BOLD       = 0b_0000_0001;
        const ITALIC     = 0b_0000_0010;
        const UNDERLINED = 0b_0000_0100;
        const HIDDEN     = 0b_0000_1000;
        // Renderer specific metadata
        //
        // Used internally to help the compositor identify drawing formats
        /// Incompatible with OCTAD
        const TWOXEL     = 0b_0001_0000;
        /// Incompatible with TWOXEL
        const OCTAD      = 0b_0010_0000;
    }
}

#[derive(Clone)]
pub struct RichText {
    pub text: Arc<String>,
    pub fg: Color,
    pub bg: Color,
    pub attributes: Attributes,
}

impl RichText {
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
