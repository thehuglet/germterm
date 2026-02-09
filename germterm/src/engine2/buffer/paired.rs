use super::{Buffer, DrawCall, Drawer, ErrorOutOfBoundsAxises};
use crate::cell::Cell;

#[derive(Clone, Copy, Debug)]
enum FrameOrder {
    CurrentOld = 0,
    OldCurrent = 1,
}

pub struct PairedBuffer {
    width: u16,
    height: u16,
    frames: Vec<Cell>,
    order: FrameOrder,
}

impl PairedBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            frames: vec![Cell::EMPTY; (width as usize * height as usize) * 2],
            order: FrameOrder::CurrentOld,
        }
    }

    fn index(&self, x: u16, y: u16, order: FrameOrder) -> usize {
        let base = (y as usize * self.width as usize + x as usize) * 2;
        base + (order as usize)
    }

    pub fn swap_frames(&mut self) {
        self.order = match self.order {
            FrameOrder::CurrentOld => FrameOrder::OldCurrent,
            FrameOrder::OldCurrent => FrameOrder::CurrentOld,
        };
    }
}

impl Buffer for PairedBuffer {
    fn set_cell_checked(
        &mut self,
        x: u16,
        y: u16,
        cell: Cell,
    ) -> Result<(), ErrorOutOfBoundsAxises> {
        if x >= self.width && y >= self.height {
            return Err(ErrorOutOfBoundsAxises::XY);
        } else if x >= self.width {
            return Err(ErrorOutOfBoundsAxises::X);
        } else if y >= self.height {
            return Err(ErrorOutOfBoundsAxises::Y);
        }

        let idx = self.index(x, y, self.order);
        self.frames[idx] = cell;
        Ok(())
    }

    fn get_cell_checked(&self, x: u16, y: u16) -> Result<&Cell, ErrorOutOfBoundsAxises> {
        if x >= self.width && y >= self.height {
            return Err(ErrorOutOfBoundsAxises::XY);
        } else if x >= self.width {
            return Err(ErrorOutOfBoundsAxises::X);
        } else if y >= self.height {
            return Err(ErrorOutOfBoundsAxises::Y);
        }

        let idx = self.index(x, y, self.order);
        Ok(&self.frames[idx])
    }

    fn get_cell_mut_checked(
        &mut self,
        x: u16,
        y: u16,
    ) -> Result<&mut Cell, ErrorOutOfBoundsAxises> {
        if x >= self.width && y >= self.height {
            return Err(ErrorOutOfBoundsAxises::XY);
        } else if x >= self.width {
            return Err(ErrorOutOfBoundsAxises::X);
        } else if y >= self.height {
            return Err(ErrorOutOfBoundsAxises::Y);
        }

        let idx = self.index(x, y, self.order);
        Ok(&mut self.frames[idx])
    }

    fn set_cell(&mut self, x: u16, y: u16, cell: Cell) {
        let idx = self.index(x, y, self.order);
        self.frames[idx] = cell;
    }

    fn get_cell_mut(&mut self, x: u16, y: u16) -> &mut Cell {
        let idx = self.index(x, y, self.order);
        &mut self.frames[idx]
    }

    fn get_cell(&self, x: u16, y: u16) -> &Cell {
        let idx = self.index(x, y, self.order);
        &self.frames[idx]
    }
}

impl Drawer for PairedBuffer {
    fn draw(&mut self) -> impl Iterator<Item = DrawCall<'_>> {
        let width = self.width;
        let height = self.height;
        let order = self.order as usize;
        let old_order = 1 - order;

        self.swap_frames();
        let frames_slice = &self.frames;

