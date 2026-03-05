use crate::{
    cell::Cell,
    color::{Color, blend_source_over},
    core::buffer::flat::FlatBuffer,
};

#[inline]
pub fn compose_cell(bottom: &mut Cell, top: &Cell) {
    // Cyan is a convenience placeholder for no value here.
    // It should NEVER be visible in the terminal.
    //
    // If you ever see cyan where there shouldn't be cyan,
    // it means something clearly went wrong.
    let top_fg: Color = top.style.fg().unwrap_or(Color::CYAN);
    let top_bg: Color = top.style.bg().unwrap_or(Color::CYAN);

    let cover_fg_with_bg_case: bool =
        top.style.has_fg() && top.style.has_bg() && top_fg.a() == 0 && top_bg.a() == 255;
    let translucent_fg_keep_char_case: bool = top.style.has_fg() && top_fg.a() == 0;

    // --- ch, attributes ---
    if cover_fg_with_bg_case || !translucent_fg_keep_char_case {
        bottom.ch = top.ch;
        bottom.style.attributes = top.style.attributes;
    }

    // --- fg ---
    if top.style.has_fg() {
        let bottom_fg: Color = bottom.style.fg().unwrap_or(Color::TRANSPARENT);

        let override_fg_case: bool = top_fg.a() == 255;
        let blend_fg_with_fg_case: bool = (1..254).contains(&top_fg.a());

        if override_fg_case {
            bottom.style.set_fg(top_fg);
        } else if blend_fg_with_fg_case {
            bottom.style.set_fg(blend_source_over(bottom_fg, top_fg));
        } else if cover_fg_with_bg_case {
            // When fg is covered up, we set it's color
            // to transparent to prevent anomalies
            bottom.style.set_fg(Color::TRANSPARENT)
        }
    }

    // --- bg ---
    if top.style.has_bg() {
        let bottom_bg: Color = bottom.style.bg().unwrap_or(Color::TRANSPARENT);

        let override_bg: bool = top_bg.a() == 255;
        let blend_bg_with_bg: bool = (1..254).contains(&top_bg.a());

        if override_bg {
            bottom.style.set_bg(top_bg);
        } else if blend_bg_with_bg {
            bottom.style.set_bg(blend_source_over(bottom_bg, top_bg));
        }
    }
}

