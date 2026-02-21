use crate::{
    cell::{Cell, CellFormat},
    color::Color,
    engine2::{
        buffer::{Buffer, Drawer},
        draw::Position,
    },
    rich_text::Attributes,
};

#[doc(hidden)]
pub fn cell_for_pos(pos: Position) -> Cell {
    let [x1, x2] = pos.x.to_be_bytes();
    let [y1, y2] = pos.y.to_be_bytes();
    Cell {
        ch: 'a',
        fg: Color::new(x1, x2, y1, y2),
        bg: Color::new(x2, x1, y2, y1),
        attributes: Attributes::empty(),
        format: CellFormat::Standard,
    }
}

#[doc(hidden)]
pub fn draw_sorted<Buf: Buffer + Drawer>(buf: &mut Buf) -> Vec<(u16, u16, char)> {
    let mut calls: Vec<_> = buf
        .draw()
        .map(|dc| (dc.pos.x, dc.pos.y, dc.cell.ch))
        .collect();
    calls.sort();
    buf.end_frame();
    buf.start_frame();
    calls
}

/// Generates a test module exercising the [`Buffer`] contract for a concrete
/// type.
///
/// # Parameters
///
/// - `$module_name` - the name of the generated `mod` (e.g. `paired_buffer`).
/// - `$constructor` - an expression that takes a [`Size`] and returns an
///   instance of `$buffer_type` (e.g. `PairedBuffer::new`).
/// - `$buffer_type` - the concrete type under test; must implement [`Buffer`].
///
/// # Tests generated
///
/// - **`size`** - `size()` returns the value passed to the constructor.
/// - **`set_cell_checked` / `get_cell_checked`** - round-trip at the origin,
///   the last valid position, and an arbitrary interior position; all three
///   out-of-bounds variants (`X`, `Y`, `XY`) return the correct error.
/// - **`set_cell` / `get_cell`** - infallible round-trip; both panic when the
///   position is out of bounds.
/// - **`get_cell_mut_checked`** - mutation through a mutable reference; all
///   three out-of-bounds variants return the correct error.
/// - **`get_cell_mut`** - mutation round-trip; panics when out of bounds.
/// - **Independence** - writes to distinct positions do not alias each other.
/// - **Overwrite** - writing a second value to the same position replaces the
///   first.
/// - **`fill`** - every cell in the grid equals the fill value afterwards.
/// - **`clear`** - every cell equals [`Cell::EMPTY`] after clearing.
#[macro_export]
macro_rules! buffer_tests {
    ($module_name:ident, $constructor:expr, $buffer_type:ty) => {
        mod $module_name {
    #[rustfmt::skip]
            use super::{$buffer_type};
            use $crate::{
                cell::Cell,
                engine2::buffer::test::cell_for_pos,
                engine2::{
                    buffer::{Buffer, ErrorOutOfBoundsAxises},
                    draw::{Position, Size},
                },
            };

            type Buf = $buffer_type;

            fn new_buf(size: Size) -> Buf {
                let mut buf = $constructor(size);
                buf.start_frame();
                buf
            }

            // size

            #[test]
            fn size_matches_constructor() {
                let size = Size::new(10, 5);
                let buf = new_buf(size);
                assert_eq!(buf.size(), size);
            }

            #[test]
            fn size_1x1() {
                let buf = new_buf(Size::new(1, 1));
                assert_eq!(buf.size(), Size::new(1, 1));
            }

            // set_cell_checked / get_cell_checked

            #[test]
            fn set_and_get_cell_checked_origin() {
                let mut buf = new_buf(Size::new(5, 5));
                let pos = Position::ZERO;
                buf.set_cell_checked(pos, cell_for_pos(pos)).unwrap();
                assert_eq!(buf.get_cell_checked(pos).unwrap(), &cell_for_pos(pos));
            }

            #[test]
            fn set_and_get_cell_checked_last_valid() {
                let mut buf = new_buf(Size::new(5, 5));
                let pos = Position::new(4, 4);
                buf.set_cell_checked(pos, cell_for_pos(pos)).unwrap();
                assert_eq!(buf.get_cell_checked(pos).unwrap(), &cell_for_pos(pos));
            }

            #[test]
            fn set_and_get_cell_checked_arbitrary() {
                let mut buf = new_buf(Size::new(8, 6));
                let pos = Position::new(3, 2);
                buf.set_cell_checked(pos, cell_for_pos(pos)).unwrap();
                assert_eq!(buf.get_cell_checked(pos).unwrap(), &cell_for_pos(pos));
            }

            #[test]
            fn set_cell_checked_x_out_of_bounds() {
                let mut buf = new_buf(Size::new(4, 4));
                let err = buf
                    .set_cell_checked(Position::new(4, 0), cell_for_pos(Position::ZERO))
                    .unwrap_err();
                assert_eq!(err, ErrorOutOfBoundsAxises::X);
            }

            #[test]
            fn set_cell_checked_y_out_of_bounds() {
                let mut buf = new_buf(Size::new(4, 4));
                let err = buf
                    .set_cell_checked(Position::new(0, 4), cell_for_pos(Position::ZERO))
                    .unwrap_err();
                assert_eq!(err, ErrorOutOfBoundsAxises::Y);
            }

            #[test]
            fn set_cell_checked_xy_out_of_bounds() {
                let mut buf = new_buf(Size::new(4, 4));
                let err = buf
                    .set_cell_checked(Position::new(4, 4), cell_for_pos(Position::ZERO))
                    .unwrap_err();
                assert_eq!(err, ErrorOutOfBoundsAxises::XY);
            }

            #[test]
            fn get_cell_checked_x_out_of_bounds() {
                let buf = new_buf(Size::new(4, 4));
                let err = buf.get_cell_checked(Position::new(4, 0)).unwrap_err();
                assert_eq!(err, ErrorOutOfBoundsAxises::X);
            }

            #[test]
            fn get_cell_checked_y_out_of_bounds() {
                let buf = new_buf(Size::new(4, 4));
                let err = buf.get_cell_checked(Position::new(0, 4)).unwrap_err();
                assert_eq!(err, ErrorOutOfBoundsAxises::Y);
            }

            #[test]
            fn get_cell_checked_xy_out_of_bounds() {
                let buf = new_buf(Size::new(4, 4));
                let err = buf.get_cell_checked(Position::new(4, 4)).unwrap_err();
                assert_eq!(err, ErrorOutOfBoundsAxises::XY);
            }

            // set_cell / get_cell (infallible)

            #[test]
            fn set_and_get_cell_infallible() {
                let mut buf = new_buf(Size::new(5, 5));
                let pos = Position::new(2, 3);
                buf.set_cell(pos, cell_for_pos(pos));
                assert_eq!(buf.get_cell(pos), &cell_for_pos(pos));
            }

            #[test]
            #[should_panic]
            fn set_cell_panics_out_of_bounds() {
                let mut buf = new_buf(Size::new(4, 4));
                buf.set_cell(Position::new(10, 10), cell_for_pos(Position::ZERO));
            }

            #[test]
            #[should_panic]
            fn get_cell_panics_out_of_bounds() {
                let buf = new_buf(Size::new(4, 4));
                buf.get_cell(Position::new(10, 10));
            }

            // get_cell_mut_checked

            #[test]
            fn get_cell_mut_checked_and_modify() {
                let mut buf = new_buf(Size::new(5, 5));
                let pos = Position::new(1, 1);
                *buf.get_cell_mut_checked(pos).unwrap() = cell_for_pos(pos);
                assert_eq!(buf.get_cell(pos), &cell_for_pos(pos));
            }

            #[test]
            fn get_cell_mut_checked_x_out_of_bounds() {
                let mut buf = new_buf(Size::new(4, 4));
                let err = buf.get_cell_mut_checked(Position::new(4, 0)).unwrap_err();
                assert_eq!(err, ErrorOutOfBoundsAxises::X);
            }

            #[test]
            fn get_cell_mut_checked_y_out_of_bounds() {
                let mut buf = new_buf(Size::new(4, 4));
                let err = buf.get_cell_mut_checked(Position::new(0, 4)).unwrap_err();
                assert_eq!(err, ErrorOutOfBoundsAxises::Y);
            }

            #[test]
            fn get_cell_mut_checked_xy_out_of_bounds() {
                let mut buf = new_buf(Size::new(4, 4));
                let err = buf.get_cell_mut_checked(Position::new(4, 4)).unwrap_err();
                assert_eq!(err, ErrorOutOfBoundsAxises::XY);
            }

            // get_cell_mut (infallible)

            #[test]
            fn get_cell_mut_infallible_and_modify() {
                let mut buf = new_buf(Size::new(5, 5));
                let pos = Position::new(2, 2);
                *buf.get_cell_mut(pos) = cell_for_pos(pos);
                assert_eq!(buf.get_cell(pos), &cell_for_pos(pos));
            }

            #[test]
            #[should_panic]
            fn get_cell_mut_panics_out_of_bounds() {
                let mut buf = new_buf(Size::new(4, 4));
                buf.get_cell_mut(Position::new(10, 10));
            }

            // writes are independent per cell

            #[test]
            fn writes_do_not_alias() {
                let mut buf = new_buf(Size::new(4, 4));
                let pos_a = Position::ZERO;
                let pos_b = Position::new(3, 3);
                buf.set_cell(pos_a, cell_for_pos(pos_a));
                buf.set_cell(pos_b, cell_for_pos(pos_b));
                assert_eq!(buf.get_cell(pos_a), &cell_for_pos(pos_a));
                assert_eq!(buf.get_cell(pos_b), &cell_for_pos(pos_b));
            }

            #[test]
            fn adjacent_cells_are_independent() {
                let mut buf = new_buf(Size::new(3, 1));
                let pos0 = Position::ZERO;
                let pos1 = Position::new(1, 0);
                let pos2 = Position::new(2, 0);
                buf.set_cell(pos0, cell_for_pos(pos0));
                buf.set_cell(pos1, cell_for_pos(pos1));
                buf.set_cell(pos2, cell_for_pos(pos2));
                assert_eq!(buf.get_cell(pos0), &cell_for_pos(pos0));
                assert_eq!(buf.get_cell(pos1), &cell_for_pos(pos1));
                assert_eq!(buf.get_cell(pos2), &cell_for_pos(pos2));
            }

            // overwrite

            #[test]
            fn overwriting_a_cell_reflects_new_value() {
                let mut buf = new_buf(Size::new(4, 4));
                let pos = Position::new(1, 1);
                let first = cell_for_pos(Position::ZERO);
                let second = cell_for_pos(Position::new(1, 0));
                buf.set_cell(pos, first);
                buf.set_cell(pos, second.clone());
                assert_eq!(buf.get_cell(pos), &second);
            }

            // fill

            #[test]
            fn fill_sets_every_cell() {
                let size = Size::new(4, 3);
                let mut buf = new_buf(size);
                let fill_cell = cell_for_pos(Position::ZERO);
                buf.fill(fill_cell.clone());
                for y in 0..size.height {
                    for x in 0..size.width {
                        assert_eq!(
                            buf.get_cell(Position::new(x, y)),
                            &fill_cell,
                            "cell at ({x},{y}) should equal the fill cell after fill"
                        );
                    }
                }
            }

            // clear

            #[test]
            fn clear_sets_every_cell_to_empty() {
                let size = Size::new(4, 3);
                let mut buf = new_buf(size);
                buf.fill(cell_for_pos(Position::ZERO));
                buf.clear();
                for y in 0..size.height {
                    for x in 0..size.width {
                        assert_eq!(
                            buf.get_cell(Position::new(x, y)),
                            &Cell::EMPTY,
                            "cell at ({x},{y}) should be EMPTY after clear"
                        );
                    }
                }
            }
        }
    };
}

