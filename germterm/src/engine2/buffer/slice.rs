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
        self.inner.set_cell_checked(translated, cell)
    }

    fn get_cell_checked(&self, pos: Position) -> Result<&Cell, super::ErrorOutOfBoundsAxises> {
        let translated = self.translate(pos)?;
        self.inner.get_cell_checked(translated)
    }

    fn get_cell_mut_checked(
        &mut self,
        pos: Position,
    ) -> Result<&mut Cell, super::ErrorOutOfBoundsAxises> {
        let translated = self.translate(pos)?;
        self.inner.get_cell_mut_checked(translated)
    }
}

// This section is kind of hacky.
//
// Rather than write a ton of tests that need to be updated with the buffer test macro we use a new
// type a bit of unsafe to reduce duplication.
#[cfg(test)]
mod tests {
    use crate::{
        buffer_tests,
        cell::Cell,
        engine2::{
            buffer::{Buffer, paired::PairedBuffer, slice::SubBuffer},
            draw::{Position, Rect, Size},
        },
    };

    pub const SCALE: u16 = 2;
    pub const TEST_CELL: Cell = Cell {
        ch: '森',
        ..Cell::EMPTY
    };

    struct OwnedSubBuffer(SubBuffer<'static, PairedBuffer>);

    impl OwnedSubBuffer {
        fn new(sz: Size) -> Self {
            let nsz = sz.scale(SCALE);
            let inner = Box::leak(Box::new(PairedBuffer::new(nsz)));
            inner.fill(TEST_CELL);

            SubBuffer::new(inner, Rect::new(Position::ZERO, sz)).clear();
            OwnedSubBuffer(SubBuffer::new(inner, Rect::new(Position::ZERO, sz)))
        }
    }

    impl Drop for OwnedSubBuffer {
        fn drop(&mut self) {
            // Assert that every cell outside the SubBuffer's area (top-left
            // quadrant) is still TEST_CELL, i.e. the SubBuffer never wrote
            // outside its own bounds.
            let inner = unsafe { Box::from_raw(self.0.inner) };
            let sz = inner.size();
            let mid_x = sz.width / SCALE;
            let mid_y = sz.height / SCALE;

            // top-right: x >= mid_x, y < mid_y
            for y in 0..mid_y {
                for x in mid_x..sz.width {
                    let pos = Position::new(x, y);
                    assert_eq!(
                        inner.get_cell(pos),
                        &TEST_CELL,
                        "Mismatch of cell in {pos:?}"
                    );
                }
            }

            // bottom-left: x < mid_x, y >= mid_y
            for y in mid_y..sz.height {
                for x in 0..mid_x {
                    let pos = Position::new(x, y);
                    assert_eq!(
                        inner.get_cell(pos),
                        &TEST_CELL,
                        "Mismatch of cell in {pos:?}"
                    );
                }
            }

            // bottom-right: x >= mid_x, y >= mid_y
            for y in mid_y..sz.height {
                for x in mid_x..sz.width {
                    let pos = Position::new(x, y);
                    assert_eq!(
                        inner.get_cell(pos),
                        &TEST_CELL,
                        "Mismatch of cell in {pos:?}"
                    );
                }
            }
        }
    }

    impl Buffer for OwnedSubBuffer {
        fn set_cell(&mut self, pos: Position, cell: crate::cell::Cell) {
            self.0
                .set_cell_checked(pos, cell)
                .expect("out of bounds set_cell")
        }

        fn get_cell(&self, pos: Position) -> &crate::cell::Cell {
            self.0
                .get_cell_checked(pos)
                .expect("out of bounds get_cell")
        }

        fn get_cell_mut(&mut self, pos: Position) -> &mut crate::cell::Cell {
            self.0
                .get_cell_mut_checked(pos)
                .expect("out of bounds get_cell_mut")
        }

        fn fill(&mut self, cell: crate::cell::Cell) {
            self.0.fill(cell);
        }

        fn clear(&mut self) {
            self.0.fill(crate::cell::Cell::EMPTY);
        }

        fn start_frame(&mut self) {
            self.0.start_frame();
        }

        fn end_frame(&mut self) {
            self.0.end_frame();
        }

        fn size(&self) -> Size {
            self.0.size()
        }

        fn set_cell_checked(
            &mut self,
            pos: Position,
            cell: crate::cell::Cell,
        ) -> Result<(), crate::engine2::buffer::ErrorOutOfBoundsAxises> {
            self.0.set_cell_checked(pos, cell)
        }

        fn get_cell_checked(
            &self,
            pos: Position,
        ) -> Result<&crate::cell::Cell, crate::engine2::buffer::ErrorOutOfBoundsAxises> {
            self.0.get_cell_checked(pos)
        }

        fn get_cell_mut_checked(
            &mut self,
            pos: Position,
        ) -> Result<&mut crate::cell::Cell, crate::engine2::buffer::ErrorOutOfBoundsAxises>
        {
            self.0.get_cell_mut_checked(pos)
        }
    }

    buffer_tests! {
        buffer_tests,
        OwnedSubBuffer::new,
        OwnedSubBuffer
    }
}
