use crate::{
    cell::Cell,
    engine2::{
        buffer::{Buffer, ErrorOutOfBoundsAxises},
        draw::{Position, Rect, Size},
    },
};

/// A borrowed rectangular view into a parent [`Buffer`].
///
/// `SubBuffer` translates all positions by a [`SubBuffer::origin`] offset
/// before forwarding them to the parent buffer, presenting a sub-region as if
/// it were an independent buffer starting at `(0, 0)`.
///
/// Implements [`Buffer`] itself, so it can be passed directly into
/// [`FrameContext`](crate::engine2::widget::FrameContext) or any other
/// context expecting a buffer without the callee knowing it operates on
/// a sub-region.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SubBuffer<'a, Buf: Buffer + ?Sized> {
    inner: &'a mut Buf,
    // Never make this public as we never want a widget to grow its area.
    area: Rect,
}

impl<'a, Buf: Buffer + ?Sized> SubBuffer<'a, Buf> {
    /// Creates a new `SubBuffer` viewing into `inner` at the given
    /// `origin` with the given drawable `size`.
    pub fn new(inner: &'a mut Buf, area: Rect) -> Self {
        Self { inner, area }
    }

    /// The top-left corner of this subbuffer in the parent buffer's
    /// coordinate space.
    pub fn origin(&self) -> Position {
        self.area.origin
    }

    /// The drawable dimensions of this subbuffer.
    pub fn size(&self) -> Size {
        self.area.size
    }

    /// Translates a local position into the parent buffer's coordinate space.
    ///
    /// Returns [`ErrorOutOfBoundsAxises`] if `pos` is outside this
    /// sub-buffer's own bounds (i.e. `pos` is not within `self.size`).
    #[inline(always)]
    fn translate(&self, pos: Position) -> Result<Position, ErrorOutOfBoundsAxises> {
        self.area
            .size
            .contains(pos)
            .map(|_| Position::new(self.origin().x + pos.x, self.origin().y + pos.y))
    }

    /// Shrink the buffer from the left side.
    #[inline(always)]
    pub fn shrink_left(&mut self, by: u16) {
        self.area.origin.x = self.area.origin.x.saturating_add(by);
        self.area.size.width = self.area.size.width.saturating_sub(by);
    }

    /// Shrink the buffer from the right side.
    #[inline(always)]
    pub fn shrink_right(&mut self, by: u16) {
        self.area.size.width = self.area.size.width.saturating_sub(by);
    }

    /// Shrink the buffer horizontally.
    ///
    /// The buffer is shrunk at the left and right side.
    pub fn shrink_width(&mut self, by: u16) {
        // When the shrink amount would exceed the width, collapse to zero width
        // whilst keeping the origin centered rather than biasing to one side.
        //
        // A zero-size buffer produces no draws, so the exact behavior rarely
        // matters for rendering. However callers may read `origin` and `size`
        // directly (e.g. to compute layout distances), and a symmetrically
        // centered origin is a more predictable result than an arbitrarily
        // left-biased one.
        if self.size().width <= by * 2 {
            self.area.origin.x += self.size().width / 2;
            self.area.size.width = 0;
        } else {
            self.shrink_left(by);
            self.shrink_right(by);
        }
    }

    /// Shrink the buffer from the top side.
    #[inline(always)]
    pub fn shrink_top(&mut self, by: u16) {
        self.area.origin.y = self.origin().y.saturating_add(by);
        self.area.size.height = self.size().height.saturating_sub(by);
    }

    /// Shrink the buffer from the bottom side.
    #[inline(always)]
    pub fn shrink_bottom(&mut self, by: u16) {
        self.area.size.height = self.size().height.saturating_sub(by);
    }

    /// Shrink the buffer vertically.
    ///
    /// The buffer is shrunk at the top and bottom side.
    pub fn shrink_height(&mut self, by: u16) {
        // see comments in `Self::shrink_width` for why we do this
        if self.size().height <= by * 2 {
            self.area.origin.y += self.size().height / 2;
            self.area.size.height = 0;
        } else {
            self.shrink_top(by);
            self.shrink_bottom(by);
        }
    }

