use crate::core::widget::Widget;

/// Vertical position of a title on a block border.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TitlePosition {
    /// Render the title on the top border.
    #[default]
    Top,
    /// Render the title on the bottom border.
    Bottom,
}

/// Horizontal alignment of a title within its border.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TitleAlignment {
    /// Align the title to the left edge.
    #[default]
    Left,
    /// Center the title horizontally.
    Center,
    /// Align the title to the right edge.
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A widget displayed on a block's border with configurable position and alignment.
pub struct Title<W> {
    title: W,
    position: TitlePosition,
    alignment: TitleAlignment,
}

impl<W> Title<W> {
    /// Creates a new title with the given widget, defaulting to top-left placement.
    pub fn new(title: W) -> Self
    where
        W: Widget,
    {
        Self {
            title,
            position: TitlePosition::default(),
            alignment: TitleAlignment::default(),
        }
    }

    /// Gets the title's vertical position.
    #[inline]
    pub fn position(&self) -> TitlePosition {
        self.position
    }

    /// Sets the title's vertical position.
    #[inline]
    pub fn set_position(&mut self, position: TitlePosition) {
        self.position = position;
    }

    /// Builder-sets the title's vertical position.
    #[inline]
    pub fn with_position(mut self, position: TitlePosition) -> Self {
        self.set_position(position);
        self
    }

    /// Gets the title's horizontal alignment.
    #[inline]
    pub fn alignment(&self) -> TitleAlignment {
        self.alignment
    }

    /// Sets the title's horizontal alignment.
    #[inline]
    pub fn set_alignment(&mut self, alignment: TitleAlignment) {
        self.alignment = alignment;
    }

    /// Builder-sets the title's horizontal alignment.
    #[inline]
    pub fn with_alignment(mut self, alignment: TitleAlignment) -> Self {
        self.set_alignment(alignment);
        self
    }

    /// Returns a reference to the inner widget.
    #[inline]
    pub(crate) fn inner(&self) -> &W {
        &self.title
    }
}
