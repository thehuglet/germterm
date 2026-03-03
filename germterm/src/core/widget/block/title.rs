use crate::core::widget::Widget;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TitlePosition {
    #[default]
    Top,
    Bottom,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TitleAlignment {
    #[default]
    Left,
    Center,
    Right,
}

impl TitleAlignment {
    pub(crate) const fn as_index(self) -> usize {
        match self {
            Self::Left => 0,
            Self::Center => 1,
            Self::Right => 2,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Title<W> {
    title: W,
    position: TitlePosition,
    alignment: TitleAlignment,
}

impl<W> Title<W> {
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

    #[inline]
    pub fn position(&self) -> TitlePosition {
        self.position
    }

    #[inline]
    pub fn set_position(&mut self, position: TitlePosition) {
        self.position = position;
    }

    #[inline]
    pub fn with_position(mut self, position: TitlePosition) -> Self {
        self.set_position(position);
        self
    }

    #[inline]
    pub fn alignment(&self) -> TitleAlignment {
        self.alignment
    }

    #[inline]
    pub fn set_alignment(&mut self, alignment: TitleAlignment) {
        self.alignment = alignment;
    }

    #[inline]
    pub fn with_alignment(mut self, alignment: TitleAlignment) -> Self {
        self.set_alignment(alignment);
        self
    }

    #[inline]
    pub(crate) fn inner(&self) -> &W {
        &self.title
    }
}