#[inline]
pub fn compose_buffers(bottom_buf: &mut FlatBuffer, top_buf: &FlatBuffer) {
    for (bottom, top) in bottom_buf.cells_mut().zip(top_buf.cells()) {
        compose_cell(bottom, top);
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
    todo!();
    // let mask_a = BLOCKTAD_CHAR_LUT
    //     .iter()
    //     .position(|&c| c == a)
    //     .expect("char not in BLOCKTAD LUT") as u8;
    // let mask_b = BLOCKTAD_CHAR_LUT
    //     .iter()
    //     .position(|&c| c == b)
    //     .expect("char not in BLOCKTAD LUT") as u8;

    // let merged_mask = mask_a | mask_b;

    // BLOCKTAD_CHAR_LUT[merged_mask as usize]
}

#[cfg(test)]
mod tests {
    use crate::{
        cell::{Cell, CellFormat},
        color::Color,
        core::{
            buffer::{Buffer, flat::FlatBuffer},
            draw::{Position, Size},
        },
        style::{Attributes, Style},
    };

    // Convenience helper for constructing a flat 1x1 buffer
    fn make_buf() -> FlatBuffer {
        FlatBuffer::new(Size::new(1, 1))
    }

    // Convenience helper for drawing a single standard cell at (0, 0)
    fn draw_standard_cell<Buf: Buffer>(
        buf: &mut Buf,
        ch: char,
        fg: impl Into<Option<Color>>,
        bg: impl Into<Option<Color>>,
    ) {
        let fg: Option<Color> = fg.into();
        let bg: Option<Color> = bg.into();

        buf.set_cell(
            Position::new(0, 0),
            Cell {
                ch,
                style: Style::default().with_fg(fg).with_bg(bg),
                format: CellFormat::Standard,
            },
        );
    }

    // Convenience helper for retrieving the cell at (0, 0)
    fn cell<Buf: Buffer>(buf: &Buf) -> &Cell {
        buf.get_cell(Position::new(0, 0))
    }

    #[test]
    fn cell_opaque_bg_over_opaque_bg() {
        let mut buf = make_buf();

        draw_standard_cell(&mut buf, ' ', Color::TRANSPARENT, Color::BLACK);
        draw_standard_cell(&mut buf, ' ', Color::TRANSPARENT, Color::WHITE);

        let expected = Cell {
            ch: ' ',
            style: Style::new(Color::TRANSPARENT, Color::WHITE, Attributes::empty()),
            format: CellFormat::Standard,
        };
        assert_eq!(*cell(&buf), expected);
    }

    #[test]
    fn cell_opaque_fg_over_opaque_fg() {
        let mut buf = make_buf();

        draw_standard_cell(&mut buf, '*', Color::BLACK, Color::TRANSPARENT);
        draw_standard_cell(&mut buf, '*', Color::WHITE, Color::TRANSPARENT);

        let expected = Cell {
            ch: '*',
            style: Style::new(Color::WHITE, Color::TRANSPARENT, Attributes::empty()),
            format: CellFormat::Standard,
        };
        assert_eq!(*cell(&buf), expected);
    }

    #[test]
    fn cell_transparent_bg_over_opaque_bg() {
        let mut buf = make_buf();

        draw_standard_cell(&mut buf, ' ', Color::TRANSPARENT, Color::RED);
        draw_standard_cell(&mut buf, ' ', Color::TRANSPARENT, Color::TRANSPARENT);

        let expected = Cell {
            ch: ' ',
            style: Style::new(Color::TRANSPARENT, Color::RED, Attributes::empty()),
            format: CellFormat::Standard,
        };
        assert_eq!(*cell(&buf), expected);
    }

    #[test]
    fn cell_transparent_fg_over_opaque_fg() {
        let mut buf = make_buf();

        draw_standard_cell(&mut buf, '*', Color::RED, Color::TRANSPARENT);
        draw_standard_cell(&mut buf, '*', Color::TRANSPARENT, Color::TRANSPARENT);

        let expected = Cell {
            ch: '*',
            style: Style::new(Color::RED, Color::TRANSPARENT, Attributes::empty()),
            format: CellFormat::Standard,
        };
        assert_eq!(*cell(&buf), expected);
    }

    #[test]
    fn cell_translucent_bg_over_opaque_bg() {
        let mut buf = make_buf();

        draw_standard_cell(&mut buf, ' ', Color::TRANSPARENT, Color::BLACK);
        draw_standard_cell(
            &mut buf,
            ' ',
            Color::TRANSPARENT,
            Color::RED.with_alpha(127),
        );

        let expected = Cell {
            ch: ' ',
            style: Style::new(
                Color::TRANSPARENT,
                Color::new(127, 0, 0, 255),
                Attributes::empty(),
            ),
            format: CellFormat::Standard,
        };
        assert_eq!(*cell(&buf), expected);
    }

    #[test]
    fn cell_translucent_fg_over_opaque_fg() {
        let mut buf = make_buf();

        draw_standard_cell(&mut buf, '*', Color::BLACK, Color::TRANSPARENT);
        draw_standard_cell(
            &mut buf,
            '*',
            Color::RED.with_alpha(127),
            Color::TRANSPARENT,
        );

        let expected = Cell {
            ch: '*',
            style: Style::new(
                Color::new(127, 0, 0, 255),
                Color::TRANSPARENT,
                Attributes::empty(),
            ),
            format: CellFormat::Standard,
        };
        assert_eq!(*cell(&buf), expected);
    }

    #[test]
    fn cell_translucent_bg_over_transparent_bg() {
        let mut buf = make_buf();

        draw_standard_cell(
            &mut buf,
            ' ',
            Color::TRANSPARENT,
            Color::RED.with_alpha(127),
        );

        let expected = Cell {
            ch: ' ',
            style: Style::new(
                Color::TRANSPARENT,
                Color::new(127, 0, 0, 127),
                Attributes::empty(),
            ),
            format: CellFormat::Standard,
        };
        assert_eq!(*cell(&buf), expected);
    }

    #[test]
    fn cell_translucent_fg_over_transparent_fg() {
        let mut buf = make_buf();

        draw_standard_cell(
            &mut buf,
            '*',
            Color::RED.with_alpha(127),
            Color::TRANSPARENT,
        );

        let expected = Cell {
            ch: '*',
            style: Style::new(
                Color::new(127, 0, 0, 127),
                Color::TRANSPARENT,
                Attributes::empty(),
            ),
            format: CellFormat::Standard,
        };
        assert_eq!(*cell(&buf), expected);
    }

    #[test]
    fn cell_transparent_fg_opaque_bg_over_opaque_fg() {
        let mut buf = make_buf();

        draw_standard_cell(&mut buf, '*', Color::RED, Color::TRANSPARENT);
        draw_standard_cell(&mut buf, ' ', Color::TRANSPARENT, Color::DARK_GRAY);

        let expected = Cell {
            ch: ' ',
            style: Style::new(Color::TRANSPARENT, Color::DARK_GRAY, Attributes::empty()),
            format: CellFormat::Standard,
        };
        assert_eq!(*cell(&buf), expected);
    }

    #[test]
    fn cell_transparent_fg_transparent_bg_over_opaque_fg() {
        let mut buf = make_buf();

        draw_standard_cell(&mut buf, '*', Color::RED, Color::TRANSPARENT);
        draw_standard_cell(&mut buf, ' ', Color::TRANSPARENT, Color::TRANSPARENT);

        let expected = Cell {
            ch: '*',
            style: Style::new(Color::RED, Color::TRANSPARENT, Attributes::empty()),
            format: CellFormat::Standard,
        };
        assert_eq!(*cell(&buf), expected);
    }
}
