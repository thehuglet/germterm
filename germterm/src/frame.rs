use crate::{
    cell::Cell,
    color::{Color, blend_source_over},
    draw::BLOCKTAD_CHAR_LUT,
    rich_text::{Attributes, RichText},
};
use crossterm::{cursor as ctcursor, queue, style as ctstyle};
use std::{
    io::{self, Stdout, Write},
    ops::{Index, IndexMut},
    str::Chars,
};

#[derive(Clone)]
pub struct DrawCall {
    pub rich_text: RichText,
    pub x: i16,
    pub y: i16,
}

pub struct DiffProduct<'a> {
    pub cell: &'a Cell,
    pub x: u16,
    pub y: u16,
}

pub struct Frame<'a>(&'a [Cell], usize);
pub struct FrameMut<'a>(&'a mut [Cell], usize);
impl<'a> Index<usize> for Frame<'a> {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index * 2 + self.1]
    }
}

impl<'a> Index<usize> for FrameMut<'a> {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index * 2 + self.1]
    }
}

impl<'a> IndexMut<usize> for FrameMut<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index * 2 + self.1]
    }
}

#[derive(Clone, Copy, Debug)]
enum FrameOrder {
    CurrentOld = 0,
    OldCurrent = 1,
}

pub struct FramePair {
    /// This stores double of the cell count.
    ///
    /// Each cell is followed by its new or old version depending on the value of [`FrameOrder`]
    pub(crate) frames: Vec<Cell>,
    order: FrameOrder,
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) layered_draw_queue: Vec<Vec<DrawCall>>,
}

impl FramePair {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            order: FrameOrder::OldCurrent,
            frames: vec![Cell::EMPTY; (width as usize * height as usize) * 2],
            width,
            height,
            layered_draw_queue: vec![],
        }
    }

    pub fn diff(&self) -> impl Iterator<Item = DiffProduct<'_>> {
        debug_assert!(self.frames.len().is_multiple_of(2));
        let width = self.width;
        let order = self.order as usize;

        unsafe { self.frames.as_chunks_unchecked::<2>() }
            .iter()
            .enumerate()
            .filter_map(move |(i, cells @ [left, right])| {
                if left != right {
                    let x = (i % width as usize) as u16;
                    let y = (i / width as usize) as u16;
                    Some(DiffProduct {
                        cell: unsafe { cells.get_unchecked(order) },
                        x,
                        y,
                    })
                } else {
                    None
                }
            })
    }

    pub fn current(&self) -> Frame<'_> {
        Frame(self.frames.as_slice(), self.order as usize)
    }

    pub fn current_mut(&mut self) -> FrameMut<'_> {
        FrameMut(self.frames.as_mut_slice(), self.order as usize)
    }

    /// Swap the current and old frames
    pub fn swap_frames(&mut self) {
        self.order = match self.order {
            FrameOrder::CurrentOld => FrameOrder::OldCurrent,
            FrameOrder::OldCurrent => FrameOrder::CurrentOld,
        };
    }

    pub fn current_mut_and_layered_mut(&mut self) -> (FrameMut<'_>, &mut Vec<Vec<DrawCall>>) {
        let frame = FrameMut(&mut self.frames, self.order as usize);
        let layers = &mut self.layered_draw_queue;
        (frame, layers)
    }
}

pub(crate) fn compose_frame_buffer(
    mut buffer: FrameMut<'_>,
    draw_queue: impl Iterator<Item = DrawCall>,
    cols: u16,
    rows: u16,
    default_blending_color: Color,
) {
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
            let old_cell: Cell = buffer[cell_index];
            let new_cell: Cell = Cell {
                ch,
                fg: draw_call.rich_text.fg,
                bg: draw_call.rich_text.bg,
                attributes: draw_call.rich_text.attributes,
            };

            buffer[cell_index] = compose_cell(old_cell, new_cell, default_blending_color);
        }
    }
}

