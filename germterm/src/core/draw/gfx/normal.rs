use crate::{
    cell::Cell,
    core::{
        buffer::{Buffer, slice::SubBuffer},
        draw::{Position, Rect},
    },
    style::Style,
};

/// Draws a vertical line downward from `start` for up to `len` cells.
///
/// The provided `cell` is merged into each buffer cell the line touches.
/// If the line extends past the bottom edge of the buffer it is truncated.
/// If `start` is outside the buffer, nothing is drawn.
///
/// # Returns
///
/// The number of cells written.
pub fn draw_vline<Buf: Buffer>(buf: &mut Buf, start: Position, len: u16, cell: Cell) -> u16 {
    let sz = buf.size();
    if !sz.is_within(start) {
        return 0;
    }

    let renderable = sz.height - start.y;
    let to_render = renderable.min(len);
    for y in start.y..start.y + to_render {
        let cur = buf.get_cell_mut(Position::new(start.x, y));
        cur.merge(cell);
    }

    to_render
}

/// Draws a horizontal line rightward from `start` for up to `len` cells.
///
/// The provided `cell` is merged into each buffer cell the line touches.
/// If the line extends past the right edge of the buffer it is truncated.
/// If `start` is outside the buffer, nothing is drawn.
///
/// # Returns
///
/// The number of cells written.
pub fn draw_hline<Buf: Buffer>(buf: &mut Buf, start: Position, len: u16, cell: Cell) -> u16 {
    let sz = buf.size();
    if !sz.is_within(start) {
        return 0;
    }

    let renderable = sz.width - start.x;
    let to_render = renderable.min(len);
    for x in start.x..start.x + to_render {
        let cur = buf.get_cell_mut(Position::new(x, start.y));
        cur.merge(cell);
    }

    to_render
}

/// Draws a line between two points using
/// [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm).
///
/// The provided `cell` is merged into each buffer cell the line touches.
/// The line is truncated at the buffer edges; `start` and `end` may be
/// given in any order - the result is the same either way.
///
/// # Coordinate model
///
/// Terminal cells are twice as tall as they are wide, so the
/// algorithm works in a **doubled-x** coordinate space (each cell column
/// maps to two x-units). This keeps diagonal lines looking visually correct
/// without the caller having to think about aspect ratio.
///
/// # Returns
///
/// The number of cells written.
pub fn draw_line<Buf: Buffer>(buf: &mut Buf, start: Position, end: Position, cell: Cell) -> u32 {
    let sz = buf.size();
    if sz.area() == 0 {
        return 0;
    }

    // Always draw top-to-bottom (left-to-right when horizontal) so that
    // dy >= 0 and swapping start/end gives identical output.
    let (start, end) = if start.y < end.y || (start.y == end.y && start.x <= end.x) {
        (start, end)
    } else {
        (end, start)
    };

    // Each cell column occupies 2 x-units to compensate for the ~1:2 cell
    // aspect ratio. All arithmetic below works in this doubled space;
    // `cell_pos` converts back to buffer coordinates when plotting.
    let (mut x, mut y) = (start.x as i32 * 2, start.y as i32);
    let (ex, ey) = (end.x as i32 * 2, end.y as i32);

    // dy >= 0 is guaranteed by canonicalization. dx may be negative (line
    // goes left), so we track its sign separately.
    let dy = ey - y;
    let x_dir = (ex - x).signum();
    let dx = (ex - x).abs();

    let cell_pos = |x: i32, y: i32| Position::new((x / 2) as u16, y as u16);
    let (w, h) = (sz.width as i32 * 2, sz.height as i32);

    // Steep  (2 * dy > dx): major = y, minor = x.  Iterate one row per step.
    // Shallow (otherwise): major = x, minor = y.  Iterate one column per step.
    let steep = dy * 2 > dx;

    let (steps, major_step, minor_step) = if steep {
        (dy, (0, 1), (2 * x_dir, 0))
    } else {
        (dx / 2, (2 * x_dir, 0), (0, 1))
    };

    // Clamp major axis to buffer
    let steps = steps.min(if steep {
        h - 1 - y
    } else if x_dir >= 0 {
        (w - 1 - x) / 2
    } else {
        x / 2
    });

    // step_inc  - added to the error every iteration
    // error     - initial error (biased so "add then check" works slope is at either direction)
    // error_dec - subtracted when the minor axis advances
    let (step_inc, mut error, error_dec) = if steep {
        (dx, -dy, 2 * dy)
    } else {
        (2 * dy, 2 * dy - dx, dx)
    };

    // Draw
    for i in 0..=steps {
        buf.get_cell_mut(cell_pos(x, y)).merge(cell);

        error += step_inc;
        if error >= 0 {
            let (nx, ny) = (x + minor_step.0, y + minor_step.1);
            // Minor axis wants to advance - check it stays in bounds.
            if nx < 0 || nx >= w || ny >= h {
                return (i + 1) as u32;
            }
            x = nx;
            y = ny;
            error -= error_dec;
        }

        x += major_step.0;
        y += major_step.1;
    }

    (steps + 1) as u32
}

