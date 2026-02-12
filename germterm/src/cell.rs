use crate::{color::Color, rich_text::Attributes};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellFormat {
    Standard,
    Twoxel,
    Octad,
    Blocktad,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub attributes: Attributes,
    pub format: CellFormat,
}

impl Cell {
    pub const EMPTY: Cell = Cell {
        ch: ' ',
        fg: Color::CLEAR,
        bg: Color::CLEAR,
        attributes: Attributes::from_bits_truncate(
            Attributes::NO_FG_COLOR.bits() | Attributes::NO_BG_COLOR.bits(),
        ),
        format: CellFormat::Standard,
    };
}
