use sinstr::{SinStr, sinstr_literal};

use crate::style::Style;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellFormat {
    Standard,
    Twoxel,
    Octad,
    Blocktad,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cell {
    ch: SinStr,
    style: Style,
    format: CellFormat,
}

impl Cell {
    pub const EMPTY: Cell = Cell {
        ch: sinstr_literal!(" "),
        style: Style::EMPTY,
        format: CellFormat::Standard,
    };

    pub fn new(s: &str, style: Style) -> Self {
        Self {
            ch: SinStr::new(s),
            style,
            format: CellFormat::Standard,
        }
    }

    pub fn as_str(&self) -> &str {
        self.ch.as_str()
    }

    pub fn style(&self) -> Style {
        self.style
    }

    pub fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    pub fn set_str(&mut self, s: &str) {
        self.ch.set_str(s);
    }

    pub fn merge(&mut self, other: &Self) {
        // We can use `SinStr::clone_from` here which is faster than
        // `SinStr::set_str`
        self.ch.clone_from(&other.ch);
        self.style.merge(other.style);
    }
}
