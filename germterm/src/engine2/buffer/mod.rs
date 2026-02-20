pub mod diffed;
pub mod flat;
pub mod paired;
pub mod slice;
pub mod test;
pub mod utils;

use super::DrawCall;
use crate::{
    cell::Cell,
    engine2::{Position, draw::Size},
};

/// Indicates which axis (or axes) caused an out-of-bounds access.
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorOutOfBoundsAxises {
    /// The X coordinate was out of bounds.
    X,
    /// The Y coordinate was out of bounds.
    Y,
    /// Both the X and Y coordinates were out of bounds.
    XY,
}

/// A 2D grid of [`Cell`]s that can be read and written by position.
///
/// Implementors manage their own internal storage of [`Cell`]'s.
/// Implementations must provide the checked variants of read/write methods
/// which return an error if the given position falls outside the buffer's
/// bounds. The unchecked variants must panic when a given position is out of bounds.
pub trait Buffer {
    /// The size of the area that can be drawn in this buffer
    fn size(&self) -> Size;

    /// Sets the cell at `pos`, returning an error if `pos` is outside bounds.
    fn set_cell_checked(&mut self, pos: Position, cell: Cell)
    -> Result<(), ErrorOutOfBoundsAxises>;
    /// Sets the cell at `pos` without bounds checking.
    ///
    /// # Panics
    ///
    /// Panics if `pos` is out of bounds.
    fn set_cell(&mut self, pos: Position, cell: Cell) {
        self.set_cell_checked(pos, cell)
            .expect("out of bounds set_cell")
    }

    /// Returns a reference to the cell at `pos`, returning an error if `pos` is outside bounds.
    fn get_cell_checked(&self, pos: Position) -> Result<&Cell, ErrorOutOfBoundsAxises>;

    /// Returns a reference to the cell at `pos` without bounds checking.
    ///
    /// # Panics
    ///
    /// Panics if `pos` is out of bounds.
    fn get_cell(&self, pos: Position) -> &Cell {
        self.get_cell_checked(pos).expect("out of bounds get_cell")
    }

    /// Returns a mutable reference to the cell at `pos`, returning an error if `pos` is outside bounds.
    fn get_cell_mut_checked(&mut self, pos: Position) -> Result<&mut Cell, ErrorOutOfBoundsAxises>;

    /// Returns a mutable reference to the cell at `pos` without bounds checking.
    ///
    /// # Panics
    ///
    /// Panics if `pos` is out of bounds.
    fn get_cell_mut(&mut self, pos: Position) -> &mut Cell {
        self.get_cell_mut_checked(pos)
            .expect("out of bounds get_cell_mut")
    }

    /// Fills the entire buffer with `cell`.
    fn fill(&mut self, cell: Cell) {
        let size = self.size();
        for y in 0..size.height {
            for x in 0..size.width {
                self.set_cell(Position { x, y }, cell);
            }
        }
    }

    /// Clears the buffer by filling it with [`Cell::EMPTY`].
    fn clear(&mut self) {
        self.fill(Cell::EMPTY);
    }

    /// Called at the beginning of a frame. Implementations may use this to
    /// clear or prepare the buffer for new draw commands.
    fn start_frame(&mut self) {}
    /// Called at the end of a frame. Implementations may use this to
    /// flush or finalise the buffer contents.
    fn end_frame(&mut self) {}
}

pub trait ResizableBuffer: Buffer {
    /// Resizes this buffer to `size`.
    ///
    /// After performing a resize the [`Size`] provided here must be returned from
    /// [`Buffer::size`]. Not doing so may result in incorrect values or panics.
    fn resize(&mut self, size: Size);
}

/// Produces an iterator of [`DrawCall`]s representing cells that need to be
/// rendered to the terminal for the current frame.
pub trait Drawer {
    fn draw(&mut self) -> impl Iterator<Item = DrawCall<'_>>;
}
