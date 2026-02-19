macro_rules! buffer_tests {
    ($module_name:ident, $constructor:tt, $buffer_type:ty) => {
        mod $module_name {
            use germterm::{
                cell::Cell,
                engine2::{
                    buffer::{Buffer, ErrorOutOfBoundsAxises},
                    draw::{Position, Size},
                },
            };

            type Buf = $buffer_type;

            fn new_buf(size: Size) -> Buf {
                $constructor(size)
            }

            // A non-empty cell distinct from Cell::EMPTY, for use in tests.
            fn cell_a() -> Cell {
                Cell {
                    ch: 'A',
                    ..Cell::EMPTY
                }
            }

            fn cell_b() -> Cell {
                Cell {
                    ch: 'B',
                    ..Cell::EMPTY
                }
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
                let pos = Position::new(0, 0);
                buf.set_cell_checked(pos, cell_a()).unwrap();
                assert_eq!(buf.get_cell_checked(pos).unwrap(), &cell_a());
            }

            #[test]
            fn set_and_get_cell_checked_last_valid() {
                let mut buf = new_buf(Size::new(5, 5));
                let pos = Position::new(4, 4);
                buf.set_cell_checked(pos, cell_a()).unwrap();
                assert_eq!(buf.get_cell_checked(pos).unwrap(), &cell_a());
            }

            #[test]
            fn set_and_get_cell_checked_arbitrary() {
                let mut buf = new_buf(Size::new(8, 6));
                let pos = Position::new(3, 2);
                buf.set_cell_checked(pos, cell_b()).unwrap();
                assert_eq!(buf.get_cell_checked(pos).unwrap(), &cell_b());
            }

            #[test]
            fn set_cell_checked_x_out_of_bounds() {
                let mut buf = new_buf(Size::new(4, 4));
                let err = buf
                    .set_cell_checked(Position::new(4, 0), cell_a())
                    .unwrap_err();
                assert_eq!(err, ErrorOutOfBoundsAxises::X);
            }

            #[test]
            fn set_cell_checked_y_out_of_bounds() {
                let mut buf = new_buf(Size::new(4, 4));
                let err = buf
                    .set_cell_checked(Position::new(0, 4), cell_a())
                    .unwrap_err();
                assert_eq!(err, ErrorOutOfBoundsAxises::Y);
            }

            #[test]
            fn set_cell_checked_xy_out_of_bounds() {
                let mut buf = new_buf(Size::new(4, 4));
                let err = buf
                    .set_cell_checked(Position::new(4, 4), cell_a())
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
                buf.set_cell(pos, cell_a());
                assert_eq!(buf.get_cell(pos), &cell_a());
            }

            #[test]
            #[should_panic]
            fn set_cell_panics_out_of_bounds() {
                let mut buf = new_buf(Size::new(4, 4));
                buf.set_cell(Position::new(10, 10), cell_a());
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
                *buf.get_cell_mut_checked(pos).unwrap() = cell_a();
                assert_eq!(buf.get_cell(pos), &cell_a());
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
                *buf.get_cell_mut(pos) = cell_b();
                assert_eq!(buf.get_cell(pos), &cell_b());
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
                let pos_a = Position::new(0, 0);
                let pos_b = Position::new(3, 3);
                buf.set_cell(pos_a, cell_a());
                buf.set_cell(pos_b, cell_b());
                assert_eq!(buf.get_cell(pos_a), &cell_a());
                assert_eq!(buf.get_cell(pos_b), &cell_b());
            }

            #[test]
            fn adjacent_cells_are_independent() {
                let mut buf = new_buf(Size::new(3, 1));
                buf.set_cell(Position::new(0, 0), cell_a());
                buf.set_cell(Position::new(1, 0), cell_b());
                buf.set_cell(Position::new(2, 0), cell_a());
                assert_eq!(buf.get_cell(Position::new(0, 0)), &cell_a());
                assert_eq!(buf.get_cell(Position::new(1, 0)), &cell_b());
                assert_eq!(buf.get_cell(Position::new(2, 0)), &cell_a());
            }

            // overwrite

            #[test]
            fn overwriting_a_cell_reflects_new_value() {
                let mut buf = new_buf(Size::new(4, 4));
                let pos = Position::new(1, 1);
                buf.set_cell(pos, cell_a());
                buf.set_cell(pos, cell_b());
                assert_eq!(buf.get_cell(pos), &cell_b());
            }

            // fill

            #[test]
            fn fill_sets_every_cell() {
                let size = Size::new(4, 3);
                let mut buf = new_buf(size);
                buf.fill(cell_a());
                for y in 0..size.height {
                    for x in 0..size.width {
                        assert_eq!(
                            buf.get_cell(Position::new(x, y)),
                            &cell_a(),
                            "cell at ({x},{y}) should be cell_a after fill"
                        );
                    }
                }
            }

            // clear

            #[test]
            fn clear_sets_every_cell_to_empty() {
                let size = Size::new(4, 3);
                let mut buf = new_buf(size);
                buf.fill(cell_a());
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
