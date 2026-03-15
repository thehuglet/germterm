use crate::{
    cell::Cell,
    color::{Color, blend_source_over},
    core::buffer::flat::FlatBuffer,
};

#[inline]
pub fn compose_cell(bottom: &mut Cell, top: &Cell, bg_fallback: Color) {
    let top_has_fg: bool = top.style.has_fg();
    let top_has_bg: bool = top.style.has_bg();

    // Cyan is a convenience placeholder for no value here.
    // It should NEVER be visible in the terminal.
    //
    // If you ever see cyan where there shouldn't be cyan,
    // it means something clearly went wrong.
    let top_fg: Color = top.style.fg().unwrap_or(Color::CYAN);
    let top_bg: Color = top.style.bg().unwrap_or(Color::CYAN);

    let cover_fg_with_bg: bool =
        top.style.has_fg() && top.style.has_bg() && top_fg.a() == 0 && top_bg.a() == 255;
    let translucent_fg_keep_char: bool = top.style.has_fg() && top_fg.a() == 0;
    // let keep_old_on_space: bool = top.ch == ' ';
    let keep_old_ch_and_fg: bool = top.ch == ' ' || top_fg.a() == 0;

    // --- ch, attributes ---
    if cover_fg_with_bg {
        bottom.ch = ' ';
        bottom.style.set_attributes(top.style.attributes());
    } else if !(translucent_fg_keep_char || keep_old_ch_and_fg) {
        bottom.ch = top.ch;
        bottom.style.set_attributes(top.style.attributes());
    }

    // --- fg ---
    if top_has_fg {
        let translucent_on_clear: bool = top_fg.a() == 0 && !bottom.style.has_fg();
        let override_fg: bool = top_fg.a() == 255;
        let blend_fg_with_fg: bool = (1..254).contains(&top_fg.a());

        if let Some(bottom_fg) = bottom.style.fg() {
            if keep_old_ch_and_fg {
                // Do nothing
            } else if override_fg {
                bottom.style.set_fg(top_fg);
            } else if blend_fg_with_fg {
                bottom.style.set_fg(blend_source_over(bottom_fg, top_fg));
            } else if cover_fg_with_bg {
                // When fg is covered up, we set it's color
                // to none to prevent anomalies
                bottom.style.set_fg(None);
            }
        }
    } else {
        bottom.style.set_fg(None);
    }

    // --- bg ---
    if top_has_bg {
        let translucent_on_clear: bool = top_bg.a() == 0 && !bottom.style.has_bg();
        let override_bg: bool = top_bg.a() == 255;
        let blend_bg_with_bg: bool = (1..254).contains(&top_bg.a());

        if let Some(bottom_bg) = bottom.style.bg() {
            if override_bg {
                bottom.style.set_bg(top_bg);
            } else if blend_bg_with_bg {
                bottom.style.set_bg(blend_source_over(bottom_bg, top_bg));
            }
        }
    } else {
        bottom.style.set_bg(None);
    }
}

