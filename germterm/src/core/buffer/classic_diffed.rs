use crate::{
    cell::Cell,
    core::{
        Position,
        buffer::{Drawer, ResizableBuffer},
        draw::Size,
    },
};

use super::{Buffer, DrawCall};
pub struct ClassicDiffedBuffers<Buf: Buffer> {
    size: Size,
    current_buf: Buf,
    last_frame: Vec<(Position, Cell)>,
}

impl<Buf: Buffer> ClassicDiffedBuffers<Buf> {
    pub fn new(size: Size, buf: Buf) -> Self {
        Self {
            size,
            current_buf: buf,
            last_frame: {
                (0..size.width)
                    .flat_map(|y| {
                        (0..size.height).map(move |x| (Position::new(x, y), Cell::TRANSPARENT))
                    })
                    .collect()
            },
        }
    }

    pub fn current_buffer_mut(&mut self) -> &mut Buf {
        &mut self.current_buf
    }
}

impl<Buf> Buffer for ClassicDiffedBuffers<Buf>
where
    Buf: Buffer + Drawer,
{
    fn size(&self) -> Size {
        self.size
    }

    fn set_cell_checked(
        &mut self,
        pos: Position,
        cell: Cell,
    ) -> Result<(), super::ErrorOutOfBoundsAxises> {
        self.current_buf.set_cell_checked(pos, cell)
    }

    fn get_cell_checked(&self, pos: Position) -> Result<&Cell, super::ErrorOutOfBoundsAxises> {
        self.current_buf.get_cell_checked(pos)
    }

    fn get_cell_mut_checked(
        &mut self,
        pos: Position,
    ) -> Result<&mut Cell, super::ErrorOutOfBoundsAxises> {
        self.current_buf.get_cell_mut_checked(pos)
    }

    fn fill(&mut self, cell: Cell) {
        self.current_buf.fill(cell);
    }

    fn clear(&mut self) {
        self.current_buf.clear()
    }

    fn start_frame(&mut self) {
        self.current_buf.start_frame();
    }

    fn end_frame(&mut self) {
        self.current_buf.end_frame();
        self.last_frame = self
            .current_buf
            .draw()
            .map(|dc| (dc.pos, *dc.cell))
            .collect();
    }
}

impl<Buf> ResizableBuffer for ClassicDiffedBuffers<Buf>
where
    Buf: Buffer + ResizableBuffer + Drawer,
{
    fn resize(&mut self, size: Size) {
        self.size = size;
        self.current_buf.resize(size);
    }
}

// This doesnt work
impl<CurrentBuf> Drawer for ClassicDiffedBuffers<CurrentBuf>
where
    CurrentBuf: Buffer + Drawer,
{
    fn draw(&mut self) -> impl Iterator<Item = DrawCall<'_>> {
        let old = &self.last_frame;
        self.current_buf
            .draw()
            .enumerate()
            .filter(move |(i, dc)| match old.get(*i) {
                Some((_, cell)) => cell != dc.cell,
                None => true,
            })
            .map(|(_, dc)| dc)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::DiffedBuffers;
//     use crate::{
//         buffer_resizing_tests, buffer_tests,
//         core::{buffer::flat::FlatBuffer, draw::Size},
//         drawer_diffed_buffer_tests,
//     };

//     type TestBuffer = DiffedBuffers<FlatBuffer>;

//     fn new_tb(sz: Size) -> TestBuffer {
//         TestBuffer::new(sz, FlatBuffer::new(sz), FlatBuffer::new(sz))
//     }
//     buffer_tests! {
//         buffer_tests,
//         super::new_tb,
//         TestBuffer
//     }

//     buffer_resizing_tests! {
//         resizing_tests,
//         super::new_tb,
//         TestBuffer
//     }

//     drawer_diffed_buffer_tests! {
//         diffed_tests,
//         super::new_tb,
//         TestBuffer
//     }
// }