pub(crate) fn build_crossterm_content_style(cell: &Cell) -> crossterm::style::ContentStyle {
    use crossterm::style as ctstyle;

    let fg_color: Option<ctstyle::Color> = if cell.fg == Color::NO_COLOR {
        None
    } else {
        Some(ctstyle::Color::Rgb {
            r: cell.fg.r(),
            g: cell.fg.g(),
            b: cell.fg.b(),
        })
    };

    let bg_color: Option<ctstyle::Color> = if cell.bg == Color::NO_COLOR {
        None
    } else {
        Some(ctstyle::Color::Rgb {
            r: cell.bg.r(),
            g: cell.bg.g(),
            b: cell.bg.b(),
        })
    };

    let attributes = [
        (Attributes::BOLD, ctstyle::Attribute::Bold),
        (Attributes::ITALIC, ctstyle::Attribute::Italic),
        (Attributes::UNDERLINED, ctstyle::Attribute::Underlined),
        (Attributes::HIDDEN, ctstyle::Attribute::Hidden),
    ]
    .iter()
    .fold(
        ctstyle::Attributes::none(),
        |ct_attrs, (attribute, ct_attr)| {
            if cell.attributes.contains(*attribute) {
                ct_attrs | *ct_attr
            } else {
                ct_attrs
            }
        },
    );

    ctstyle::ContentStyle {
        foreground_color: fg_color,
        background_color: bg_color,
        underline_color: None,
        attributes,
    }
}

pub(crate) fn draw_to_terminal<'a>(
    stdout: &mut Stdout,
    diff_products: impl Iterator<Item = DiffProduct<'a>>,
) -> io::Result<()> {
    for diff_product in diff_products {
        let x: u16 = diff_product.x;
        let y: u16 = diff_product.y;
        let cell: &Cell = diff_product.cell;

        let style: ctstyle::ContentStyle = build_crossterm_content_style(cell);
        queue!(
            stdout,
            ctcursor::MoveTo(x, y),
            ctstyle::SetAttribute(ctstyle::Attribute::Reset),
            ctstyle::SetStyle(style),
            ctstyle::Print(cell.ch),
        )?;
    }

    stdout.flush()?;
    Ok(())
}

