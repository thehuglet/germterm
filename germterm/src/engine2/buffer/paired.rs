use std::cmp::Ordering;

use super::{Buffer, DrawCall, Drawer};
use crate::{
    cell::Cell,
    engine2::{Position, buffer::ResizableBuffer, draw::Size},
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

impl Buffer for PairedBuffer {
    fn set_cell(&mut self, pos: Position, cell: Cell) {
        let cur = self.index_current();
        self.frames[pos.to_index(self.size.width)][cur] = cell;
    }

    fn get_cell_mut(&mut self, pos: Position) -> &mut Cell {
        let cur = self.index_current();
        &mut self.frames[pos.to_index(self.size.width)][cur]
    }

    fn get_cell(&self, pos: Position) -> &Cell {
        &self.frames[pos.to_index(self.size.width)][self.index_current()]
    }

    fn start_frame(&mut self) {
        for x in 0..self.size.width {
            for y in 0..self.size.height {
                let idx = Position { x, y }.to_index(self.size.width);
                let cur = self.index_current();
                self.frames[idx][cur] = Cell::EMPTY;
            }
        }
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
        let width = self.size.width;
        let height = self.size.height;
        let cur_idx = self.index_current();
        let cur_old = self.index_old();

        self.swap_frames();
        let frames = &self.frames;

        (0..height).flat_map(move |y| {
            (0..width).filter_map(move |x| {
                let pos = Position { x, y };
                let idx = pos.to_index(width);

                let current_cell = &frames[idx][cur_idx];
                let old_cell = &frames[idx][cur_old];

                if current_cell != old_cell {
                    Some(DrawCall {
                        pos,
                        cell: current_cell,
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
    use crate::engine2::draw::Size;

    use super::*;

    #[test]
    fn test_new() {
        let sz = Size::new(10, 5);
        let buf = PairedBuffer::new(sz);
        assert_eq!(sz, buf.size);
        assert_eq!(buf.frames.len(), 10 * 5);
    }

    #[test]
    fn test_set_get_cell() {
        let mut buf = PairedBuffer::new(Size::new(10, 5));
        let mut cell = Cell::EMPTY;
        cell.ch = 'X';

        buf.set_cell(Position { x: 1, y: 1 }, cell);
        assert_eq!(buf.get_cell(Position { x: 1, y: 1 }).ch, 'X');
        assert_eq!(buf.get_cell(Position { x: 0, y: 0 }).ch, ' '); // default is space
    }

    #[test]
    fn test_swap_frames() {
        let mut buf = PairedBuffer::new(Size::new(10, 5));
        let mut cell_a = Cell::EMPTY;
        cell_a.ch = 'A';
        let mut cell_b = Cell::EMPTY;
        cell_b.ch = 'B';

        // Write to current frame (0)
        buf.set_cell(Position { x: 0, y: 0 }, cell_a);
        assert_eq!(buf.get_cell(Position { x: 0, y: 0 }).ch, 'A');

        // Swap to frame 1
        buf.swap_frames();
        // Frame 1 should be empty (from initialization)
        assert_eq!(buf.get_cell(Position { x: 0, y: 0 }).ch, ' ');

        // Write to frame 1
        buf.set_cell(Position { x: 0, y: 0 }, cell_b);
        assert_eq!(buf.get_cell(Position { x: 0, y: 0 }).ch, 'B');

        // Swap back to frame 0
        buf.swap_frames();
        // Should see 'A' again
        assert_eq!(buf.get_cell(Position { x: 0, y: 0 }).ch, 'A');
    }

    #[test]
    fn test_draw_diff() {
        let mut buf = PairedBuffer::new(Size::new(2, 2));
        let mut cell_a = Cell::EMPTY;
        cell_a.ch = 'A';

        // Initial state: order 0. Both frames empty.
        // Draw: compares 0 vs 1. No diff. Swaps to 1.
        assert_eq!(buf.draw().count(), 0);
        // Now order is 1.

        // Write 'A' to current frame (1).
        buf.set_cell(Position { x: 0, y: 0 }, cell_a);

        // Draw: compares 1 ('A') vs 0 (empty). Diff! Swaps to 0.
        let calls: Vec<_> = buf.draw().collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].pos.x, 0);
        assert_eq!(calls[0].pos.y, 0);
        assert_eq!(calls[0].cell.ch, 'A');
        // Now order is 0.

        // Write 'A' to current frame (0).
        buf.set_cell(Position { x: 0, y: 0 }, cell_a);

        // Draw: compares 0 ('A') vs 1 ('A'). Equal. No diff. Swaps to 1.
        assert_eq!(buf.draw().count(), 0);
    }

    #[test]
    fn test_out_of_bounds() {
        let mut buf = PairedBuffer::new(Size::new(10, 5));
        let size = Size {
            width: 10,
            height: 5,
        };
        assert!(
            buf.set_cell_checked(size, Position { x: 10, y: 0 }, Cell::EMPTY)
                .is_err()
        );
        assert!(
            buf.set_cell_checked(size, Position { x: 0, y: 5 }, Cell::EMPTY)
                .is_err()
        );
        assert!(
            buf.get_cell_checked(size, Position { x: 10, y: 0 })
                .is_err()
        );
    }

    #[test]
    fn test_multiple_swaps_and_draws() {
        let mut buf = PairedBuffer::new(Size::new(5, 5));
        let mut cell_a = Cell::EMPTY;
        cell_a.ch = 'A';
        let mut cell_b = Cell::EMPTY;
        cell_b.ch = 'B';

        // 0. Initial state (both empty).
        // Current buffer: 0. Old buffer: 1.
        // Draw => compare 0 vs 1. No changes. Swap => Current: 1. Old: 0.
        assert_eq!(buf.draw().count(), 0);

        // 1. Write 'A' to current buffer (1).
        buf.set_cell(Position { x: 2, y: 2 }, cell_a);

        // Draw => compare 1 (A) vs 0 (Empty). Change detected at (2,2).
        // Swap => Current: 0. Old: 1.
        let calls: Vec<_> = buf.draw().collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].cell.ch, 'A');

        // 2. Write 'B' to current buffer (0).
        buf.set_cell(Position { x: 2, y: 2 }, cell_b);

        // Draw => compare 0 (B) vs 1 (A). Change detected at (2,2).
        // Swap => Current: 1. Old: 0.
        let calls: Vec<_> = buf.draw().collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].cell.ch, 'B');

        // 3. Write 'B' to current buffer (1).
        buf.set_cell(Position { x: 2, y: 2 }, cell_b);

        // Draw => compare 1 (B) vs 0 (B). No changes.
        // Swap => Current: 0. Old: 1.
        assert_eq!(buf.draw().count(), 0);

        // 4. Write Empty to current buffer (0).
        buf.set_cell(Position { x: 2, y: 2 }, Cell::EMPTY);

        // Draw => compare 0 (Empty) vs 1 (B). Change detected.
        // Swap => Current: 1. Old: 0.
        let calls: Vec<_> = buf.draw().collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].cell.ch, ' ');
    }

    #[test]
    fn test_complex_moving_pixel() {
        let mut buf = PairedBuffer::new(Size::new(5, 1));
        let mut cell = Cell::EMPTY;
        cell.ch = '#';

        // Frame 0: Draw '#' at (0,0)
        buf.set_cell(Position { x: 0, y: 0 }, cell);

        // Draw 0:
        // Current (0): [(0,0)='#']
        // Old (1):     []
        // Diff: (0,0) -> '#'
        let calls: Vec<_> = buf.draw().collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].pos.x, 0);
        assert_eq!(calls[0].cell.ch, '#');

        // Swap happened. Current is 1. Old is 0.
        // Old (0) still has '#' at (0,0).

        // Frame 1: Move '#' to (1,0).
        // We are writing to Buffer 1. It is currently empty (never touched).
        buf.set_cell(Position { x: 1, y: 0 }, cell);

        // Draw 1:
        // Current (1): [(1,0)='#']
        // Old (0):     [(0,0)='#']
        // Diff:
        // (0,0): 1=' ' vs 0='#'. Diff -> ' ' (Clear old pos)
        // (1,0): 1='#' vs 0=' '. Diff -> '#' (Draw new pos)
        let mut calls: Vec<_> = buf.draw().collect();
        calls.sort_by_key(|c| c.pos.x); // Sort by x to ensure deterministic order check
        assert_eq!(calls.len(), 2);

        // Clear (0,0)
        assert_eq!(calls[0].pos.x, 0);
        assert_eq!(calls[0].cell.ch, ' ');

        // Draw (1,0)
        assert_eq!(calls[1].pos.x, 1);
        assert_eq!(calls[1].cell.ch, '#');

        // Swap happened. Current is 0. Old is 1.
        // Buffer 0 still has '#' at (0,0) from Frame 0!
        // Buffer 1 has '#' at (1,0) from Frame 1.

        // Frame 2: Move '#' to (2,0).
        // We are writing to Buffer 0.
        // IMPORTANT: We must clear the old artifact at (0,0) on Buffer 0,
        // because it persists from Frame 0.
        buf.set_cell(Position { x: 0, y: 0 }, Cell::EMPTY);
        buf.set_cell(Position { x: 2, y: 0 }, cell);

        // Draw 2:
        // Current (0): [(0,0)=' ', (2,0)='#']
        // Old (1):     [(1,0)='#']
        // Diff:
        // (0,0): 0=' ' vs 1=' '. Same. No DrawCall.
        // (1,0): 0=' ' vs 1='#'. Diff -> ' ' (Clear old pos from Frame 1)
        // (2,0): 0='#' vs 1=' '. Diff -> '#' (Draw new pos)
        let mut calls: Vec<_> = buf.draw().collect();
        calls.sort_by_key(|c| c.pos.x);
        assert_eq!(calls.len(), 2);

        // Clear (1,0)
        assert_eq!(calls[0].pos.x, 1);
        assert_eq!(calls[0].cell.ch, ' ');

        // Draw (2,0)
        assert_eq!(calls[1].pos.x, 2);
        assert_eq!(calls[1].cell.ch, '#');
    }

    #[test]
    fn test_swap_persistence() {
        let mut buf = PairedBuffer::new(Size::new(5, 5));
        let mut cell_1 = Cell::EMPTY;
        cell_1.ch = '1';
        let mut cell_2 = Cell::EMPTY;
        cell_2.ch = '2';

        // Write '1' to (0,0) on Frame A
        buf.set_cell(Position { x: 0, y: 0 }, cell_1);
        assert_eq!(buf.get_cell(Position { x: 0, y: 0 }).ch, '1');

        // Swap. Now current is Frame B.
        buf.swap_frames();
        // Frame B should be empty.
        assert_eq!(buf.get_cell(Position { x: 0, y: 0 }).ch, ' ');

        // Write '2' to (1,1) on Frame B.
        buf.set_cell(Position { x: 1, y: 1 }, cell_2);
        assert_eq!(buf.get_cell(Position { x: 1, y: 1 }).ch, '2');

        // Swap back to Frame A.
        buf.swap_frames();
        // Should see '1' at (0,0) and empty at (1,1).
        assert_eq!(buf.get_cell(Position { x: 0, y: 0 }).ch, '1');
        assert_eq!(buf.get_cell(Position { x: 1, y: 1 }).ch, ' ');

        // Swap to Frame B.
        buf.swap_frames();
        // Should see empty at (0,0) and '2' at (1,1).
        assert_eq!(buf.get_cell(Position { x: 0, y: 0 }).ch, ' ');
        assert_eq!(buf.get_cell(Position { x: 1, y: 1 }).ch, '2');
    }

    #[test]
    fn test_overwrite_after_swap() {
        let mut buf = PairedBuffer::new(Size::new(3, 3));
        let mut cell = Cell::EMPTY;

        // Frame A: Write 'A'
        cell.ch = 'A';
        buf.set_cell(Position { x: 0, y: 0 }, cell);

        buf.swap_frames();

        // Frame B: Write 'B' at same spot
        cell.ch = 'B';
        buf.set_cell(Position { x: 0, y: 0 }, cell);

        // Check B
        assert_eq!(buf.get_cell(Position { x: 0, y: 0 }).ch, 'B');

        buf.swap_frames();
        // Check A is still 'A'
        assert_eq!(buf.get_cell(Position { x: 0, y: 0 }).ch, 'A');

        // Overwrite A with 'C'
        cell.ch = 'C';
        buf.set_cell(Position { x: 0, y: 0 }, cell);
        assert_eq!(buf.get_cell(Position { x: 0, y: 0 }).ch, 'C');

        buf.swap_frames();
        // Check B is still 'B'
        assert_eq!(buf.get_cell(Position { x: 0, y: 0 }).ch, 'B');
    }
}