    /// Returns a new `SubBuffer` viewing a sub-region of this one.
    ///
    /// The `origin` and `size` are relative to this `SubBuffer`.
    pub fn sub_region(&mut self, origin: Position, size: Size) -> SubBuffer<'_, Self> {
        SubBuffer::new(
            self,
            Rect::from_xywh(origin.x, origin.y, size.width, size.height),
        )
    }

    /// Returns a new `SubBuffer` representing the inner area after applying a margin.
    pub fn inner(&mut self, margin: u16) -> SubBuffer<'_, Self> {
        let mut child = self.sub_region(Position::ZERO, self.area.size);
        child.shrink_width(margin);
        child.shrink_height(margin);
        child
    }
}

impl<Buf: Buffer + ?Sized> Buffer for SubBuffer<'_, Buf> {
    fn size(&self) -> Size {
        self.area.size
    }

    fn set_cell_checked(
        &mut self,
        pos: Position,
        cell: Cell,
    ) -> Result<(), super::ErrorOutOfBoundsAxises> {
        let translated = self.translate(pos)?;
        self.inner.set_cell(translated, cell);
        Ok(())
    }

    fn get_cell_checked(&self, pos: Position) -> Result<&Cell, super::ErrorOutOfBoundsAxises> {
        let translated = self.translate(pos)?;
        Ok(self.inner.get_cell(translated))
    }

    fn get_cell_mut_checked(
        &mut self,
        pos: Position,
    ) -> Result<&mut Cell, super::ErrorOutOfBoundsAxises> {
        let translated = self.translate(pos)?;
        Ok(self.inner.get_cell_mut(translated))
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
            let mut sub_buffer = SubBuffer::new(&mut buf, Rect::from_xywh(3, 4, 5, 5));
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
            let mut sub_buffer = SubBuffer::new(&mut buf, Rect::from_xywh(5, 10, 10, 5));
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

        let sub_buffer = SubBuffer::new(&mut buf, Rect::from_xywh(4, 5, 5, 5));
        // Local (2, 2) maps to parent (6, 7)
        assert_eq!(sub_buffer.get_cell(Position::new(2, 2)).ch, 'Z');
    }

    #[test]
    fn get_cell_mut_modifies_parent() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));

        {
            let mut sub_buffer = SubBuffer::new(&mut buf, Rect::from_xywh(1, 1, 5, 5));
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
            let mut left = SubBuffer::new(&mut buf, Rect::from_xywh(0, 0, 10, 10));
            left.set_cell(Position::new(5, 5), cell_a);
        }
        {
            let mut right = SubBuffer::new(&mut buf, Rect::from_xywh(10, 0, 10, 10));
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
            let mut outer = SubBuffer::new(&mut buf, Rect::from_xywh(2, 3, 15, 15));
            {
                let mut inner = SubBuffer::new(&mut outer, Rect::from_xywh(4, 5, 5, 5));
                inner.set_cell(Position::new(1, 1), cell);
            }
        }

        // (2+4+1, 3+5+1) = (7, 9)
        assert_eq!(buf.get_cell(Position::new(7, 9)).ch, 'N');
    }

    #[test]
    fn checked_write_uses_sub_buffer_size() {
        let mut buf = PairedBuffer::new(Size::new(20, 20));
        let mut sub_buffer = SubBuffer::new(&mut buf, Rect::from_xywh(0, 0, 5, 5));

        // Within sub_buffer bounds
        assert!(
            sub_buffer
                .set_cell_checked(Position::new(4, 4), Cell::EMPTY)
                .is_ok()
        );

        // Outside sub_buffer bounds
        assert!(
            sub_buffer
                .set_cell_checked(Position::new(5, 0), Cell::EMPTY)
                .is_err()
        );
        assert!(
            sub_buffer
                .set_cell_checked(Position::new(0, 5), Cell::EMPTY)
                .is_err()
        );
    }

    #[test]
    fn size_and_origin_accessors() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let sub_buffer = SubBuffer::new(&mut buf, Rect::from_xywh(3, 4, 5, 6));

        assert_eq!(sub_buffer.origin().x, 3);
        assert_eq!(sub_buffer.origin().y, 4);
        assert_eq!(sub_buffer.size().width, 5);
        assert_eq!(sub_buffer.size().height, 6);
    }

    #[test]
    fn shrink_left_adjusts_origin_and_size() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub_buffer = SubBuffer::new(&mut buf, Rect::from_xywh(2, 2, 8, 8));
        sub_buffer.shrink_left(2);
        assert_eq!(sub_buffer.origin(), Position::new(4, 2));
        assert_eq!(sub_buffer.size(), Size::new(6, 8));
    }

    #[test]
    fn shrink_right_adjusts_size() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub_buffer = SubBuffer::new(&mut buf, Rect::from_xywh(2, 2, 8, 8));
        sub_buffer.shrink_right(2);
        assert_eq!(sub_buffer.origin(), Position::new(2, 2));
        assert_eq!(sub_buffer.size(), Size::new(6, 8));
    }

    #[test]
    fn shrink_width_adjusts_origin_and_size() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub_buffer = SubBuffer::new(&mut buf, Rect::from_xywh(2, 2, 8, 8));
        sub_buffer.shrink_width(2);
        assert_eq!(sub_buffer.origin(), Position::new(4, 2));
        assert_eq!(sub_buffer.size(), Size::new(4, 8));
    }

    #[test]
    fn shrink_top_adjusts_origin_and_size() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub_buffer = SubBuffer::new(&mut buf, Rect::from_xywh(2, 2, 8, 8));
        sub_buffer.shrink_top(2);
        assert_eq!(sub_buffer.origin(), Position::new(2, 4));
        assert_eq!(sub_buffer.size(), Size::new(8, 6));
    }

    #[test]
    fn shrink_bottom_adjusts_size() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub_buffer = SubBuffer::new(&mut buf, Rect::from_xywh(2, 2, 8, 8));
        sub_buffer.shrink_bottom(2);
        assert_eq!(sub_buffer.origin(), Position::new(2, 2));
        assert_eq!(sub_buffer.size(), Size::new(8, 6));
    }

    #[test]
    fn shrink_height_adjusts_origin_and_size() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub_buffer = SubBuffer::new(&mut buf, Rect::from_xywh(2, 2, 8, 8));
        sub_buffer.shrink_height(2);
        assert_eq!(sub_buffer.origin(), Position::new(2, 4));
        assert_eq!(sub_buffer.size(), Size::new(8, 4));
    }

    #[test]
    #[should_panic]
    fn set_cell_panics_on_out_of_bounds() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub_buffer = SubBuffer::new(&mut buf, Rect::from_xywh(2, 2, 5, 5));
        sub_buffer.set_cell(Position::new(5, 0), Cell::EMPTY);
    }

    #[test]
    #[should_panic]
    fn get_cell_panics_on_out_of_bounds() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let sub_buffer = SubBuffer::new(&mut buf, Rect::from_xywh(2, 2, 5, 5));
        sub_buffer.get_cell(Position::new(0, 5));
    }

    #[test]
    #[should_panic]
    fn get_cell_mut_panics_on_out_of_bounds() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub_buffer = SubBuffer::new(&mut buf, Rect::from_xywh(2, 2, 5, 5));
        sub_buffer.get_cell_mut(Position::new(5, 5));
    }

    #[test]
    fn fill_and_clear() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        {
            let mut sub = SubBuffer::new(&mut buf, Rect::from_xywh(2, 2, 3, 3));

            let mut cell = Cell::EMPTY;
            cell.ch = 'F';
            sub.fill(cell);

            // sub is still alive here, so we must access through sub or wait until it's dropped.
            assert_eq!(sub.get_cell(Position::ZERO).ch, 'F');
        }

        assert_eq!(buf.get_cell(Position::new(2, 2)).ch, 'F');
        assert_eq!(buf.get_cell(Position::new(4, 4)).ch, 'F');
        assert_eq!(buf.get_cell(Position::new(1, 1)).ch, ' ');

        {
            let mut sub = SubBuffer::new(&mut buf, Rect::from_xywh(2, 2, 3, 3));
            sub.clear();
        }
        assert_eq!(buf.get_cell(Position::new(2, 2)).ch, ' ');
    }

    #[test]
    fn inner_margin() {
        let mut buf = PairedBuffer::new(Size::new(10, 10));
        let mut sub = SubBuffer::new(&mut buf, Rect::from_xywh(0, 0, 10, 10));

        {
            let mut inner = sub.inner(2);
            assert_eq!(inner.size(), Size::new(6, 6));
            assert_eq!(inner.origin(), Position::new(2, 2));

            let mut cell = Cell::EMPTY;
            cell.ch = 'I';
            inner.set_cell(Position::ZERO, cell);
        }

        // Inner is dropped, so we can access sub or buf.
        assert_eq!(sub.get_cell(Position::new(2, 2)).ch, 'I');
    }
}
