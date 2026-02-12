use crate::{
    cell::Cell,
    engine2::{buffer::Drawer, draw::Size, Position},
};

use super::{Buffer, DrawCall};

#[derive(Clone, Copy, Debug)]
enum FrameOrder {
    CurrentOld = 0,
    OldCurrent = 1,
}

pub struct DiffedBuffers<Buf: Buffer> {
    size: Size,
    cells: [Buf; 2],
    frame_order: FrameOrder,
}

impl<Buf: Buffer> DiffedBuffers<Buf> {
    pub fn new(size: Size, buf1: Buf, buf2: Buf) -> Self {
        Self {
            size,
            cells: [buf1, buf2],
            frame_order: FrameOrder::CurrentOld,
        }
    }

    pub fn swap_frames(&mut self) {
        self.frame_order = match self.frame_order {
            FrameOrder::CurrentOld => FrameOrder::OldCurrent,
            FrameOrder::OldCurrent => FrameOrder::CurrentOld,
        };
    }
}

impl<Buf: Buffer> Buffer for DiffedBuffers<Buf> {
    fn set_cell(&mut self, pos: Position, cell: Cell) {
        let idx = 1 - self.frame_order as usize;
        self.cells[idx].set_cell(pos, cell);
    }

    fn get_cell(&self, pos: Position) -> &Cell {
        let idx = 1 - self.frame_order as usize;
        self.cells[idx].get_cell(pos)
    }

    fn get_cell_mut(&mut self, pos: Position) -> &mut Cell {
        let idx = 1 - self.frame_order as usize;
        self.cells[idx].get_cell_mut(pos)
    }

    fn start_frame(&mut self) {
        let idx = 1 - self.frame_order as usize;
        self.cells[idx].start_frame();
    }

    fn end_frame(&mut self) {
        let idx = 1 - self.frame_order as usize;
        self.cells[idx].end_frame();
    }

    fn resize(&mut self, size: Size) {
        self.cells.iter_mut().for_each(|b| b.resize(size))
    }
}

impl<Buf: Buffer> Drawer for DiffedBuffers<Buf> {
    fn draw(&mut self) -> impl Iterator<Item = DrawCall<'_>> {
        let width = self.size.width;
        let height = self.size.height;
        let order = 1 - self.frame_order as usize;
        let old_order = 1 - order;

        self.swap_frames();
        let current_buf = &self.cells[order];
        let old_buf = &self.cells[old_order];

        (0..height).flat_map(move |y| {
            (0..width).filter_map(move |x| {
                let pos = Position { x, y };
                let current_cell = current_buf.get_cell(pos);
                let old_cell = old_buf.get_cell(pos);

                if current_cell != old_cell {
                    Some(DrawCall {
                        cell: current_cell,
                        pos,
                    })
                } else {
                    None
                }
            })
        })
    }
}
