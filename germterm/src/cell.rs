use crate::style::Style;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellFormat {
    Standard,
    Twoxel,
    Octad,
    Blocktad,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub style: Style,
    pub format: CellFormat,
}

impl Cell {
    pub const EMPTY: Cell = Cell {
        ch: ' ',
        style: Style::EMPTY,
        format: CellFormat::Standard,
    };

    pub const CLEAR: Cell = Cell {
        ch: ' ',
        style: Style::CLEAR,
        format: CellFormat::Standard,
    };

    pub const CLEAR_FG: Cell = Cell {
        ch: ' ',
        style: Style::CLEAR_FG,
        format: CellFormat::Standard,
    };

    pub const CLEAR_BG: Cell = Cell {
        ch: ' ',
        style: Style::CLEAR_BG,
        format: CellFormat::Standard,
    };

    pub fn merge(&mut self, other: Self) {
        self.ch = other.ch;
        self.style.merge(other.style);
    }
}
