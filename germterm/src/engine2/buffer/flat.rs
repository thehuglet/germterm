use std::{cmp::Ordering, ptr};

use super::{Buffer, DrawCall, Drawer, ErrorOutOfBoundsAxises, ResizableBuffer};
use crate::{
    cell::Cell,
    engine2::{Position, draw::Size},
};

/// A flat buffer that stores every cell in a single `Vec<Cell>` in row-major
/// order.
///
/// Unlike [`super::paired::PairedBuffer`] this buffer does **not** diff frames:
/// [`Drawer::draw`] always emits every cell on every call.
///
/// If you want a simple and fast `Buffer` that offers diffing, [`DiffedBuffers<FlatBuffer>`] can be
/// used instead.
///
/// [`DiffedBuffers<FlatBuffer>`]: super::diffed::DiffedBuffers
pub struct FlatBuffer {
    size: Size,
    cells: Vec<Cell>,
}

impl FlatBuffer {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            cells: vec![Cell::EMPTY; size.area() as usize],
        }
    }
}

impl Buffer for FlatBuffer {
    fn size(&self) -> Size {
        self.size
    }

    fn set_cell_checked(
        &mut self,
        pos: Position,
        cell: Cell,
    ) -> Result<(), ErrorOutOfBoundsAxises> {
        self.size.contains(pos)?;
        self.cells[pos.to_index(self.size.width)] = cell;
        Ok(())
    }

    fn get_cell_checked(&self, pos: Position) -> Result<&Cell, ErrorOutOfBoundsAxises> {
        self.size.contains(pos)?;
        Ok(&self.cells[pos.to_index(self.size.width)])
    }

    fn get_cell_mut_checked(&mut self, pos: Position) -> Result<&mut Cell, ErrorOutOfBoundsAxises> {
        self.size.contains(pos)?;
        Ok(&mut self.cells[pos.to_index(self.size.width)])
    }

    fn fill(&mut self, cell: Cell) {
        self.cells.fill(cell);
    }

    fn start_frame(&mut self) {
        self.clear();
    }
}

impl Drawer for FlatBuffer {
    fn draw(&mut self) -> impl Iterator<Item = DrawCall<'_>> {
        let width = self.size.width;
        let height = self.size.height;
        let cells = &self.cells;

        (0..height).flat_map(move |y| {
            (0..width).map(move |x| {
                let pos = Position { x, y };
                DrawCall {
                    pos,
                    cell: &cells[pos.to_index(width)],
                }
            })
        })
    }
}

impl ResizableBuffer for FlatBuffer {
    fn resize(&mut self, size: Size) {
        // NOTE: The implementation is intended to perform the minimum number of copies needed to
        // resize. This is technically possible to do with safe Rust with just a small overhead but
        // the code becomes extremely verbose.
        //
        // This implementation is short simple and fast.
        if self.size == size {
            return;
        }

        let old_w = self.size.width as usize;
        let new_w = size.width as usize;
        let old_h = self.size.height as usize;

        self.cells
            .reserve_exact((size.area() as usize).saturating_sub(self.cells.len()));

        // NOTE: I have no idea why Cell would be unpin but if it somehow does this gives a compile error
        // rather than UB.
        #[allow(unused)]
        fn is_copy<T: Unpin>(arg: &T) {
            let _ = || is_copy::<Cell>(&Cell::EMPTY);
        }

        // Width
        match old_w.cmp(&new_w) {
            // Grow case
            Ordering::Less => {
                let new_len = size.area() as usize;
                let grow_by = new_w - old_w;

                // SAFETY:
                // - Enough capacity has been allocated.
                // - New gaps (uninit Cell's) have been set to `Cell::EMPTY`
                // - `Vec::set_len` is called last, after all writes.
                unsafe {
                    let base = self.cells.as_mut_ptr();

                    // Scatter rows from last to second. Row 0 is already at
                    // offset 0 and doesn't need to move.
                    for y in (1..old_h).rev() {
                        let src = base.add(y * old_w);
                        let dst = base.add(y * new_w);
                        ptr::copy(src, dst, old_w);

                        // Fill the new columns at the end of this row.
                        std::slice::from_raw_parts_mut(dst.add(old_w), grow_by).fill(Cell::EMPTY);
                    }

                    // Row 0: fill trailing gap last, after all rows have been
                    // scattered, so that row 1's old data is not overwritten.
                    if old_h > 0 {
                        std::slice::from_raw_parts_mut(base.add(old_w), grow_by).fill(Cell::EMPTY);
                    }

                    self.cells.set_len(new_len);
                }
            }
            // Shrink case
            Ordering::Greater => unsafe {
                let base = self.cells.as_mut_ptr();
                for y in 1..old_h {
                    ptr::copy(base.add(y * old_w), base.add(y * new_w), new_w);
                }
            },
            Ordering::Equal => {}
        }

        // Height
        self.cells.resize(size.area() as usize, Cell::EMPTY);

        self.size = size;
    }
}

#[cfg(test)]
mod tests {

    use super::FlatBuffer;
    use crate::{buffer_resizing_tests, buffer_tests, drawer_buffer_tests};

    buffer_tests! {
        buffer_tests,
        FlatBuffer::new,
        FlatBuffer
    }

    drawer_buffer_tests! {
        drawer_tests,
        FlatBuffer::new,
        FlatBuffer
    }

    buffer_resizing_tests! {
        resizing_tests,
        FlatBuffer::new,
        FlatBuffer
    }
}
