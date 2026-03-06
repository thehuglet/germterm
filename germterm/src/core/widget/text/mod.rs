use crate::core::DisplayWidth;

pub mod line;
pub mod span;

pub trait LineWidth {
    fn width(&self, display_width: &DisplayWidth) -> u16;
}
