use crate::{color::Color, rich_text::Attributes};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub attributes: Attributes,
}

impl Cell {
    pub const EMPTY: Cell = Cell {
        ch: ' ',
        fg: Color::NO_COLOR,
        bg: Color::NO_COLOR,
        attributes: Attributes::empty(),
    };
}