/// Generates tests for any type implementing [`Buffer`] + [`Drawer`] that
/// always emits every cell on every call to `draw()`, regardless of whether
/// the cell changed since the last frame.
#[macro_export]
macro_rules! drawer_buffer_tests {
    ($module_name:ident, $constructor:expr, $buffer_type:ty) => {
        mod $module_name {
    #[rustfmt::skip]
            use super::{$buffer_type};
            use $crate::{
                engine2::buffer::test::{cell_for_pos, draw_sorted},
                engine2::{
                    buffer::Buffer,
                    draw::{Position, Size},
                },
            };

            type Buf = $buffer_type;

            fn new_buf(size: Size) -> Buf {
                let mut buf = $constructor(size);
                buf.start_frame();
                buf
            }

            // draw() always emits every cell in the buffer as this buffer is not diffed

            #[test]
            fn draw_emits_all_cells() {
                let size = Size::new(3, 2);
                let mut buf = new_buf(size);
                let calls = draw_sorted(&mut buf);
                assert_eq!(
                    calls.len(),
                    (size.width * size.height) as usize,
                    "draw() must emit every cell"
                );
            }

            // each position appears exactly once

            #[test]
            fn draw_emits_each_position_once() {
                let size = Size::new(3, 2);
                let mut buf = new_buf(size);
                let calls = draw_sorted(&mut buf);
                let mut positions: Vec<(u16, u16)> =
                    calls.iter().map(|&(x, y, _)| (x, y)).collect();
                positions.dedup();
                assert_eq!(
                    positions.len(),
                    (size.width * size.height) as usize,
                    "each position must appear exactly once"
                );
            }

            // draw() still emits every cell even when nothing has changed

            #[test]
            fn draw_emits_all_cells_when_unchanged() {
                let size = Size::new(3, 2);
                let mut buf = new_buf(size);
                buf.fill(cell_for_pos(Position::ZERO));
                let _ = draw_sorted(&mut buf); // first draw
                // Nothing written to the buffer between draws.
                let calls = draw_sorted(&mut buf);
                assert_eq!(
                    calls.len(),
                    (size.width * size.height) as usize,
                    "draw() must emit every cell even when nothing changed"
                );
            }

            // cell values in the output match what was written

            #[test]
            fn draw_output_matches_written_cells() {
                let size = Size::new(2, 2);
                let mut buf = new_buf(size);
                let pos_a = Position::ZERO;
                let pos_b = Position::new(1, 1);
                buf.set_cell(pos_a, cell_for_pos(pos_a));
                buf.set_cell(pos_b, cell_for_pos(pos_b));
                let calls = draw_sorted(&mut buf);
                let find = |x, y| calls.iter().find(|&&(cx, cy, _)| cx == x && cy == y);
                assert_eq!(
                    find(pos_a.x, pos_a.y).map(|&(_, _, ch)| ch),
                    Some(cell_for_pos(pos_a).ch)
                );
                assert_eq!(
                    find(pos_b.x, pos_b.y).map(|&(_, _, ch)| ch),
                    Some(cell_for_pos(pos_b).ch)
                );
            }

            // draw() covers the full grid even after a partial write

            #[test]
            fn draw_covers_full_grid_after_partial_write() {
                let size = Size::new(4, 3);
                let mut buf = new_buf(size);
                let pos = Position::new(1, 1);
                buf.set_cell(pos, cell_for_pos(pos));
                let calls = draw_sorted(&mut buf);
                assert_eq!(
                    calls.len(),
                    (size.width * size.height) as usize,
                    "draw() must emit every cell, not just the written one"
                );
            }
        }
    };
}

