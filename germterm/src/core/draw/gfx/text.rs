use crate::core::{buffer::Buffer, draw::Position};

/// Draw the provided string starting at the specified position.
///
/// The provided string is written as much as possible in a single line of the buffer.
/// If the size exceeded the string is truncated. If the string is too short the remaining cells
/// will stay untouched.
///
/// # Returns
///
/// Returns the number of characters that were written.
pub fn draw_string<Buf: Buffer>(buf: &mut Buf, start: Position, s: &str) -> u16 {
    let sz = buf.size();
    if sz.area() == 0 || !sz.is_within(start) {
        return 0;
    }

    let mut written = 0;
    // Write until we run out of characters or we are at the end of the buffer.
    for (x, ch) in (start.x..sz.width).zip(s.chars()) {
        written += 1;
        buf.get_cell_mut(Position { x, ..start }).ch = ch;
    }

    written
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        buf_str,
        core::{
            buffer::{flat::FlatBuffer, utils::dump_buffer_to_string as dbts},
            draw::Size,
        },
    };

    fn make_buf(sz: Size) -> FlatBuffer {
        FlatBuffer::new(sz)
    }

    #[test]
    fn zero_area_buffer_returns_zero() {
        let mut buf = make_buf(Size::ZERO);
        let written = draw_string(&mut buf, Position::ZERO, "hello");
        assert_eq!(written, 0);
    }

    #[test]
    fn zero_width_buffer_returns_zero() {
        let mut buf = make_buf(Size::new(0, 5));
        let written = draw_string(&mut buf, Position::ZERO, "hello");
        assert_eq!(written, 0);
    }

    #[test]
    fn zero_height_buffer_returns_zero() {
        let mut buf = make_buf(Size::new(5, 0));
        let written = draw_string(&mut buf, Position::ZERO, "hello");
        assert_eq!(written, 0);
    }

    #[test]
    fn start_x_out_of_bounds_returns_zero() {
        let mut buf = make_buf(Size::new(5, 3));
        let written = draw_string(&mut buf, Position::new(5, 0), "hello");
        assert_eq!(written, 0);

        assert_eq!(dbts(&buf), buf_str!["     ", "     ", "     ",]);
    }

    #[test]
    fn start_y_out_of_bounds_returns_zero() {
        let mut buf = make_buf(Size::new(5, 3));
        let written = draw_string(&mut buf, Position::new(0, 3), "hello");
        assert_eq!(written, 0);

        assert_eq!(dbts(&buf), buf_str!["     ", "     ", "     ",]);
    }

    #[test]
    fn empty_string_returns_zero() {
        let mut buf = make_buf(Size::new(5, 3));
        let written = draw_string(&mut buf, Position::ZERO, "");
        assert_eq!(written, 0);

        assert_eq!(dbts(&buf), buf_str!["     ", "     ", "     ",]);
    }

    #[test]
    fn string_fits_entirely() {
        let mut buf = make_buf(Size::new(10, 3));
        let written = draw_string(&mut buf, Position::ZERO, "hello");
        assert_eq!(written, 5);

        assert_eq!(
            dbts(&buf),
            buf_str!["hello     ", "          ", "          ",]
        );
    }

    #[test]
    fn string_truncated_at_right_edge() {
        let mut buf = make_buf(Size::new(5, 3));
        let written = draw_string(&mut buf, Position::ZERO, "hello world");
        assert_eq!(written, 5);

        assert_eq!(dbts(&buf), buf_str!["hello", "     ", "     ",]);
    }

    #[test]
    fn string_exactly_fills_width() {
        let mut buf = make_buf(Size::new(5, 2));
        let written = draw_string(&mut buf, Position::ZERO, "abcde");
        assert_eq!(written, 5);

        assert_eq!(dbts(&buf), buf_str!["abcde", "     ",]);
    }

    #[test]
    fn start_at_nonzero_x() {
        let mut buf = make_buf(Size::new(10, 3));
        let written = draw_string(&mut buf, Position::new(4, 0), "hello");
        assert_eq!(written, 5);

        assert_eq!(
            dbts(&buf),
            buf_str!["    hello ", "          ", "          ",]
        );
    }

    #[test]
    fn start_at_nonzero_y() {
        let mut buf = make_buf(Size::new(10, 3));
        let written = draw_string(&mut buf, Position::new(0, 2), "hello");
        assert_eq!(written, 5);

        assert_eq!(
            dbts(&buf),
            buf_str!["          ", "          ", "hello     ",]
        );
    }

    #[test]
    fn start_at_nonzero_x_and_y() {
        let mut buf = make_buf(Size::new(10, 3));
        let written = draw_string(&mut buf, Position::new(3, 1), "test");
        assert_eq!(written, 4);

        assert_eq!(
            dbts(&buf),
            buf_str!["          ", "   test   ", "          ",]
        );
    }

    #[test]
    fn start_at_nonzero_x_truncated() {
        let mut buf = make_buf(Size::new(8, 2));
        let written = draw_string(&mut buf, Position::new(5, 0), "hello");
        assert_eq!(written, 3);

        assert_eq!(dbts(&buf), buf_str!["     hel", "        ",]);
    }

    #[test]
    fn single_cell_buffer() {
        let mut buf = make_buf(Size::new(1, 1));
        let written = draw_string(&mut buf, Position::ZERO, "hello");
        assert_eq!(written, 1);

        assert_eq!(dbts(&buf), buf_str!["h"]);
    }

    #[test]
    fn single_char_string() {
        let mut buf = make_buf(Size::new(5, 2));
        let written = draw_string(&mut buf, Position::new(2, 0), "x");
        assert_eq!(written, 1);

        assert_eq!(dbts(&buf), buf_str!["  x  ", "     ",]);
    }
}