// #[inline]
// pub fn compose_buffer(bottom_buf: &mut FlatBuffer, top_buf: &FlatBuffer) {
//     for (bottom, top) in bottom_buf.cells_mut().zip(top_buf.cells()) {
//         compose_cell(bottom, top);
//     }
// }

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
    use std::mem::MaybeUninit;

    use crate::{
        cell::{Cell, CellFormat},
        color::Color,
        core::{
            DrawCall,
            buffer::{
                Buffer, Drawer, blended::BlendedBuffer, flat::FlatBuffer, layered::LayeredBuffer,
            },
            draw::{Position, Size},
        },
        style::{Attributes, Style},
    };

    const POS: Position = Position::new(0, 0);
    const SIZE: Size = Size::new(1, 1);

    // Convenience helper for constructing a flat blended 1x1 buffer
    fn flat_blended_buf_opaque_fill() -> BlendedBuffer<FlatBuffer> {
        BlendedBuffer::new(FlatBuffer::new(SIZE))
    }

    // Convenience helper for constructing a layered blended 1x1 buffer
    fn layered_blended_buf()
    -> LayeredBuffer<BlendedBuffer<FlatBuffer>, impl Fn(Size) -> BlendedBuffer<FlatBuffer>> {
        LayeredBuffer::new(SIZE, |size| {
            let black_bg_cell = Cell {
                ch: ' ',
                style: Style::CLEAR.with_bg(Color::BLACK),
                format: CellFormat::Standard,
            };
            BlendedBuffer::new(FlatBuffer::new_with_cell(size, black_bg_cell))
        })
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
            POS,
            Cell {
                ch,
                style: Style::default().with_fg(fg).with_bg(bg),
                format: CellFormat::Standard,
            },
        );
    }

    // Convenience helper for retrieving the cell at (0, 0)
    fn buf_cell<Buf: Buffer>(buf: &Buf) -> &Cell {
        buf.get_cell(POS)
    }

    fn layered_buf_cell<Buf: Buffer + Drawer>(buf: &mut Buf) -> &Cell {
        let draw_calls: Vec<DrawCall> = buf.draw().collect();
        draw_calls[0].cell
    }

    #[test]
    fn cell_clear_bg_over_opaque_bg() {
        let mut buf = flat_blended_buf_opaque_fill();

        draw_standard_cell(&mut buf, 'a', Color::WHITE, Color::BLACK);
        draw_standard_cell(&mut buf, 'a', Color::WHITE, None);

        let expected = Cell {
            ch: 'a',
            style: Style {
                fg: MaybeUninit::new(Color::WHITE),
                bg: MaybeUninit::uninit(),
                attributes: Attributes::CLEAR_BG,
            },
            format: CellFormat::Standard,
        };
        assert_eq!(*buf_cell(&buf), expected);
    }

    #[test]
    fn cell_opaque_bg_over_opaque_bg() {
        let mut buf = flat_blended_buf_opaque_fill();

        // draw_standard_cell(&mut buf, 'a', Color::WHITE, Color::RED);

        let expected = Cell {
            ch: 'a',
            style: Style {
                fg: MaybeUninit::new(Color::WHITE),
                bg: MaybeUninit::new(Color::RED),
                attributes: Attributes::empty(),
            },
            format: CellFormat::Standard,
        };
        assert_eq!(*buf_cell(&buf), expected);
    }

    #[test]
    fn cell_opaque_fg_over_opaque_fg() {
        let mut buf = flat_blended_buf_opaque_fill();

        draw_standard_cell(&mut buf, '*', Color::BLACK, Color::TRANSPARENT);
        draw_standard_cell(&mut buf, '*', Color::WHITE, Color::TRANSPARENT);

        let expected = Cell {
            ch: '*',
            style: Style::new(Color::WHITE, Color::TRANSPARENT, Attributes::empty()),
            format: CellFormat::Standard,
        };
        assert_eq!(*buf_cell(&buf), expected);
    }

    #[test]
    fn cell_transparent_bg_over_opaque_bg() {
        let mut buf = flat_blended_buf_opaque_fill();

        draw_standard_cell(&mut buf, ' ', Color::TRANSPARENT, Color::RED);
        draw_standard_cell(&mut buf, ' ', Color::TRANSPARENT, Color::TRANSPARENT);

        let expected = Cell {
            ch: ' ',
            style: Style::new(Color::TRANSPARENT, Color::RED, Attributes::empty()),
            format: CellFormat::Standard,
        };
        assert_eq!(*buf_cell(&buf), expected);
    }

    #[test]
    fn cell_transparent_fg_over_opaque_fg() {
        let mut buf = flat_blended_buf_opaque_fill();

        draw_standard_cell(&mut buf, '*', Color::RED, Color::TRANSPARENT);
        draw_standard_cell(&mut buf, '*', Color::TRANSPARENT, Color::TRANSPARENT);

        let expected = Cell {
            ch: '*',
            style: Style::new(Color::RED, Color::TRANSPARENT, Attributes::empty()),
            format: CellFormat::Standard,
        };
        assert_eq!(*buf_cell(&buf), expected);
    }

    #[test]
    fn cell_translucent_bg_over_opaque_bg() {
        let mut buf = flat_blended_buf_opaque_fill();

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
        assert_eq!(*buf_cell(&buf), expected);
    }

    #[test]
    fn cell_translucent_fg_over_opaque_fg() {
        let mut buf = flat_blended_buf_opaque_fill();

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
        assert_eq!(*buf_cell(&buf), expected);
    }

    #[test]
    fn cell_translucent_bg_over_transparent_bg() {
        let mut buf = flat_blended_buf_opaque_fill();

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
        assert_eq!(*buf_cell(&buf), expected);
    }

    #[test]
    fn cell_translucent_fg_over_transparent_fg() {
        let mut buf = flat_blended_buf_opaque_fill();

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
        assert_eq!(*buf_cell(&buf), expected);
    }

    #[test]
    fn cell_translucent_fg_over_opaque_fg_char_interpolation() {
        let mut buf = flat_blended_buf_opaque_fill();

        draw_standard_cell(&mut buf, 'a', Color::RED, Color::TRANSPARENT);
        draw_standard_cell(
            &mut buf,
            'b',
            Color::BLUE.with_alpha(127),
            Color::TRANSPARENT,
        );

        let expected = Cell {
            ch: 'b',
            style: Style::new(
                Color::new(128, 0, 127, 255),
                Color::TRANSPARENT,
                Attributes::empty(),
            ),
            format: CellFormat::Standard,
        };
        assert_eq!(*buf_cell(&buf), expected);
    }

    #[test]
    fn cell_cover_fg_with_opaque_bg() {
        let mut buf = flat_blended_buf_opaque_fill();

        draw_standard_cell(&mut buf, '*', Color::RED, Color::TRANSPARENT);
        draw_standard_cell(&mut buf, 'a', Color::TRANSPARENT, Color::DARK_GRAY);

        let expected = Cell {
            ch: ' ',
            style: Style::new(Color::RED, Color::DARK_GRAY, Attributes::empty()),
            format: CellFormat::Standard,
        };
        assert_eq!(*buf_cell(&buf), expected);
    }

    #[test]
    fn cell_transparent_fg_transparent_bg_over_opaque_fg() {
        let mut buf = flat_blended_buf_opaque_fill();

        draw_standard_cell(&mut buf, '*', Color::RED, Color::TRANSPARENT);
        draw_standard_cell(&mut buf, ' ', Color::TRANSPARENT, Color::TRANSPARENT);

        let expected = Cell {
            ch: '*',
            style: Style::new(Color::RED, Color::TRANSPARENT, Attributes::empty()),
            format: CellFormat::Standard,
        };
        assert_eq!(*buf_cell(&buf), expected);
    }

    #[test]
    fn cell_transparent_bg_over_clear_bg() {
        let mut buf = flat_blended_buf_opaque_fill();

        buf.set_cell(
            POS,
            Cell {
                ch: ' ',
                style: Style::new(Color::TRANSPARENT, None, Attributes::CLEAR_BG),
                format: CellFormat::Standard,
            },
        );
        draw_standard_cell(&mut buf, ' ', Color::TRANSPARENT, Color::TRANSPARENT);

        let expected = Cell {
            ch: ' ',
            style: Style::new(Color::TRANSPARENT, None, Attributes::CLEAR_BG),
            format: CellFormat::Standard,
        };
        assert_eq!(*buf_cell(&buf), expected);
    }

    #[test]
    fn same_layer_transparent_bg_over_opaque_bg() {
        let mut buf = layered_blended_buf();

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
        assert_eq!(*buf_cell(&buf), expected);
    }

    #[test]
    fn same_layer_transparent_bg_over_clear_bg() {
        let mut buf = layered_blended_buf();

        buf.set_cell(
            POS,
            Cell {
                ch: ' ',
                style: Style::new(Color::TRANSPARENT, None, Attributes::CLEAR_BG),
                format: CellFormat::Standard,
            },
        );
        draw_standard_cell(&mut buf, ' ', Color::TRANSPARENT, Color::TRANSPARENT);

        let expected = Cell {
            ch: ' ',
            style: Style::new(Color::TRANSPARENT, None, Attributes::CLEAR_BG),
            format: CellFormat::Standard,
        };
        assert_eq!(*buf_cell(&buf), expected);
    }

    #[test]
    fn cross_layer_transparent_bg_over_opaque_bg() {
        let mut buf = layered_blended_buf();

        draw_standard_cell(&mut buf, ' ', Color::TRANSPARENT, Color::BLACK);
        buf.select_layer(1);
        draw_standard_cell(
            &mut buf,
            ' ',
            Color::TRANSPARENT,
            Color::RED.with_alpha(127),
        );

        let expected = Cell {
            ch: ' ',
            style: Style::CLEAR_FG.with_bg(Color::new(127, 0, 0, 255)),
            format: CellFormat::Standard,
        };
        assert_eq!(*layered_buf_cell(&mut buf), expected);
    }
}
