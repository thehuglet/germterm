use unicode_segmentation::UnicodeSegmentation;

use crate::core::{buffer::Buffer, draw::Position, widget::FrameContext};

/// Draw the provided string starting at the specified position.
///
/// The provided string is written as much as possible in a single line of the buffer.
/// If the size exceeded the string is truncated. If the string is too short the remaining cells
/// will stay untouched.
///
/// # Returns
///
/// Returns the number of cells now occupied that were written.
pub fn draw_string<Buf: Buffer, D>(
    fc: FrameContext<'_, Buf, D>,
    mut pos: Position,
    s: &str,
) -> u16 {
    let sz = fc.buffer.size();
    if sz.area() == 0 || !sz.is_within(pos) {
        return 0;
    }

    let orig = pos.x;
    for g in s.graphemes(true) {
        let grapheme_width = (fc.display_width.str_width)(g);
        let Some(added) = pos
            .x
            .checked_add(grapheme_width)
            .filter(|added| sz.width >= *added)
        else {
            break;
        };
        fc.buffer.get_cell_mut(pos).set_str(g);
        pos.x = added;
    }

    pos.x - orig
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        buf_assert_eq, buffer,
        core::{DisplayWidth, buffer::flat::FlatBuffer, draw::Size, timer::NoDelta},
    };

    fn make_buf(sz: Size) -> FlatBuffer {
        FlatBuffer::new(sz)
    }

    fn make_fc(buf: &mut FlatBuffer) -> FrameContext<'_, FlatBuffer, NoDelta> {
        FrameContext::new(NoDelta::new(), NoDelta::new(), buf, DisplayWidth::default())
    }

    #[test]
    fn zero_area_buffer_returns_zero() {
        let mut buf = make_buf(Size::ZERO);
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "hello");
        assert_eq!(written, 0);
    }

    #[test]
    fn zero_width_buffer_returns_zero() {
        let mut buf = make_buf(Size::new(0, 5));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "hello");
        assert_eq!(written, 0);
    }

    #[test]
    fn zero_height_buffer_returns_zero() {
        let mut buf = make_buf(Size::new(5, 0));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "hello");
        assert_eq!(written, 0);
    }

    #[test]
    fn start_x_out_of_bounds_returns_zero() {
        let mut buf = make_buf(Size::new(5, 3));
        let written = draw_string(make_fc(&mut buf), Position::new(5, 0), "hello");
        assert_eq!(written, 0);

        buf_assert_eq!(buf, buffer![[empty(5)], empty(2)]);
    }

    #[test]
    fn start_y_out_of_bounds_returns_zero() {
        let mut buf = make_buf(Size::new(5, 3));
        let written = draw_string(make_fc(&mut buf), Position::new(0, 3), "hello");
        assert_eq!(written, 0);

        buf_assert_eq!(buf, buffer![empty(2), [empty(5)]]);
    }

    #[test]
    fn empty_string_returns_zero() {
        let mut buf = make_buf(Size::new(5, 3));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "");
        assert_eq!(written, 0);

        buf_assert_eq!(buf, buffer![[empty(5)], empty(2)]);
    }

    #[test]
    fn string_fits_entirely() {
        let mut buf = make_buf(Size::new(10, 3));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "hello");
        assert_eq!(written, 5);

        buf_assert_eq!(buf, buffer![["h", "e", "l", "l", "o", empty(5)], empty(2)]);
    }

    #[test]
    fn string_truncated_at_right_edge() {
        let mut buf = make_buf(Size::new(5, 3));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "hello world");
        assert_eq!(written, 5);

        buf_assert_eq!(buf, buffer![["h", "e", "l", "l", "o"], empty(2)]);
    }

    #[test]
    fn string_exactly_fills_width() {
        let mut buf = make_buf(Size::new(5, 2));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "abcde");
        assert_eq!(written, 5);

        buf_assert_eq!(buf, buffer![["a", "b", "c", "d", "e"], empty(1)]);
    }

    #[test]
    fn start_at_nonzero_x() {
        let mut buf = make_buf(Size::new(10, 3));
        let written = draw_string(make_fc(&mut buf), Position::new(4, 0), "hello");
        assert_eq!(written, 5);

        buf_assert_eq!(
            buf,
            buffer![[empty(4), "h", "e", "l", "l", "o", " "], empty(2)]
        );
    }

    #[test]
    fn start_at_nonzero_y() {
        let mut buf = make_buf(Size::new(10, 3));
        let written = draw_string(make_fc(&mut buf), Position::new(0, 2), "hello");
        assert_eq!(written, 5);

        buf_assert_eq!(buf, buffer![empty(2), ["h", "e", "l", "l", "o", empty(5)],]);
    }

    #[test]
    fn start_at_nonzero_x_and_y() {
        let mut buf = make_buf(Size::new(10, 3));
        let written = draw_string(make_fc(&mut buf), Position::new(3, 1), "test");
        assert_eq!(written, 4);

        buf_assert_eq!(
            buf,
            buffer![
                empty(1),
                [" ", " ", " ", "t", "e", "s", "t", " ", " ", " "],
                empty(1)
            ]
        );
    }

    #[test]
    fn start_at_nonzero_x_truncated() {
        let mut buf = make_buf(Size::new(8, 2));
        let written = draw_string(make_fc(&mut buf), Position::new(5, 0), "hello");
        assert_eq!(written, 3);

        buf_assert_eq!(buf, buffer![[empty(5), "h", "e", "l"], empty(1),]);
    }

    #[test]
    fn single_cell_buffer() {
        let mut buf = make_buf(Size::new(1, 1));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "hello");
        assert_eq!(written, 1);

        buf_assert_eq!(buf, buffer![["h"]]);
    }

    #[test]
    fn single_char_string() {
        let mut buf = make_buf(Size::new(5, 2));
        let written = draw_string(make_fc(&mut buf), Position::new(2, 0), "x");
        assert_eq!(written, 1);

        buf_assert_eq!(buf, buffer![[empty(2), "x", empty(2)], empty(1)])
    }

    #[test]
    fn emoji_takes_two_cells() {
        let mut buf = make_buf(Size::new(5, 2));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "😀");
        assert_eq!(written, 2);

        buf_assert_eq!(buf, buffer![["😀", empty(4)], [empty(5)]]);
    }

    #[test]
    fn multiple_emoji() {
        let mut buf = make_buf(Size::new(6, 2));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "😀😎");
        assert_eq!(written, 4);

        buf_assert_eq!(
            buf,
            buffer![["😀", empty(1), "😎", empty(1), empty(2)], empty(1)]
        );
    }

    #[test]
    fn emoji_truncated() {
        let mut buf = make_buf(Size::new(3, 2));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "😀😀😀");
        assert_eq!(written, 2);

        buf_assert_eq!(buf, buffer![["😀", empty(2)], empty(1)]);
    }

    #[test]
    fn emoji_doesnt_fit_single_cell() {
        let mut buf = make_buf(Size::new(1, 2));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "😀");
        assert_eq!(written, 0);

        buf_assert_eq!(buf, buffer![empty(1), [" "]]);
    }

    #[test]
    fn cjk_char_takes_two_cells() {
        let mut buf = make_buf(Size::new(5, 2));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "中");
        assert_eq!(written, 2);

        buf_assert_eq!(buf, buffer![["中", empty(4)], empty(1)]);
    }

    #[test]
    fn cjk_mixed_with_ascii() {
        let mut buf = make_buf(Size::new(10, 2));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "中文abc");
        assert_eq!(written, 7);

        buf_assert_eq!(
            buf,
            buffer![
                ["中", empty(1), "文", empty(1), "a", "b", "c", empty(3)],
                empty(1)
            ]
        );
    }

    #[test]
    fn cjk_truncated() {
        let mut buf = make_buf(Size::new(3, 2));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "中文");
        assert_eq!(written, 2);

        buf_assert_eq![buf, buffer![["中", empty(2)], empty(1)]];
    }

    #[test]
    fn superscript_takes_one_cell() {
        let mut buf = make_buf(Size::new(5, 2));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "x²");
        assert_eq!(written, 2);

        buf_assert_eq!(buf, buffer![["x", "²", empty(3)], empty(1)]);
    }

    #[test]
    fn mathematical_symbols() {
        let mut buf = make_buf(Size::new(10, 2));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "x²+y³=z⁴");
        assert_eq!(written, 8);

        buf_assert_eq!(
            buf,
            buffer![["x", "²", "+", "y", "³", "=", "z", "⁴", empty(2)], empty(1)]
        );
    }

    #[test]
    fn mixed_unicode_content() {
        let mut buf = make_buf(Size::new(15, 2));
        let written = draw_string(make_fc(&mut buf), Position::ZERO, "Hi中😀文");

        assert_eq!(written, 8);
        buf_assert_eq!(
            buf,
            buffer!(
                ["H", "i", "中", empty(1), "😀", empty(1), "文", empty(8)],
                [empty(15)]
            )
        )
    }
}
