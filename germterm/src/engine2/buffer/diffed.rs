use crate::{
    cell::Cell,
    engine2::{
        Position,
        buffer::{Drawer, ResizableBuffer},
        draw::Size,
    },
};

use super::{Buffer, DrawCall};

#[derive(Clone, Copy, Debug)]
enum FrameOrder {
    CurrentOld = 0,
    OldCurrent = 1,
}

/// A buffer that wraps two buffers of the same type and produces draw calls
/// only for cells that differ between the current and previous frame.
///
/// On each call to [`Drawer::draw`], [`DiffedBuffers`] compares the active
/// (current) buffer against the inactive (previous) buffer and emits a
/// [`DrawCall`] only for positions where the cell has changed. The two
/// internal buffers are then swapped so the old frame becomes the baseline
/// for the next comparison.
///
/// This makes it suitable as an adapter around any existing [`Buffer`]
/// implementation when you want to minimise redundant terminal writes.
pub struct DiffedBuffers<Buf: Buffer> {
    size: Size,
    cells: [Buf; 2],
    frame_order: FrameOrder,
}

impl<Buf: Buffer> DiffedBuffers<Buf> {
    /// Creates a new `DiffedBuffers` with the given size and two pre-constructed
    /// inner buffers.
    ///
    /// The provided buffers must be correctly sized.
    pub fn new(size: Size, buf1: Buf, buf2: Buf) -> Self {
        Self {
            size,
            cells: [buf1, buf2],
            frame_order: FrameOrder::CurrentOld,
        }
    }

    /// Swaps the current and previous frame buffers.
    ///
    /// After swapping, writes go to what was previously the old buffer and
    /// the old buffer becomes the new baseline for diffing.
    pub fn swap_frames(&mut self) {
        self.frame_order = match self.frame_order {
            FrameOrder::CurrentOld => FrameOrder::OldCurrent,
            FrameOrder::OldCurrent => FrameOrder::CurrentOld,
        };
    }
}

impl<Buf: Buffer> Buffer for DiffedBuffers<Buf> {
    fn size(&self) -> Size {
        self.size
    }

    fn set_cell_checked(
        &mut self,
        pos: Position,
        cell: Cell,
    ) -> Result<(), super::ErrorOutOfBoundsAxises> {
        let idx = self.frame_order as usize;
        self.cells[idx].set_cell_checked(pos, cell)
    }

    fn get_cell_checked(&self, pos: Position) -> Result<&Cell, super::ErrorOutOfBoundsAxises> {
        let idx = self.frame_order as usize;
        self.cells[idx].get_cell_checked(pos)
    }

    fn get_cell_mut_checked(
        &mut self,
        pos: Position,
    ) -> Result<&mut Cell, super::ErrorOutOfBoundsAxises> {
        let idx = self.frame_order as usize;
        self.cells[idx].get_cell_mut_checked(pos)
    }

    fn fill(&mut self, cell: Cell) {
        let idx = self.frame_order as usize;
        self.cells[idx].fill(cell);
    }

    fn start_frame(&mut self) {
        let idx = self.frame_order as usize;
        self.cells[idx].start_frame();
    }

    fn end_frame(&mut self) {
        let idx = self.frame_order as usize;
        self.cells[idx].end_frame();
        self.swap_frames();
    }
}

impl<Buf: ResizableBuffer> ResizableBuffer for DiffedBuffers<Buf> {
    fn resize(&mut self, size: Size) {
        self.size = size;
        self.cells.iter_mut().for_each(|b| b.resize(size))
    }
}

impl<Buf: Buffer> Drawer for DiffedBuffers<Buf> {
    fn draw(&mut self) -> impl Iterator<Item = DrawCall<'_>> {
        let width = self.size.width;
        let height = self.size.height;
        let order = self.frame_order as usize;

        let current_buf = &self.cells[order];
        let old_buf = &self.cells[1 - order];

        (0..height).flat_map(move |y| {
            (0..width).filter_map(move |x| {
                let pos = Position { x, y };
                let current_cell = current_buf.get_cell(pos);
                let old_cell = old_buf.get_cell(pos);

                if current_cell != old_cell {
                    Some(DrawCall {
                        cell: current_cell,
                        pos,
                    })
                } else {
                    None
                }
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::DiffedBuffers;
    use crate::{
        buffer_resizing_tests, buffer_tests, drawer_diffed_buffer_tests,
        engine2::{buffer::flat::FlatBuffer, draw::Size},
    };

    type TestBuffer = DiffedBuffers<FlatBuffer>;

    fn new_tb(sz: Size) -> TestBuffer {
        TestBuffer::new(sz, FlatBuffer::new(sz), FlatBuffer::new(sz))
    }
    buffer_tests! {
        buffer_tests,
        super::new_tb,
        TestBuffer
    }

    buffer_resizing_tests! {
        resizing_tests,
        super::new_tb,
        TestBuffer
    }

    drawer_diffed_buffer_tests! {
        diffed_tests,
        super::new_tb,
        TestBuffer
    }
}
