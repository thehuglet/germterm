use crate::engine::{attributes::Attributes, color::PackedRgba};

#[derive(Clone)]
pub struct TerminalCell {
    pub ch: char,
    pub fg: PackedRgba,
    pub bg: PackedRgba,
    pub attributes: Attributes,
}

pub struct ScreenBuffer(pub Vec<TerminalCell>);

pub struct Screen {
    pub cols: u16,
    pub rows: u16,
    pub current_buffer: ScreenBuffer,
    pub last_buffer: ScreenBuffer,
}

impl Screen {
    pub fn new(cols: u16, rows: u16) -> Self {
        let vec_capacity: u16 = cols * rows;

        Screen {
            cols,
            rows,
            current_buffer: ScreenBuffer(vec![
                TerminalCell {
                    ch: ' ',
                    fg: PackedRgba::WHITE,
                    bg: PackedRgba::BLACK,
                    attributes: Attributes::empty(),
                };
                vec_capacity as usize
            ]),
            last_buffer: ScreenBuffer(vec![
                TerminalCell {
                    ch: ' ',
                    // Intentional `fg` difference to force the first frame redraw
                    fg: PackedRgba::BLACK,
                    bg: PackedRgba::BLACK,
                    attributes: Attributes::empty(),
                };
                vec_capacity as usize
            ]),
        }
    }
}
