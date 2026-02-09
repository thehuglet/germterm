use crate::{cell::Cell, engine2::buffer::Drawer};

use super::{Buffer, DrawCall, ErrorOutOfBoundsAxises};

#[derive(Clone, Copy, Debug)]
enum FrameOrder {
    CurrentOld = 0,
    OldCurrent = 1,
}

pub struct DiffedBufferPair<Buf: Buffer> {
    width: u16,
    height: u16,
    cells: [Buf; 2],
    frame_order: FrameOrder,
}

impl<Buf: Buffer> DiffedBufferPair<Buf> {
    pub fn new(width: u16, height: u16, buf1: Buf, buf2: Buf) -> Self {
        Self {
            width,
            height,
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

impl<Buf: Buffer> Buffer for DiffedBufferPair<Buf> {
    fn set_cell(&mut self, x: u16, y: u16, cell: Cell) {
        let idx = 1 - self.frame_order as usize;
        self.cells[idx].set_cell(x, y, cell);
    }

    fn set_cell_checked(
        &mut self,
        x: u16,
        y: u16,
        cell: Cell,
    ) -> Result<(), ErrorOutOfBoundsAxises> {
        let idx = 1 - self.frame_order as usize;
        self.cells[idx].set_cell_checked(x, y, cell)
    }

    fn get_cell(&self, x: u16, y: u16) -> &Cell {
        let idx = 1 - self.frame_order as usize;
        self.cells[idx].get_cell(x, y)
    }

    fn get_cell_mut(&mut self, x: u16, y: u16) -> &mut Cell {
        let idx = 1 - self.frame_order as usize;
        self.cells[idx].get_cell_mut(x, y)
    }

    fn get_cell_checked(&self, x: u16, y: u16) -> Result<&Cell, ErrorOutOfBoundsAxises> {
        let idx = 1 - self.frame_order as usize;
        self.cells[idx].get_cell_checked(x, y)
    }

    fn get_cell_mut_checked(
        &mut self,
        x: u16,
        y: u16,
    ) -> Result<&mut Cell, ErrorOutOfBoundsAxises> {
        let idx = 1 - self.frame_order as usize;
        self.cells[idx].get_cell_mut_checked(x, y)
    }

    fn start_frame(&mut self) {
        let idx = 1 - self.frame_order as usize;
        self.cells[idx].start_frame();
    }

    fn end_frame(&mut self) {
        let idx = 1 - self.frame_order as usize;
        self.cells[idx].end_frame();
    }
}

impl<Buf: Buffer> Drawer for DiffedBufferPair<Buf> {
    fn draw(&mut self) -> impl Iterator<Item = DrawCall<'_>> {
        let width = self.width;
        let height = self.height;
        let order = 1 - self.frame_order as usize;
        let old_order = 1 - order;

        self.swap_frames();
        let current_buf = &self.cells[order];
        let old_buf = &self.cells[old_order];

        (0..height).flat_map(move |y| {
            (0..width).filter_map(move |x| {
                let current_cell = current_buf.get_cell(x, y);
                let old_cell = old_buf.get_cell(x, y);

                if current_cell != old_cell {
                    Some(DrawCall {
                        cell: current_cell,
                        x,
                        y,
                    })
                } else {
                    None
                }
            })
        })
    }
}
