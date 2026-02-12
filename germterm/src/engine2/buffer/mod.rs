pub mod diffed;
pub mod paired;

use super::DrawCall;
use crate::{
    cell::Cell,
    engine2::{draw::Size, Position},
};

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorOutOfBoundsAxises {
    X,
    Y,
    XY,
}

pub trait Buffer {
    fn set_cell(&mut self, pos: Position, cell: Cell);
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
    fn get_cell(&self, pos: Position) -> &Cell;
    fn get_cell_mut(&mut self, pos: Position) -> &mut Cell;
    fn get_cell_checked(&self, size: Size, pos: Position) -> Result<&Cell, ErrorOutOfBoundsAxises> {
        size.contains(pos)?;
        Ok(self.get_cell(pos))
    }
    fn get_cell_mut_checked(
        &mut self,
        size: Size,
        pos: Position,
    ) -> Result<&mut Cell, ErrorOutOfBoundsAxises> {
        size.contains(pos)?;
        Ok(self.get_cell_mut(pos))
    }

    fn start_frame(&mut self) {}
    fn end_frame(&mut self) {}
    fn resize(&mut self, size: Size);
}

pub trait Drawer {
    fn draw(&mut self) -> impl Iterator<Item = DrawCall<'_>>;
}