#[inline]
fn compose_cell(old: Cell, new: Cell, default_blending_color: Color) -> Cell {
    let new_twoxel: bool = new.attributes.contains(Attributes::TWOXEL);
    let new_octad: bool = new.attributes.contains(Attributes::OCTAD);
    let new_blocktad: bool = new.attributes.contains(Attributes::BLOCKTAD);
    let old_twoxel: bool = old.attributes.contains(Attributes::TWOXEL);
    let old_octad: bool = old.attributes.contains(Attributes::OCTAD);
    let old_blocktad: bool = old.attributes.contains(Attributes::BLOCKTAD);
    let both_ch_equal: bool = old.ch == new.ch;

    // Foreground related
    let new_fg_no_color: bool = new.fg == Color::NO_COLOR;
    let new_fg_invisible: bool = new.fg.a() == 0;
    let new_fg_opaque: bool = new.fg.a() == 255;
    let new_ch_invisible: bool = new.ch == ' ' || new_fg_invisible;

    let old_fg_invisible: bool = old.fg.a() == 0;
    let old_ch_invisible: bool = old.ch == ' ' || old_fg_invisible;

    // Background related
    let new_bg_no_color: bool = new.bg == Color::NO_COLOR;
    let new_bg_invisible: bool = new.bg.a() == 0;
    let new_bg_opaque: bool = new.bg.a() == 255;
    let new_bg_translucent: bool = !new_bg_opaque && !new_bg_invisible;

    let old_bg_no_color: bool = old.bg == Color::NO_COLOR;

    if new_twoxel {
        let (ch, attributes) = if old_twoxel && !new_fg_no_color {
            // Covers case:
            // - Drawing a twoxel on top of another twoxel
            //      => Keep the old char
            (old.ch, old.attributes)
        } else {
            (new.ch, new.attributes)
        };

        let fg = if old_twoxel && both_ch_equal {
            // Covers case:
            // - Drawing a twoxel on top of another twoxel (same half-block)
            //      => Blend the old fg with the new fg
            blend_source_over(old.fg, new.fg)
        } else if old_twoxel {
            // Covers case:
            // - Drawing a twoxel on top of another twoxel (different half-block)
            //      => Keep the old fg
            old.fg
        } else if !old_bg_no_color {
            blend_source_over(old.bg, new.fg)
        } else {
            blend_source_over(default_blending_color, new.fg)
        };

        let bg = if old_twoxel && both_ch_equal {
            // Covers case:
            // - Drawing a twoxel on top of another twoxel (same half)
            //      => Keep the old bg
            old.bg
        } else if old_twoxel && old_bg_no_color {
            blend_source_over(default_blending_color, new.fg)
        } else if old_twoxel {
            // Covers case:
            // - Drawing a twoxel on top of another twoxel (different half-block)
            //      => Draw the twoxel's fg as the bg channel
            blend_source_over(old.bg, new.fg)
        } else {
            old.bg
        };

        Cell {
            ch,
            fg,
            bg,
            attributes,
        }
    } else {
        // This branch handles the following drawing formats: [standard, octad, blocktad]

        let (ch, attributes) = if new_ch_invisible && !new_bg_opaque && !new_bg_no_color {
            // Covers case:
            // - Fading an invisible character should not replace the one underneath
            //      => Keep the old character
            (old.ch, old.attributes)
        } else if new_blocktad && old_blocktad {
            // Covers case:
            // - Drawing a blocktad on top of another blocktad
            //      => Merge the blocktad chars
            (merge_blocktad(old.ch, new.ch), new.attributes)
        } else if new_octad && old_octad {
            // Covers case:
            // - Drawing an octad on top of another octad
            //      => Merge the octad braille chars
            (merge_octad(old.ch, new.ch), new.attributes)
        } else {
            (new.ch, new.attributes)
        };

        let fg = if new_bg_translucent && new_fg_invisible {
            // Covers case:
            // - Drawing a translucent bg with no visible char over a visible char
            //      => Tint the old fg with the new bg to make it look like it's underneath it
            blend_source_over(old.fg, new.bg)
        } else if !old_ch_invisible && new_ch_invisible {
            // Covers case:
            // - Drawing an invisible char on top of another char
            //      => Preserve old fg as the invisible char shouldn't be covering it
            old.fg
        } else if !old_ch_invisible && !new_fg_opaque {
            // Covers case:
            // - Drawing a non-opaque char on top of another visible char
            //      => Blend the old fg with the new fg for a smoother transition
            blend_source_over(old.fg, new.fg)
        } else if !old_bg_no_color && !new_bg_invisible {
            // Covers case:
            // - Drawing fg text with a translucent bg above a regular bg
            //      => Blend the translucent new bg with the old bg, then blend the new fg with the result
            blend_source_over(blend_source_over(old.bg, new.bg), new.fg)
        } else if old_bg_no_color && !new_bg_invisible {
            // Covers case:
            // - Drawing fg text with a translucent bg above a Color::NO_COLOR bg
            //      => Blend the translucent new bg with the default blending color, then blend the new fg with the result
            blend_source_over(blend_source_over(default_blending_color, new.bg), new.fg)
        } else if old_bg_no_color {
            // Covers case:
            // - Drawing a translucent fg char over a Color::NO_COLOR bg
            //      => Blend the new fg with the default blending color
            blend_source_over(default_blending_color, new.fg)
        } else {
            blend_source_over(old.bg, new.fg)
        };

        let bg = if new_bg_no_color {
            // Covers case:
            // - Drawing a Color::NO_COLOR bg
            //      => Erase the bg
            Color::NO_COLOR
        } else if old_bg_no_color && new_bg_invisible {
            // Covers case:
            // - Drawing a bg with an alpha of 0 over Color::NO_COLOR
            //      => Erase the bg
            Color::NO_COLOR
        } else if old_bg_no_color && !new_bg_opaque {
            // Covers cases:
            // - Drawing a translucent background over a Color::NO_COLOR bg
            //      => The new bg will be blended with the default blending color
            blend_source_over(default_blending_color, new.bg)
        } else {
            blend_source_over(old.bg, new.bg)
        };

        Cell {
            ch,
            fg,
            bg,
            attributes,
        }
    }
}

#[inline]
fn merge_octad(a: char, b: char) -> char {
    let mask_a = (a as u32) - 0x2800;
    let mask_b = (b as u32) - 0x2800;
    std::char::from_u32(0x2800 + (mask_a | mask_b)).unwrap()
}

#[inline]
fn merge_blocktad(a: char, b: char) -> char {
    let mask_a = BLOCKTAD_CHAR_LUT
        .iter()
        .position(|&c| c == a)
        .expect("char not in BLOCKTAD LUT") as u8;
    let mask_b = BLOCKTAD_CHAR_LUT
        .iter()
        .position(|&c| c == b)
        .expect("char not in BLOCKTAD LUT") as u8;

    let merged_mask = mask_a | mask_b;

    BLOCKTAD_CHAR_LUT[merged_mask as usize]
}
