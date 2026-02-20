use super::{Buffer, DrawCall, Drawer, ErrorOutOfBoundsAxises, ResizableBuffer};
use crate::{
    cell::Cell,
    engine2::{Position, draw::Size},
};

/// A flat buffer that stores every cell in a single `Vec<Cell>` in row-major
/// order.
///
/// Unlike [`super::paired::PairedBuffer`] this buffer does **not** diff frames:
/// [`Drawer::draw`] always emits every cell on every call.
///
/// If you want a simple and fast `Buffer` that offers diffing, [`DiffedBuffers<FlatBuffer>`] can be
/// used instead.
///
/// [`DiffedBuffers<FlatBuffer>`]: super::diffed::DiffedBuffers
pub struct FlatBuffer {
    size: Size,
    cells: Vec<Cell>,
}

impl FlatBuffer {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            cells: vec![Cell::EMPTY; size.area() as usize],
        }
    }
}

impl Buffer for FlatBuffer {
    fn size(&self) -> Size {
        self.size
    }

    fn set_cell_checked(
        &mut self,
        pos: Position,
        cell: Cell,
    ) -> Result<(), ErrorOutOfBoundsAxises> {
        self.size.contains(pos)?;
        self.cells[pos.to_index(self.size.width)] = cell;
        Ok(())
    }

    fn get_cell_checked(&self, pos: Position) -> Result<&Cell, ErrorOutOfBoundsAxises> {
        self.size.contains(pos)?;
        Ok(&self.cells[pos.to_index(self.size.width)])
    }

    fn get_cell_mut_checked(&mut self, pos: Position) -> Result<&mut Cell, ErrorOutOfBoundsAxises> {
        self.size.contains(pos)?;
        Ok(&mut self.cells[pos.to_index(self.size.width)])
    }

    fn fill(&mut self, cell: Cell) {
        self.cells.fill(cell);
    }

    fn start_frame(&mut self) {
        self.clear();
    }
}

impl Drawer for FlatBuffer {
    fn draw(&mut self) -> impl Iterator<Item = DrawCall<'_>> {
        let width = self.size.width;
        let height = self.size.height;
        let cells = &self.cells;

        (0..height).flat_map(move |y| {
            (0..width).map(move |x| {
                let pos = Position { x, y };
                DrawCall {
                    pos,
                    cell: &cells[pos.to_index(width)],
                }
            })
        })
    }
}

impl ResizableBuffer for FlatBuffer {
    fn resize(&mut self, size: Size) {
        if self.size == size {
            return;
        }
        self.size = size;
        self.cells = vec![Cell::EMPTY; size.area() as usize];
    }
}

#[cfg(test)]
mod tests {

    use super::FlatBuffer;
    use crate::{buffer_resizing_tests, buffer_tests, drawer_buffer_tests};

    buffer_tests! {
        buffer_tests,
        FlatBuffer::new,
        FlatBuffer
    }

    drawer_buffer_tests! {
        drawer_tests,
        FlatBuffer::new,
        FlatBuffer
    }

    buffer_resizing_tests! {
        resizing_tests,
        FlatBuffer::new,
        FlatBuffer
    }
}
