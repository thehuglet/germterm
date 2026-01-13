use std::{ops::Deref, str::Chars};

use crossterm::style::{
    Attribute as CrosstermAttribute, Attributes as CrosstermAttributes,
    ContentStyle as CrosstermContentStyle,
};

use crate::engine::{
    color::{Color, blend_over},
    draw::DrawCall,
    rich_text::Attributes,
};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct TerminalCell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub attributes: Attributes,
}

pub struct DiffProduct {
    pub x: u16,
    pub y: u16,
    pub cell: TerminalCell,
}

#[derive(Clone)]
pub struct ScreenBuffer(pub Vec<TerminalCell>);

impl Deref for ScreenBuffer {
    type Target = [TerminalCell];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Screen {
    pub cols: u16,
    pub rows: u16,
    pub current_buffer: ScreenBuffer,
    pub old_buffer: ScreenBuffer,
    pub diff_products: Vec<DiffProduct>,
}

impl Screen {
    pub fn new(cols: u16, rows: u16) -> Self {
        let vec_capacity: usize = (cols * rows) as usize;
        let empty_buffer: ScreenBuffer = ScreenBuffer(vec![
            TerminalCell {
                ch: ' ',
                fg: Color::WHITE,
                bg: Color::BLACK,
                attributes: Attributes::empty(),
            };
            vec_capacity
        ]);

        Screen {
            cols,
            rows,
            current_buffer: empty_buffer.clone(),
            old_buffer: empty_buffer.clone(),
            diff_products: Vec::with_capacity(vec_capacity),
        }
    }
}

pub fn compose_buffer(buffer: &mut ScreenBuffer, draw_queue: &[DrawCall], cols: u16, rows: u16) {
    let (cols, rows) = (cols as i16, rows as i16);

    for draw_call in draw_queue {
        let mut x: i16 = draw_call.x;
        let y: i16 = draw_call.y;

        // --- Skipping out of bounds draw calls ---
        let is_oob_left: bool = x < 0;
        let is_oob_top: bool = y < 0;
        let is_oob_right: bool = x >= cols;
        let is_oob_bottom: bool = y >= rows;

        if is_oob_right || is_oob_top || is_oob_bottom {
            continue;
        }

        let mut chars: Chars<'_> = draw_call.rich_text.text.chars();

        // --- Cropping the out of bounds left side chars ---
        if is_oob_left {
            for _ in 0..(-x) {
                chars.next();
            }
            x = 0;
        }

        let row_start_index: usize = (y as usize) * (cols as usize);
        let remaining_cols: usize = (cols - x).max(0) as usize;

        for (x_offset, ch) in chars.take(remaining_cols).enumerate() {
            let cell_index: usize = row_start_index + x as usize + x_offset;
            let old_cell: TerminalCell = buffer.0[cell_index];
            let new_cell: TerminalCell = TerminalCell {
                ch,
                fg: draw_call.rich_text.fg,
                bg: draw_call.rich_text.bg,
                attributes: draw_call.rich_text.attributes,
            };

            buffer.0[cell_index] = compose_cell(old_cell, new_cell);
        }
    }
}

#[inline]
fn compose_cell(old: TerminalCell, new: TerminalCell) -> TerminalCell {
    let old_ch_invisible: bool = old.ch == ' ' || old.fg.a() == 0;
    let new_ch_invisible: bool = new.ch == ' ' || new.fg.a() == 0;

    let out_bg: Color = if new.bg.a() == 0 {
        // new.bg invisible => Keep old.bg
        old.bg
    } else if new.bg.a() == 255 {
        // Opaque new.bg can't be blended with old.bg => Draw new.bg
        new.bg
    } else {
        // Default
        blend_over(old.bg, new.bg)
    };

    let (out_ch, out_attributes) = if new_ch_invisible {
        // Invisible new.ch => Keep old.ch
        (old.ch, old.attributes)
    } else {
        // Default
        (new.ch, new.attributes)
    };

    let out_fg: Color = if new_ch_invisible {
        // Can't blend old.fg with new.fg => Blend old.fg with new.bg instead
        blend_over(old.fg, new.bg)
    } else if old_ch_invisible {
        // Can't blend old.fg with new.fg => Blend old.bg with new.fg instead
        blend_over(old.bg, new.fg)
    } else {
        // default
        blend_over(old.fg, new.fg)
    };

    TerminalCell {
        ch: out_ch,
        fg: out_fg,
        bg: out_bg,
        attributes: out_attributes,
    }
}

pub fn diff_buffers(
    diff_products: &mut Vec<DiffProduct>,
    current_buffer: &ScreenBuffer,
    old_buffer: &ScreenBuffer,
    cols: u16,
) {
    let cols: usize = cols as usize;

    diff_products.clear();

    let row_pairs = old_buffer.chunks(cols).zip(current_buffer.chunks(cols));

    for (y, (old_row, new_row)) in row_pairs.enumerate() {
        let y: u16 = y as u16;
        let cell_pairs = old_row.iter().zip(new_row.iter());

        for (x, (old_cell, new_cell)) in cell_pairs.enumerate() {
            let x: u16 = x as u16;

            if old_cell != new_cell {
                diff_products.push(DiffProduct {
                    x,
                    y,
                    cell: *new_cell,
                });
            }
        }
    }
}

pub fn build_crossterm_content_style(cell: &TerminalCell) -> crossterm::style::ContentStyle {
    let fg_color: crossterm::style::Color = crossterm::style::Color::Rgb {
        r: cell.fg.r(),
        g: cell.fg.g(),
        b: cell.fg.b(),
    };

    let bg_color: crossterm::style::Color = crossterm::style::Color::Rgb {
        r: cell.bg.r(),
        g: cell.bg.g(),
        b: cell.bg.b(),
    };

    let attributes = [
        (Attributes::BOLD, CrosstermAttribute::Bold),
        (Attributes::ITALIC, CrosstermAttribute::Italic),
        (Attributes::UNDERLINED, CrosstermAttribute::Underlined),
        (Attributes::HIDDEN, CrosstermAttribute::Hidden),
    ]
    .iter()
    .fold(
        CrosstermAttributes::none(),
        |crossterm_attrs, (attribute, crossterm_attribute)| {
            if cell.attributes.contains(*attribute) {
                crossterm_attrs | *crossterm_attribute
            } else {
                crossterm_attrs
            }
        },
    );

    CrosstermContentStyle {
        foreground_color: Some(fg_color),
        background_color: Some(bg_color),
        underline_color: None,
        attributes,
    }
}
