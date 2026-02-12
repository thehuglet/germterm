pub mod diffed;
pub mod paired;
pub mod slice;

use super::DrawCall;
use crate::{
    cell::Cell,
    engine2::{draw::Size, Position},
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
/// Implementors manage their own internal storage and frame lifecycle.
/// The checked variants of read/write methods return an error if the
/// given position falls outside the provided [`Size`] bounds.
pub trait Buffer {
    /// Sets the cell at `pos` without bounds checking.
    ///
    /// # Panics
    ///
    /// Implementations are free to panic if `pos` is out of bounds.
    fn set_cell(&mut self, pos: Position, cell: Cell);
    /// Sets the cell at `pos`, returning an error if `pos` is outside `size`.
    fn set_cell_checked(
        &mut self,
        size: Size,
        pos: Position,
        cell: Cell,
    ) -> Result<(), ErrorOutOfBoundsAxises> {
        if let err @ Err(_) = size.contains(pos) {
            return err;
        }

        self.set_cell(pos, cell);
        Ok(())
    }
    /// Returns a reference to the cell at `pos` without bounds checking.
    ///
    /// # Panics
    ///
    /// Implementations are free to panic if `pos` is out of bounds.
    fn get_cell(&self, pos: Position) -> &Cell;
    /// Returns a mutable reference to the cell at `pos` without bounds checking.
    ///
    /// # Panics
    ///
    /// Implementations are free to panic if `pos` is out of bounds.
    fn get_cell_mut(&mut self, pos: Position) -> &mut Cell;
    /// Returns a reference to the cell at `pos`, returning an error if `pos` is outside `size`.
    fn get_cell_checked(&self, size: Size, pos: Position) -> Result<&Cell, ErrorOutOfBoundsAxises> {
        size.contains(pos)?;
        Ok(self.get_cell(pos))
    }
    /// Returns a mutable reference to the cell at `pos`, returning an error if `pos` is outside `size`.
    fn get_cell_mut_checked(
        &mut self,
        size: Size,
        pos: Position,
    ) -> Result<&mut Cell, ErrorOutOfBoundsAxises> {
        size.contains(pos)?;
        Ok(self.get_cell_mut(pos))
    }

    /// Called at the beginning of a frame. Implementations may use this to
    /// clear or prepare the buffer for new draw commands.
    fn start_frame(&mut self) {}
    /// Called at the end of a frame. Implementations may use this to
    /// flush or finalise the buffer contents.
    fn end_frame(&mut self) {}
    /// Resizes the buffer to the given [`Size`].
    fn resize(&mut self, size: Size);

    /// Returns a [`SubSlice`](slice::SubSlice) viewing into this buffer at
    /// `origin` with the given `size`.
    ///
    /// All positions written through the sub-slice are translated by `origin`
    /// before reaching this buffer. The sub-slice's checked methods use `size`
    /// as the bounds.
    fn subslice(&mut self, origin: Position, size: Size) -> slice::SubBuffer<'_, Self>
    where
        Self: Sized,
    {
        slice::SubBuffer::new(self, origin, size)
    }
}

/// Produces an iterator of [`DrawCall`]s representing cells that need to be
/// rendered to the terminal for the current frame.
pub trait Drawer {
    fn draw(&mut self) -> impl Iterator<Item = DrawCall<'_>>;
}