/// Set the style of the cells in `area`.
///
/// The style is set using [`Style::merge`].
///
/// # Returns
///
/// The number of cells that had its style merged.
pub fn draw_style<Buf: Buffer>(buf: &mut Buf, area: Rect, style: Style) -> u32 {
    let sz = buf.size();
    if sz.area_is_within(area) {
        return 0;
    }

    let sub = SubBuffer::new(buf, area);
    let sz = sub.size();

    // TODO: finish implementing this once Cell stores Style
    for y in 0..sz.height {
        for x in 0..sz.width {
            todo!();
            // sub.get_cell_mut(Position { x, y }).style.merge(style);
        }
    }

    sz.area()
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

    fn test_cell() -> Cell {
        Cell {
            ch: '#',
            ..Cell::EMPTY
        }
    }

    fn make_buf(sz: Size) -> FlatBuffer {
        FlatBuffer::new(sz)
    }

    #[test]
    fn vertical_line() {
        let mut buf = make_buf(Size::new(10, 10));
        let len = 10;
        let written = draw_vline(&mut buf, Position::ZERO, len, test_cell());
        assert_eq!(written, len);

        assert_eq!(
            dbts(&buf),
            buf_str![
                "#         ",
                "#         ",
                "#         ",
                "#         ",
                "#         ",
                "#         ",
                "#         ",
                "#         ",
                "#         ",
                "#         ",
            ]
        );
    }

    #[test]
    fn vertical_line_partial() {
        let mut buf = make_buf(Size::new(10, 10));
        let len = 6;
        let start = Position::new(4, 4);
        let written = draw_vline(&mut buf, start, len, test_cell());
        assert_eq!(written, len);

        assert_eq!(
            dbts(&buf),
            buf_str![
                "          ",
                "          ",
                "          ",
                "          ",
                "    #     ",
                "    #     ",
                "    #     ",
                "    #     ",
                "    #     ",
                "    #     ",
            ]
        );
    }

    #[test]
    fn vertical_line_truncated_bottom() {
        let mut buf = make_buf(Size::new(10, 10));
        let written = draw_vline(&mut buf, Position::new(4, 7), 10, test_cell());
        assert_eq!(written, 3);

        assert_eq!(
            dbts(&buf),
            buf_str![
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "    #     ",
                "    #     ",
                "    #     ",
            ],
        );
    }

    #[test]
    fn vertical_line_truncated_top() {
        let mut buf = make_buf(Size::new(10, 10));
        let written = draw_vline(&mut buf, Position::new(4, 8), 5, test_cell());
        assert_eq!(written, 2);

        assert_eq!(
            dbts(&buf),
            buf_str![
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "    #     ",
                "    #     ",
            ],
        );
    }

    #[test]
    fn horizontal_line() {
        let mut buf = make_buf(Size::new(10, 10));
        let len = 10;
        let written = draw_hline(&mut buf, Position::ZERO, len, test_cell());
        assert_eq!(written, len);

        assert_eq!(
            dbts(&buf),
            buf_str![
                "##########",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );
    }

    #[test]
    fn horizontal_line_partial() {
        let mut buf = make_buf(Size::new(10, 10));
        let len = 4;
        let written = draw_hline(&mut buf, Position::new(2, 2), len, test_cell());
        assert_eq!(written, len);

        assert_eq!(
            dbts(&buf),
            buf_str![
                "          ",
                "          ",
                "  ####    ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );
    }

    #[test]
    fn horizontal_line_truncated_right() {
        let mut buf = make_buf(Size::new(10, 10));
        let written = draw_hline(&mut buf, Position::new(7, 4), 10, test_cell());
        assert_eq!(written, 3);

        assert_eq!(
            dbts(&buf),
            buf_str![
                "          ",
                "          ",
                "          ",
                "          ",
                "       ###",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );
    }

    #[test]
    fn horizontal_line_truncated_left() {
        let mut buf = make_buf(Size::new(10, 10));
        let written = draw_hline(&mut buf, Position::new(15, 4), 5, test_cell());
        assert_eq!(written, 0);

        assert_eq!(
            dbts(&buf),
            buf_str![
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );
    }

    #[test]
    fn diagonal_line() {
        let mut buf = make_buf(Size::new(10, 10));
        let drawn = draw_line(
            &mut buf,
            Position::new(0, 0),
            Position::new(9, 9),
            test_cell(),
        );

        assert_eq!(drawn, 10);

        assert_eq!(
            dbts(&buf),
            buf_str![
                "#         ",
                " #        ",
                "  #       ",
                "   #      ",
                "    #     ",
                "     #    ",
                "      #   ",
                "       #  ",
                "        # ",
                "         #",
            ],
        );
    }

    #[test]
    fn diagonal_line_partial() {
        let mut buf = make_buf(Size::new(10, 10));
        let drawn = draw_line(
            &mut buf,
            Position::new(2, 1),
            Position::new(5, 7),
            test_cell(),
        );

        assert_eq!(drawn, 7);

        assert_eq!(
            dbts(&buf),
            buf_str![
                "          ",
                "  #       ",
                "   #      ",
                "   #      ",
                "    #     ",
                "    #     ",
                "     #    ",
                "     #    ",
                "          ",
                "          ",
            ],
        );
    }

    #[test]
    fn diagonal_line_inverted() {
        let mut buf = make_buf(Size::new(10, 10));
        let drawn = draw_line(
            &mut buf,
            Position::new(5, 7),
            Position::new(2, 1),
            test_cell(),
        );

        assert_eq!(drawn, 7);

        assert_eq!(
            dbts(&buf),
            buf_str![
                "          ",
                "  #       ",
                "   #      ",
                "   #      ",
                "    #     ",
                "    #     ",
                "     #    ",
                "     #    ",
                "          ",
                "          ",
            ],
        );
    }

    #[test]
    fn diagonal_line_truncated_bottom() {
        let mut buf = make_buf(Size::new(10, 10));
        let drawn = draw_line(
            &mut buf,
            Position::new(1, 3),
            Position::new(6, 15),
            test_cell(),
        );

        assert_eq!(drawn, 7);

        assert_eq!(
            dbts(&buf),
            buf_str![
                "          ",
                "          ",
                "          ",
                " #        ",
                " #        ",
                "  #       ",
                "  #       ",
                "   #      ",
                "   #      ",
                "    #     ",
            ],
        );
    }

    #[test]
    fn diagonal_line_truncated_right() {
        let mut buf = make_buf(Size::new(10, 10));
        let drawn = draw_line(
            &mut buf,
            Position::new(5, 2),
            Position::new(20, 5),
            test_cell(),
        );

        assert_eq!(drawn, 5);

        assert_eq!(
            dbts(&buf),
            buf_str![
                "          ",
                "          ",
                "     #### ",
                "         #",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );
    }

    #[test]
    fn diagonal_line_truncated_left() {
        let mut buf = make_buf(Size::new(10, 10));
        let drawn = draw_line(
            &mut buf,
            Position::new(8, 3),
            Position::new(2, 15),
            test_cell(),
        );

        assert_eq!(drawn, 7);

        assert_eq!(
            dbts(&buf),
            buf_str![
                "          ",
                "          ",
                "          ",
                "        # ",
                "       #  ",
                "       #  ",
                "      #   ",
                "      #   ",
                "     #    ",
                "     #    ",
            ],
        );
    }
}