        (0..height).flat_map(move |y| {
            (0..width).filter_map(move |x| {
                let base_idx = (y as usize * width as usize + x as usize) * 2;
                let current_idx = base_idx + order;
                let old_idx = base_idx + old_order;

                let current_cell = &frames_slice[current_idx];
                let old_cell = &frames_slice[old_idx];

                if current_cell != old_cell {
                    Some(DrawCall {
                        x,
                        y,
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
    use super::*;

    #[test]
    fn test_new() {
        let buf = PairedBuffer::new(10, 5);
        assert_eq!(buf.width, 10);
        assert_eq!(buf.height, 5);
        assert_eq!(buf.frames.len(), 10 * 5 * 2);
    }

    #[test]
    fn test_set_get_cell() {
        let mut buf = PairedBuffer::new(10, 5);
        let mut cell = Cell::EMPTY;
        cell.ch = 'X';

        buf.set_cell(1, 1, cell);
        assert_eq!(buf.get_cell(1, 1).ch, 'X');
        assert_eq!(buf.get_cell(0, 0).ch, ' '); // default is space
    }

    #[test]
    fn test_swap_frames() {
        let mut buf = PairedBuffer::new(10, 5);
        let mut cell_a = Cell::EMPTY;
        cell_a.ch = 'A';
        let mut cell_b = Cell::EMPTY;
        cell_b.ch = 'B';

        // Write to current frame (0)
        buf.set_cell(0, 0, cell_a);
        assert_eq!(buf.get_cell(0, 0).ch, 'A');

        // Swap to frame 1
        buf.swap_frames();
        // Frame 1 should be empty (from initialization)
        assert_eq!(buf.get_cell(0, 0).ch, ' ');

        // Write to frame 1
        buf.set_cell(0, 0, cell_b);
        assert_eq!(buf.get_cell(0, 0).ch, 'B');

        // Swap back to frame 0
        buf.swap_frames();
        // Should see 'A' again
        assert_eq!(buf.get_cell(0, 0).ch, 'A');
    }

    #[test]
    fn test_draw_diff() {
        let mut buf = PairedBuffer::new(2, 2);
        let mut cell_a = Cell::EMPTY;
        cell_a.ch = 'A';

        // Initial state: order 0. Both frames empty.
        // Draw: compares 0 vs 1. No diff. Swaps to 1.
        assert_eq!(buf.draw().count(), 0);
        // Now order is 1.

        // Write 'A' to current frame (1).
        buf.set_cell(0, 0, cell_a);

        // Draw: compares 1 ('A') vs 0 (empty). Diff! Swaps to 0.
        let calls: Vec<_> = buf.draw().collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].x, 0);
        assert_eq!(calls[0].y, 0);
        assert_eq!(calls[0].cell.ch, 'A');
        // Now order is 0.

        // Write 'A' to current frame (0).
        buf.set_cell(0, 0, cell_a);

        // Draw: compares 0 ('A') vs 1 ('A'). Equal. No diff. Swaps to 1.
        assert_eq!(buf.draw().count(), 0);
    }

    #[test]
    fn test_out_of_bounds() {
        let mut buf = PairedBuffer::new(10, 5);
        assert!(buf.set_cell_checked(10, 0, Cell::EMPTY).is_err());
        assert!(buf.set_cell_checked(0, 5, Cell::EMPTY).is_err());
        assert!(buf.get_cell_checked(10, 0).is_err());
    }

    #[test]
    fn test_multiple_swaps_and_draws() {
        let mut buf = PairedBuffer::new(5, 5);
        let mut cell_a = Cell::EMPTY;
        cell_a.ch = 'A';
        let mut cell_b = Cell::EMPTY;
        cell_b.ch = 'B';

        // 0. Initial state (both empty).
        // Current buffer: 0. Old buffer: 1.
        // Draw => compare 0 vs 1. No changes. Swap => Current: 1. Old: 0.
        assert_eq!(buf.draw().count(), 0);

        // 1. Write 'A' to current buffer (1).
        buf.set_cell(2, 2, cell_a);

        // Draw => compare 1 (A) vs 0 (Empty). Change detected at (2,2).
        // Swap => Current: 0. Old: 1.
        let calls: Vec<_> = buf.draw().collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].cell.ch, 'A');

        // 2. Write 'B' to current buffer (0).
        buf.set_cell(2, 2, cell_b);

        // Draw => compare 0 (B) vs 1 (A). Change detected at (2,2).
        // Swap => Current: 1. Old: 0.
        let calls: Vec<_> = buf.draw().collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].cell.ch, 'B');

        // 3. Write 'B' to current buffer (1).
        buf.set_cell(2, 2, cell_b);

        // Draw => compare 1 (B) vs 0 (B). No changes.
        // Swap => Current: 0. Old: 1.
        assert_eq!(buf.draw().count(), 0);

        // 4. Write Empty to current buffer (0).
        buf.set_cell(2, 2, Cell::EMPTY);

        // Draw => compare 0 (Empty) vs 1 (B). Change detected.
        // Swap => Current: 1. Old: 0.
        let calls: Vec<_> = buf.draw().collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].cell.ch, ' ');
    }

    #[test]
    fn test_complex_moving_pixel() {
        let mut buf = PairedBuffer::new(5, 1);
        let mut cell = Cell::EMPTY;
        cell.ch = '#';

        // Frame 0: Draw '#' at (0,0)
        buf.set_cell(0, 0, cell);

        // Draw 0:
        // Current (0): [(0,0)='#']
        // Old (1):     []
        // Diff: (0,0) -> '#'
        let calls: Vec<_> = buf.draw().collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].x, 0);
        assert_eq!(calls[0].cell.ch, '#');

        // Swap happened. Current is 1. Old is 0.
        // Old (0) still has '#' at (0,0).

        // Frame 1: Move '#' to (1,0).
        // We are writing to Buffer 1. It is currently empty (never touched).
        buf.set_cell(1, 0, cell);

        // Draw 1:
        // Current (1): [(1,0)='#']
        // Old (0):     [(0,0)='#']
        // Diff:
        // (0,0): 1=' ' vs 0='#'. Diff -> ' ' (Clear old pos)
        // (1,0): 1='#' vs 0=' '. Diff -> '#' (Draw new pos)
        let mut calls: Vec<_> = buf.draw().collect();
        calls.sort_by_key(|c| c.x); // Sort by x to ensure deterministic order check
        assert_eq!(calls.len(), 2);

        // Clear (0,0)
        assert_eq!(calls[0].x, 0);
        assert_eq!(calls[0].cell.ch, ' ');

        // Draw (1,0)
        assert_eq!(calls[1].x, 1);
        assert_eq!(calls[1].cell.ch, '#');

        // Swap happened. Current is 0. Old is 1.
        // Buffer 0 still has '#' at (0,0) from Frame 0!
        // Buffer 1 has '#' at (1,0) from Frame 1.

        // Frame 2: Move '#' to (2,0).
        // We are writing to Buffer 0.
        // IMPORTANT: We must clear the old artifact at (0,0) on Buffer 0,
        // because it persists from Frame 0.
        buf.set_cell(0, 0, Cell::EMPTY);
        buf.set_cell(2, 0, cell);

        // Draw 2:
        // Current (0): [(0,0)=' ', (2,0)='#']
        // Old (1):     [(1,0)='#']
        // Diff:
        // (0,0): 0=' ' vs 1=' '. Same. No DrawCall.
        // (1,0): 0=' ' vs 1='#'. Diff -> ' ' (Clear old pos from Frame 1)
        // (2,0): 0='#' vs 1=' '. Diff -> '#' (Draw new pos)
        let mut calls: Vec<_> = buf.draw().collect();
        calls.sort_by_key(|c| c.x);
        assert_eq!(calls.len(), 2);

        // Clear (1,0)
        assert_eq!(calls[0].x, 1);
        assert_eq!(calls[0].cell.ch, ' ');

        // Draw (2,0)
        assert_eq!(calls[1].x, 2);
        assert_eq!(calls[1].cell.ch, '#');
    }

    #[test]
    fn test_swap_persistence() {
        let mut buf = PairedBuffer::new(5, 5);
        let mut cell_1 = Cell::EMPTY;
        cell_1.ch = '1';
        let mut cell_2 = Cell::EMPTY;
        cell_2.ch = '2';

        // Write '1' to (0,0) on Frame A
        buf.set_cell(0, 0, cell_1);
        assert_eq!(buf.get_cell(0, 0).ch, '1');

        // Swap. Now current is Frame B.
        buf.swap_frames();
        // Frame B should be empty.
        assert_eq!(buf.get_cell(0, 0).ch, ' ');

        // Write '2' to (1,1) on Frame B.
        buf.set_cell(1, 1, cell_2);
        assert_eq!(buf.get_cell(1, 1).ch, '2');

        // Swap back to Frame A.
        buf.swap_frames();
        // Should see '1' at (0,0) and empty at (1,1).
        assert_eq!(buf.get_cell(0, 0).ch, '1');
        assert_eq!(buf.get_cell(1, 1).ch, ' ');

        // Swap to Frame B.
        buf.swap_frames();
        // Should see empty at (0,0) and '2' at (1,1).
        assert_eq!(buf.get_cell(0, 0).ch, ' ');
        assert_eq!(buf.get_cell(1, 1).ch, '2');
    }

    #[test]
    fn test_overwrite_after_swap() {
        let mut buf = PairedBuffer::new(3, 3);
        let mut cell = Cell::EMPTY;

        // Frame A: Write 'A'
        cell.ch = 'A';
        buf.set_cell(0, 0, cell);

        buf.swap_frames();

        // Frame B: Write 'B' at same spot
        cell.ch = 'B';
        buf.set_cell(0, 0, cell);

        // Check B
        assert_eq!(buf.get_cell(0, 0).ch, 'B');

        buf.swap_frames();
        // Check A is still 'A'
        assert_eq!(buf.get_cell(0, 0).ch, 'A');

        // Overwrite A with 'C'
        cell.ch = 'C';
        buf.set_cell(0, 0, cell);
        assert_eq!(buf.get_cell(0, 0).ch, 'C');

        buf.swap_frames();
        // Check B is still 'B'
        assert_eq!(buf.get_cell(0, 0).ch, 'B');
    }

    #[test]
    fn test_redundant_updates_yield_no_draws() {
        let mut buf = PairedBuffer::new(3, 3);
        let mut cell = Cell::EMPTY;
        cell.ch = 'X';

        // 1. Basic redundancy check
        // Frame A: Write 'X' at (0,0)
        buf.set_cell(0, 0, cell);

        // Draw 1: A vs B(empty). Yields (0,0) -> 'X'. Swap -> B is current.
        assert_eq!(buf.draw().count(), 1);

        // Frame B: Write 'X' at (0,0).
        // Since A (old) has 'X' at (0,0), and B (current) has 'X' at (0,0).
        buf.set_cell(0, 0, cell);

        // Draw 2: B vs A. 'X' == 'X'. Should yield NONE.
        let count = buf.draw().count();
        assert_eq!(
            count, 0,
            "Expected 0 draw calls when content matches old frame"
        );

        // 2. Mixed redundancy check
        // Current is A (after Draw 2). Old is B ('X').

        // Write 'X' at (0,0) [Redundant, matches B]
        buf.set_cell(0, 0, cell);

        // Write 'Y' at (1,1) [New]
        let mut cell_y = Cell::EMPTY;
        cell_y.ch = 'Y';
        buf.set_cell(1, 1, cell_y);

        // Draw 3: A vs B.
        // (0,0): 'X' vs 'X' -> No draw.
        // (1,1): 'Y' vs ' ' -> Draw.
        let calls: Vec<_> = buf.draw().collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].x, 1);
        assert_eq!(calls[0].y, 1);
        assert_eq!(calls[0].cell.ch, 'Y');

        // 3. Redundant Clear check
        // Current is B. Old is A ('X' at 0,0; 'Y' at 1,1).
        // B has 'X' at 0,0 (persisted/written previously) and ' ' at 1,1 (default).

        // Explicitly write ' ' at (1,1) to B.
        // A has 'Y' at (1,1). So this IS a change (clearing).
        buf.set_cell(1, 1, Cell::EMPTY);

        // Draw 4: B vs A.
        // (0,0): 'X' vs 'X' -> No draw.
        // (1,1): ' ' vs 'Y' -> Draw (clearing).
        let calls: Vec<_> = buf.draw().collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].x, 1);
        assert_eq!(calls[0].y, 1);
        assert_eq!(calls[0].cell.ch, ' ');

        // 4. Truly Redundant Clear
        // Current is A. Old is B ('X' at 0,0; ' ' at 1,1).

        // Write ' ' at (1,1) to A.
        buf.set_cell(1, 1, Cell::EMPTY);
        // Write 'X' at (0,0) to A.
        buf.set_cell(0, 0, cell);

        // Draw 5: A vs B.
        // (0,0): 'X' vs 'X' -> No draw.
        // (1,1): ' ' vs ' ' -> No draw.
        assert_eq!(buf.draw().count(), 0);
    }
}
