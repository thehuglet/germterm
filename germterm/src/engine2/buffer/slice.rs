use crate::{
    cell::Cell,
    engine2::draw::{Position, Size},
};

use super::Buffer;

/// A borrowed rectangular view into a parent [`Buffer`].
///
/// `SubBuffer` translates all positions by an [`origin`](SubBuffer::origin)
/// offset before forwarding them to the parent buffer, presenting a
/// sub-region as if it were an independent buffer starting at `(0, 0)`.
///
/// Implements [`Buffer`] itself, so it can be passed directly into
/// [`FrameContext`](crate::engine2::widget::FrameContext) or any other
/// context expecting a buffer without the callee knowing it operates on
/// a sub-region.
///
/// # Panics
///
/// The unchecked [`Buffer`] methods (`set_cell`, `get_cell`, `get_cell_mut`)
/// follow the same contract as the parent buffer: if the translated position
/// falls outside the parent's bounds the parent is free to panic. Use the
/// checked variants with [`size`](SubBuffer::size) to guarantee bounds safety.
pub struct SubBuffer<'a, Buf: Buffer + ?Sized> {
    inner: &'a mut Buf,
    origin: Position,
    size: Size,
}

impl<'a, Buf: Buffer + ?Sized> SubBuffer<'a, Buf> {
    /// Creates a new `SubBuffer` viewing into `inner` at the given
    /// `origin` with the given drawable `size`.
    pub fn new(inner: &'a mut Buf, origin: Position, size: Size) -> Self {
        Self {
            inner,
            origin,
            size,
        }
    }

    /// The top-left corner of this subbuffer in the parent buffer's
    /// coordinate space.
    pub fn origin(&self) -> Position {
        self.origin
    }

    /// The drawable dimensions of this subbuffer.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Translates a local position into the parent buffer's coordinate space.
    #[inline(always)]
    fn translate(&self, pos: Position) -> Position {
        Position::new(self.origin.x + pos.x, self.origin.y + pos.y)
    }

    #[inline(always)]
    pub fn shrink_left(&mut self, by: u16) {
        self.origin.x = self.origin.x.saturating_add(by);
        self.size.width = self.size.width.saturating_sub(by);
    }

    #[inline(always)]
    pub fn shrink_right(&mut self, by: u16) {
        self.size.width = self.size.width.saturating_sub(by);
    }

    pub fn shrink_width(&mut self, by: u16) {
        self.shrink_left(by);
        self.shrink_right(by);
    }

    #[inline(always)]
    pub fn shrink_top(&mut self, by: u16) {
        self.origin.y = self.origin.y.saturating_add(by);
        self.size.height = self.size.height.saturating_sub(by);
    }

    #[inline(always)]
    pub fn shrink_bottom(&mut self, by: u16) {
        self.size.height = self.size.height.saturating_sub(by);
    }

    pub fn shrink_height(&mut self, by: u16) {
        self.shrink_top(by);
        self.shrink_bottom(by);
    }
}

