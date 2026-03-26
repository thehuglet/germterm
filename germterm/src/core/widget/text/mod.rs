use crate::core::DisplayWidth;

pub mod line;
pub mod span;

/// Determines the length of a piece of text needed to render properly.
///
/// The returned value is in cell units, as measured by the provided
/// [`DisplayWidth`] strategy.
pub trait LineWidth {
    /// Returns the length of `self` in cell units.
    ///
    /// The `display_width` parameter controls how individual characters
    /// and strings are measured (e.g. Unicode width vs. a custom scheme).
    fn width(&self, display_width: &DisplayWidth) -> u16;
}