/// Generates tests for any type implementing [`Buffer`] + [`Drawer`] that
/// diffs frames and only emits cells that changed since the last `draw()`.
///
/// The constructor receives `(size, inner_buf_1, inner_buf_2)`.
#[macro_export]
macro_rules! drawer_diffed_buffer_tests {
    ($module_name:ident, $constructor:expr, $buffer_type:ty) => {
        mod $module_name {
    #[rustfmt::skip]
            use super::{$buffer_type};
            use $crate::{
                cell::Cell,
                engine2::{
                    buffer::{
                        Buffer,
                        test::{cell_for_pos, draw_sorted},
                    },
                    draw::{Position, Size},
                },
            };

            type Buf = $buffer_type;

            fn new_buf(size: Size) -> Buf {
                let mut buf = $constructor(size);
                buf.start_frame();
                buf
            }

            // fresh buffer: both frames are EMPTY so nothing differs

            #[test]
            fn fresh_buffer_no_draw_calls() {
                let mut buf = new_buf(Size::new(4, 4));
                assert_eq!(
                    draw_sorted(&mut buf).len(),
                    0,
                    "no cells differ on a fresh buffer"
                );
            }

            // a written cell is emitted exactly once

            #[test]
            fn changed_cell_emitted_once() {
                let mut buf = new_buf(Size::new(4, 4));
                let _ = draw_sorted(&mut buf); // swap: new current frame is blank

                let pos = Position::new(1, 2);
                buf.set_cell(pos, cell_for_pos(pos));
                let calls = draw_sorted(&mut buf);
                assert_eq!(calls, [(pos.x, pos.y, cell_for_pos(pos).ch)]);
            }

            // an unchanged cell is NOT emitted

            #[test]
            fn unchanged_cell_not_emitted() {
                let mut buf = new_buf(Size::new(4, 4));
                let _ = draw_sorted(&mut buf);

                let pos = Position::ZERO;
                buf.set_cell(pos, cell_for_pos(pos));
                let _ = draw_sorted(&mut buf); // cell_for_pos(pos) is now in the old frame

                // Write the same value again - no diff.
                buf.set_cell(pos, cell_for_pos(pos));
                assert_eq!(
                    draw_sorted(&mut buf).len(),
                    0,
                    "identical cell must not produce a draw call"
                );
            }

            // only the changed cells among many are emitted

            #[test]
            fn only_changed_cells_emitted() {
                let size = Size::new(4, 4);
                let mut buf = new_buf(size);
                let _ = draw_sorted(&mut buf);

                // Establish cell_for_pos(x,y) at every position.
                for y in 0..size.height {
                    for x in 0..size.width {
                        let pos = Position::new(x, y);
                        buf.set_cell(pos, cell_for_pos(pos));
                    }
                }
                let _ = draw_sorted(&mut buf);

                // In the new frame, only write (2,2) — all other positions are EMPTY
                // (start_frame cleared them), so every position except (2,2) differs
                // from its old value.
                let pos_changed = Position::new(2, 2);
                buf.set_cell(pos_changed, cell_for_pos(pos_changed));
                let calls = draw_sorted(&mut buf);

                // (2,2) is unchanged (same cell as old frame), so it must NOT appear.
                assert!(
                    !calls
                        .iter()
                        .any(|&(x, y, _)| x == pos_changed.x && y == pos_changed.y),
                    "the cell identical to the previous frame must not produce a draw call"
                );
                // Every other position changed (from cell_for_pos(x,y) to EMPTY),
                // so the total number of emitted cells must be width*height - 1.
                assert_eq!(
                    calls.len(),
                    (size.width * size.height - 1) as usize,
                    "all cells except the unchanged one must be emitted"
                );
            }

            // overwriting a cell with a different value is emitted

            #[test]
            fn overwrite_with_different_value_emitted() {
                let mut buf = new_buf(Size::new(4, 4));
                let _ = draw_sorted(&mut buf);

                let pos_a = Position::new(1, 1);
                let pos_b = Position::new(2, 1);
                buf.set_cell(pos_a, cell_for_pos(pos_a));
                let _ = draw_sorted(&mut buf); // cell_for_pos(pos_a) is now old frame

                buf.set_cell(pos_a, cell_for_pos(pos_b));
                let calls = draw_sorted(&mut buf);
                assert!(
                    calls.iter().any(|&(x, y, ch)| x == pos_a.x
                        && y == pos_a.y
                        && ch == cell_for_pos(pos_b).ch),
                    "overwritten cell must be emitted with its new value"
                );
            }

            // a cell that disappears (reverts to EMPTY) is emitted

            #[test]
            fn cleared_cell_emitted() {
                let mut buf = new_buf(Size::new(4, 4));
                let _ = draw_sorted(&mut buf);

                let pos = Position::new(2, 2);
                buf.set_cell(pos, cell_for_pos(pos));
                let _ = draw_sorted(&mut buf); // cell_for_pos(pos) is now old frame

                // Current frame is blank (start_frame cleared it); (2,2) now
                // differs from the old value, so the empty value must be emitted.
                let calls = draw_sorted(&mut buf);
                assert_eq!(
                    calls,
                    [(pos.x, pos.y, Cell::EMPTY.ch)],
                    "the cleared cell must be emitted with Cell::EMPTY's character"
                );
            }

            // when nothing changes, draw() emits nothing

            #[test]
            fn stable_frame_no_draw_calls() {
                let size = Size::new(3, 3);
                let mut buf = new_buf(size);

                let _ = draw_sorted(&mut buf);
                // start_frame cleared the current frame; old frame is EMPTY.
                // Write fill_cell and draw so old frame becomes fill_cell everywhere.
                let fill_cell = cell_for_pos(Position::ZERO);
                buf.fill(fill_cell.clone());
                let _ = draw_sorted(&mut buf);
                // start_frame cleared current frame; write fill_cell again to match old.
                buf.fill(fill_cell);
                assert_eq!(
                    draw_sorted(&mut buf).len(),
                    0,
                    "no draw calls when the frame is identical to the previous one"
                );
            }

            // first non-empty frame after a blank one emits all changed cells

            #[test]
            fn first_fill_emits_all_cells() {
                let size = Size::new(3, 2);
                let mut buf = new_buf(size);
                let _ = draw_sorted(&mut buf); // old frame = EMPTY, current = fresh blank

                buf.fill(cell_for_pos(Position::ZERO));
                let calls = draw_sorted(&mut buf);
                assert_eq!(
                    calls.len(),
                    (size.width * size.height) as usize,
                    "every cell differs from EMPTY so all must be emitted"
                );
            }

            // two independent positions are each emitted independently

            #[test]
            fn two_changed_cells_both_emitted() {
                let mut buf = new_buf(Size::new(4, 4));
                let _ = draw_sorted(&mut buf);

                let pos_a = Position::ZERO;
                let pos_b = Position::new(3, 3);
                buf.set_cell(pos_a, cell_for_pos(pos_a));
                buf.set_cell(pos_b, cell_for_pos(pos_b));
                let calls = draw_sorted(&mut buf);
                assert_eq!(
                    calls,
                    [
                        (pos_a.x, pos_a.y, cell_for_pos(pos_a).ch),
                        (pos_b.x, pos_b.y, cell_for_pos(pos_b).ch),
                    ]
                );
            }
        }
    };
}