impl<Buf: Buffer + ?Sized> Buffer for SubBuffer<'_, Buf> {
    fn set_cell(&mut self, pos: Position, cell: Cell) {
        if !self.size.is_within(pos) {
            panic!("out of bounds set_cell for subbuffer of");
        }
        self.inner.set_cell(self.translate(pos), cell);
    }

    fn get_cell(&self, pos: Position) -> &Cell {
        if !self.size.is_within(pos) {
            panic!("out of bounds get_cell for subbuffer of");
        }
        self.inner.get_cell(self.translate(pos))
    }

    fn get_cell_mut(&mut self, pos: Position) -> &mut Cell {
        if !self.size.is_within(pos) {
            panic!("out of bounds get_cell_mut for subbuffer of");
        }
        self.inner.get_cell_mut(self.translate(pos))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine2::buffer::paired::PairedBuffer;

    #[test]
    fn write_at_origin_translates_to_parent() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut cell = Cell::EMPTY;
        cell.ch = 'X';

        {
            let mut sub_buffer = SubBuffer::new(&mut buf, Position::new(3, 4), Size::new(5, 5));
            sub_buffer.set_cell(Position::new(0, 0), cell);
        }

        assert_eq!(buf.get_cell(Position::new(3, 4)).ch, 'X');
    }

    #[test]
    fn write_with_offset_translates_correctly() {
        let mut buf = PairedBuffer::new(Size::new(20, 20));
        let mut cell = Cell::EMPTY;
        cell.ch = 'A';

        {
            let mut sub_buffer = SubBuffer::new(&mut buf, Position::new(5, 10), Size::new(10, 5));
            sub_buffer.set_cell(Position::new(2, 3), cell);
        }

        // (5+2, 10+3) = (7, 13)
        assert_eq!(buf.get_cell(Position::new(7, 13)).ch, 'A');
        // Original local position should be untouched
        assert_eq!(buf.get_cell(Position::new(2, 3)).ch, ' ');
    }

    #[test]
    fn read_through_sub_buffer_reads_parent() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut cell = Cell::EMPTY;
        cell.ch = 'Z';

        buf.set_cell(Position::new(6, 7), cell);

        let sub_buffer = SubBuffer::new(&mut buf, Position::new(4, 5), Size::new(5, 5));
        // Local (2, 2) maps to parent (6, 7)
        assert_eq!(sub_buffer.get_cell(Position::new(2, 2)).ch, 'Z');
    }

    #[test]
    fn get_cell_mut_modifies_parent() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));

        {
            let mut sub_buffer = SubBuffer::new(&mut buf, Position::new(1, 1), Size::new(5, 5));
            let cell = sub_buffer.get_cell_mut(Position::new(0, 0));
            cell.ch = 'M';
        }

        assert_eq!(buf.get_cell(Position::new(1, 1)).ch, 'M');
    }

    #[test]
    fn sequential_sub_buffers_write_to_different_regions() {
        let mut buf = PairedBuffer::new(Size::new(20, 10));
        let mut cell_a = Cell::EMPTY;
        cell_a.ch = 'A';
        let mut cell_b = Cell::EMPTY;
        cell_b.ch = 'B';

        {
            let mut left = SubBuffer::new(&mut buf, Position::new(0, 0), Size::new(10, 10));
            left.set_cell(Position::new(5, 5), cell_a);
        }
        {
            let mut right = SubBuffer::new(&mut buf, Position::new(10, 0), Size::new(10, 10));
            right.set_cell(Position::new(5, 5), cell_b);
        }

        assert_eq!(buf.get_cell(Position::new(5, 5)).ch, 'A');
        assert_eq!(buf.get_cell(Position::new(15, 5)).ch, 'B');
    }

    #[test]
    fn nested_sub_buffers_compound_offsets() {
        let mut buf = PairedBuffer::new(Size::new(20, 20));
        let mut cell = Cell::EMPTY;
        cell.ch = 'N';

        {
            let mut outer = SubBuffer::new(&mut buf, Position::new(2, 3), Size::new(15, 15));
            {
                let mut inner = SubBuffer::new(&mut outer, Position::new(4, 5), Size::new(5, 5));
                inner.set_cell(Position::new(1, 1), cell);
            }
        }

        // (2+4+1, 3+5+1) = (7, 9)
        assert_eq!(buf.get_cell(Position::new(7, 9)).ch, 'N');
    }

    #[test]
    fn checked_write_uses_sub_buffer_size() {
        let mut buf = PairedBuffer::new(Size::new(20, 20));
        let mut sub_buffer = SubBuffer::new(&mut buf, Position::new(0, 0), Size::new(5, 5));
        let sz = sub_buffer.size();

        // Within sub_buffer bounds
        assert!(
            sub_buffer
                .set_cell_checked(sz, Position::new(4, 4), Cell::EMPTY)
                .is_ok()
        );

        // Outside sub_buffer bounds
        assert!(
            sub_buffer
                .set_cell_checked(sz, Position::new(5, 0), Cell::EMPTY)
                .is_err()
        );
        assert!(
            sub_buffer
                .set_cell_checked(sz, Position::new(0, 5), Cell::EMPTY)
                .is_err()
        );
    }

    #[test]
    fn size_and_origin_accessors() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let sub_buffer = SubBuffer::new(&mut buf, Position::new(3, 4), Size::new(5, 6));

        assert_eq!(sub_buffer.origin().x, 3);
        assert_eq!(sub_buffer.origin().y, 4);
        assert_eq!(sub_buffer.size().width, 5);
        assert_eq!(sub_buffer.size().height, 6);
    }

    #[test]
    fn shrink_left_adjusts_origin_and_size() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub_buffer = SubBuffer::new(&mut buf, Position::new(2, 2), Size::new(8, 8));
        sub_buffer.shrink_left(2);
        assert_eq!(sub_buffer.origin(), Position::new(4, 2));
        assert_eq!(sub_buffer.size(), Size::new(6, 8));
    }

    #[test]
    fn shrink_right_adjusts_size() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub_buffer = SubBuffer::new(&mut buf, Position::new(2, 2), Size::new(8, 8));
        sub_buffer.shrink_right(2);
        assert_eq!(sub_buffer.origin(), Position::new(2, 2));
        assert_eq!(sub_buffer.size(), Size::new(6, 8));
    }

    #[test]
    fn shrink_width_adjusts_origin_and_size() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub_buffer = SubBuffer::new(&mut buf, Position::new(2, 2), Size::new(8, 8));
        sub_buffer.shrink_width(2);
        assert_eq!(sub_buffer.origin(), Position::new(4, 2));
        assert_eq!(sub_buffer.size(), Size::new(4, 8));
    }

    #[test]
    fn shrink_top_adjusts_origin_and_size() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub_buffer = SubBuffer::new(&mut buf, Position::new(2, 2), Size::new(8, 8));
        sub_buffer.shrink_top(2);
        assert_eq!(sub_buffer.origin(), Position::new(2, 4));
        assert_eq!(sub_buffer.size(), Size::new(8, 6));
    }

    #[test]
    fn shrink_bottom_adjusts_size() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub_buffer = SubBuffer::new(&mut buf, Position::new(2, 2), Size::new(8, 8));
        sub_buffer.shrink_bottom(2);
        assert_eq!(sub_buffer.origin(), Position::new(2, 2));
        assert_eq!(sub_buffer.size(), Size::new(8, 6));
    }

    #[test]
    fn shrink_height_adjusts_origin_and_size() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub_buffer = SubBuffer::new(&mut buf, Position::new(2, 2), Size::new(8, 8));
        sub_buffer.shrink_height(2);
        assert_eq!(sub_buffer.origin(), Position::new(2, 4));
        assert_eq!(sub_buffer.size(), Size::new(8, 4));
    }

    #[test]
    #[should_panic]
    fn set_cell_panics_on_out_of_bounds() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub_buffer = SubBuffer::new(&mut buf, Position::new(2, 2), Size::new(5, 5));
        sub_buffer.set_cell(Position::new(5, 0), Cell::EMPTY);
    }

    #[test]
    #[should_panic]
    fn get_cell_panics_on_out_of_bounds() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let sub_buffer = SubBuffer::new(&mut buf, Position::new(2, 2), Size::new(5, 5));
        sub_buffer.get_cell(Position::new(0, 5));
    }

    #[test]
    #[should_panic]
    fn get_cell_mut_panics_on_out_of_bounds() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub_buffer = SubBuffer::new(&mut buf, Position::new(2, 2), Size::new(5, 5));
        sub_buffer.get_cell_mut(Position::new(5, 5));
    }
}

