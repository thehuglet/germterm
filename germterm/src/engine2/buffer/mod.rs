pub mod diffed;
pub mod paired;

use super::DrawCall;
use crate::{cell::Cell, draw::Layer};

pub enum ErrorOutOfBoundsAxises {
    X,
    Y,
    XY,
}

pub trait Buffer {
    fn set_cell(&mut self, x: u16, y: u16, cell: Cell);
    fn set_cell_checked(
        &mut self,
        x: u16,
        y: u16,
        cell: Cell,
    ) -> Result<(), ErrorOutOfBoundsAxises>;
    fn get_cell(&self, x: u16, y: u16) -> &Cell;
    fn get_cell_mut(&mut self, x: u16, y: u16) -> &mut Cell;
    fn get_cell_checked(&self, x: u16, y: u16) -> Result<&Cell, ErrorOutOfBoundsAxises>;
    fn get_cell_mut_checked(&mut self, x: u16, y: u16)
    -> Result<&mut Cell, ErrorOutOfBoundsAxises>;

    fn start_frame(&mut self) {}
    fn end_frame(&mut self) {}
}

pub trait Drawer {
    fn draw(&mut self) -> impl Iterator<Item = DrawCall<'_>>;
}
