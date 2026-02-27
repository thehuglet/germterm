use std::cmp::Ordering;

use super::{Buffer, DrawCall, Drawer};
#[cfg(test)]
use crate::{buffer_resizing_tests, buffer_tests, drawer_diffed_buffer_tests};
use crate::{
    cell::Cell,
    core::{Position, buffer::ResizableBuffer, draw::Size},
};

#[derive(Clone, Copy, Debug)]
enum FrameOrder {
    CurrentOld = 0,
    OldCurrent = 1,
}

/// The default buffer implementation provided by the library.
///
/// `PairedBuffer` stores two frames of [`Cell`]s in a single flat `Vec`,
/// interleaved at the cell level so that both the current and previous frame
/// for any given position sit adjacent in memory. The active frame is
/// selected by a `FrameOrder` flag that is toggled on each [`swap_frames`]
/// call.
///
/// On each call to [`Drawer::draw`], the current and previous cells at every
/// position are compared and a [`DrawCall`] is emitted only for positions
/// where they differ, minimising redundant terminal writes. After diffing,
/// the frames are swapped automatically.
///
/// [`swap_frames`]: PairedBuffer::swap_frames
pub struct PairedBuffer {
    size: Size,
    frames: Vec<[Cell; 2]>,
    order: FrameOrder,
}

impl PairedBuffer {
    /// Creates a new `PairedBuffer` with the given size.
    ///
    /// Both buffers are initialised to [`Cell::EMPTY`].
    pub fn new(size: Size) -> Self {
        Self {
            size,
            frames: vec![[Cell::EMPTY; 2]; size.area() as usize],
            order: FrameOrder::CurrentOld,
        }
    }

    #[inline(always)]
    fn index_current(&self) -> usize {
        self.order as usize
    }

    #[inline(always)]
    fn index_old(&self) -> usize {
        1 - self.order as usize
    }

    /// Swaps the current and previous frame buffers.
    ///
    /// After swapping, writes and reads target what was previously the old
    /// frame, and the old frame becomes the new baseline for diffing.
    pub fn swap_frames(&mut self) {
        self.order = match self.order {
            FrameOrder::CurrentOld => FrameOrder::OldCurrent,
            FrameOrder::OldCurrent => FrameOrder::CurrentOld,
        };
    }
}

#[cold]
#[inline(never)]
fn cold() {}

impl Buffer for PairedBuffer {
    fn size(&self) -> Size {
        self.size
    }

    fn set_cell_checked(
        &mut self,
        pos: Position,
        cell: Cell,
    ) -> Result<(), super::ErrorOutOfBoundsAxises> {
        if let err @ Err(_) = self.size.contains(pos) {
            cold();
            return err;
        }

        let cur = self.index_current();
        self.frames[pos.to_index(self.size.width)][cur] = cell;
        Ok(())
    }

    fn get_cell_checked(&self, pos: Position) -> Result<&Cell, super::ErrorOutOfBoundsAxises> {
        if let Err(err) = self.size.contains(pos) {
            cold();
            return Err(err);
        }
        Ok(&self.frames[pos.to_index(self.size.width)][self.index_current()])
    }

    fn get_cell_mut_checked(
        &mut self,
        pos: Position,
    ) -> Result<&mut Cell, super::ErrorOutOfBoundsAxises> {
        if let Err(err) = self.size.contains(pos) {
            cold();
            return Err(err);
        }
        let cur = self.index_current();
        Ok(&mut self.frames[pos.to_index(self.size.width)][cur])
    }

    fn fill(&mut self, cell: Cell) {
        let cur = self.index_current();
        for frame in &mut self.frames {
            frame[cur] = cell;
        }
    }

    fn start_frame(&mut self) {
        self.clear();
    }

    fn end_frame(&mut self) {
        self.swap_frames();
    }
}

impl ResizableBuffer for PairedBuffer {
    fn resize(&mut self, size: Size) {
        let w_new = size.width;
        let w_old = self.size.width;
        let old_total_size = self.size.area();
        let new_total_size = size.area();

        // If growing reserve the needed space in bulk
        if new_total_size > old_total_size {
            self.frames
                .reserve((new_total_size - old_total_size) as usize);
        }

        match w_old.cmp(&w_new) {
            // Grow case
            Ordering::Less => {}
            // Shrink case
            Ordering::Greater => todo!(),
            Ordering::Equal => {}
        }

        todo!();
    }
}

impl Drawer for PairedBuffer {
    fn draw(&mut self) -> impl Iterator<Item = DrawCall<'_>> {
        let cur_idx = self.index_current();
        let width = self.size.width as usize;

        let s: &Self = self;
        assert_eq!(self.size.area(), self.frames.len() as u32);
        (0..(self.frames.len())).filter_map(move |i| {
            let lr @ [l, r] = unsafe { s.frames.get_unchecked(i) };
            if l != r {
                let x = (i % width) as u16;
                let y = (i / width) as u16;
                Some(DrawCall {
                    pos: Position { x, y },
                    cell: unsafe { lr.get_unchecked(cur_idx) },
                })
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
buffer_tests! {
    buffer_tests,
    PairedBuffer::new,
    PairedBuffer
}
#[cfg(test)]
drawer_diffed_buffer_tests! {
    diffed_tests,
    PairedBuffer::new,
    PairedBuffer
}
#[cfg(test)]
buffer_resizing_tests! {
    resizing_tests,
    PairedBuffer::new,
    PairedBuffer
}