/// Generates tests for any type implementing [`ResizableBuffer`].
///
/// # Parameters
///
/// - `$module_name` - the name of the generated `mod`.
/// - `$constructor` - an expression that takes a [`Size`] and returns an
///   instance of `$buffer_type`.
/// - `$buffer_type` - the concrete type under test; must implement
///   [`ResizableBuffer`].
///
/// # Tests generated
///
/// - **`size_after_resize`** - `size()` returns the new size after `resize()`.
/// - **`resize_larger`** - resizing to a larger grid succeeds and the new cells
///   are accessible without panic.
/// - **`resize_smaller`** - resizing to a smaller grid succeeds; cells within
///   the new bounds are still accessible.
/// - **`resize_to_same_size`** - resizing to the same dimensions is a no-op;
///   existing cell data is preserved.
/// - **`resize_then_write`** - writing to the last valid position after resize
///   round-trips correctly.
/// - **`resize_multiple_times`** - the buffer can be resized repeatedly; only
///   the final size is reported by `size()`.
/// - **`resize_to_1x1`** - the buffer can shrink to a single cell.
#[macro_export]
macro_rules! buffer_resizing_tests {
    ($module_name:ident, $constructor:expr, $buffer_type:ty) => {
        mod $module_name {
    #[rustfmt::skip]
            use super::{$buffer_type};
            use $crate::{
                engine2::buffer::test::cell_for_pos,
                engine2::{
                    buffer::{Buffer, ResizableBuffer},
                    draw::{Position, Size},
                },
            };

            type Buf = $buffer_type;

            fn new_buf(size: Size) -> Buf {
                let mut buf = $constructor(size);
                buf.start_frame();
                buf
            }

            #[test]
            fn size_after_resize() {
                let mut buf = new_buf(Size::new(4, 4));
                let new_size = Size::new(8, 6);
                buf.resize(new_size);
                assert_eq!(buf.size(), new_size);
            }

            #[test]
            fn resize_larger() {
                let mut buf = new_buf(Size::new(2, 2));
                buf.resize(Size::new(5, 5));
                // Writing and reading the new last-valid position must not panic.
                let pos = Position::new(4, 4);
                buf.set_cell(pos, cell_for_pos(pos));
                assert_eq!(buf.get_cell(pos), &cell_for_pos(pos));
            }

            #[test]
            fn resize_smaller() {
                let mut buf = new_buf(Size::new(6, 6));
                buf.resize(Size::new(3, 3));
                assert_eq!(buf.size(), Size::new(3, 3));
                // Cells within the new bounds must still be accessible.
                let pos = Position::new(2, 2);
                buf.set_cell(pos, cell_for_pos(pos));
                assert_eq!(buf.get_cell(pos), &cell_for_pos(pos));
            }

            #[test]
            fn resize_to_same_size() {
                let mut buf = new_buf(Size::new(4, 4));
                let pos = Position::new(1, 1);
                buf.set_cell(pos, cell_for_pos(pos));
                buf.resize(Size::new(4, 4));
                assert_eq!(buf.size(), Size::new(4, 4));
                assert_eq!(buf.get_cell(pos), &cell_for_pos(pos));
            }

            #[test]
            fn resize_then_write() {
                let mut buf = new_buf(Size::new(2, 2));
                let new_size = Size::new(7, 3);
                buf.resize(new_size);
                let pos = Position::new(new_size.width - 1, new_size.height - 1);
                buf.set_cell(pos, cell_for_pos(pos));
                assert_eq!(buf.get_cell(pos), &cell_for_pos(pos));
            }

            #[test]
            fn resize_multiple_times() {
                let mut buf = new_buf(Size::new(2, 2));
                buf.resize(Size::new(10, 10));
                buf.resize(Size::new(3, 3));
                buf.resize(Size::new(6, 4));
                assert_eq!(buf.size(), Size::new(6, 4));
            }

            #[test]
            fn resize_to_1x1() {
                let mut buf = new_buf(Size::new(5, 5));
                buf.resize(Size::new(1, 1));
                assert_eq!(buf.size(), Size::new(1, 1));
                let pos = Position::ZERO;
                buf.set_cell(pos, cell_for_pos(pos));
                assert_eq!(buf.get_cell(pos), &cell_for_pos(pos));
            }

            #[test]
            fn content_preserved_after_grow() {
                let mut buf = new_buf(Size::new(2, 2));
                let pos = Position::ZERO;
                buf.set_cell(pos, cell_for_pos(pos));
                buf.resize(Size::new(4, 4));
                assert_eq!(buf.get_cell(pos), &cell_for_pos(pos));
            }

            #[test]
            fn content_preserved_after_shrink() {
                let mut buf = new_buf(Size::new(4, 4));
                let pos = Position::ZERO;
                buf.set_cell(pos, cell_for_pos(pos));
                buf.resize(Size::new(2, 2));
                assert_eq!(buf.get_cell(pos), &cell_for_pos(pos));
            }
        }
    };
}
