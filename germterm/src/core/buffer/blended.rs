use crate::{
    cell::Cell,
    color::Color,
    core::{
        DrawCall,
        buffer::{Buffer, Drawer},
        compositor::compose_cell,
        draw::{Position, Size},
    },
};

pub struct BlendedBuffer<Buf: Buffer> {
    inner: Buf,
    default_blending_bg_color: Color,
}

impl<Buf: Buffer> BlendedBuffer<Buf> {
    pub fn new(inner: Buf) -> Self {
        Self {
            inner,
            default_blending_bg_color: Color::BLACK,
        }
    }
}

impl<Buf: Buffer> Buffer for BlendedBuffer<Buf> {
    fn size(&self) -> Size {
        self.inner.size()
    }

    fn set_cell_checked(
        &mut self,
        pos: Position,
        mut cell: Cell,
    ) -> Result<(), super::ErrorOutOfBoundsAxises> {
        let bottom_cell = self.get_cell_mut_checked(pos)?;
        cell.style.premultiply_fg_and_bg();
        compose_cell(bottom_cell, &cell);

        Ok(())
    }

    fn get_cell_checked(&self, pos: Position) -> Result<&Cell, super::ErrorOutOfBoundsAxises> {
        self.inner.get_cell_checked(pos)
    }

    fn get_cell_mut_checked(
        &mut self,
        pos: Position,
    ) -> Result<&mut Cell, super::ErrorOutOfBoundsAxises> {
        self.inner.get_cell_mut_checked(pos)
    }
}

impl<Buf> Drawer for BlendedBuffer<Buf>
where
    Buf: Buffer + Drawer,
{
    fn draw(&mut self) -> impl Iterator<Item = DrawCall<'_>> {
        self.inner.draw()
    }
}
