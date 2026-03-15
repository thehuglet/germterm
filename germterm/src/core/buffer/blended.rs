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
    bg_fallback: Color,
}

impl<Buf: Buffer> BlendedBuffer<Buf> {
    pub fn new(inner: Buf) -> Self {
        Self {
            inner,
            bg_fallback: Color::BLACK,
        }
    }

    pub fn with_bg_fallback(mut self, color: Color) -> Self {
        self.bg_fallback = color;
        self
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
        let bg_fallback: Color = self.bg_fallback;
        let bottom_cell = self.get_cell_mut_checked(pos)?;
        cell.style.premultiply_fg_and_bg();
        compose_cell(bottom_cell, &cell, bg_fallback);

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

    fn clear(&mut self) {
        self.inner.clear()
    }

    fn start_frame(&mut self) {
        self.inner.start_frame()
    }

    fn end_frame(&mut self) {
        self.inner.end_frame()
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
