use std::collections::BTreeMap;

use crate::{
    cell::{Cell, CellFormat},
    color::Color,
    core::{
        DrawCall,
        buffer::{Buffer, Drawer, ResizableBuffer, flat::FlatBuffer},
        compositor::compose_cell,
        draw::{Position, Size},
    },
    style::Style,
};

pub struct LayeredBuffer<Buf, F>
where
    Buf: Buffer,
    F: Fn(Size) -> Buf,
{
    size: Size,
    layers: BTreeMap<isize, Buf>,
    selected_layer_index: isize,
    buf_factory: F,
    composed_buffer: FlatBuffer,
}

impl<Buf, F> LayeredBuffer<Buf, F>
where
    Buf: Buffer,
    F: Fn(Size) -> Buf,
{
    pub fn new(size: Size, buf_factory: F) -> Self {
        let mut layered_buffer = Self {
            size,
            layers: BTreeMap::new(),
            selected_layer_index: 0,
            buf_factory,
            composed_buffer: FlatBuffer::new(size),
        };
        layered_buffer.create_layer(0);

        layered_buffer
    }

    pub fn select_layer(&mut self, index: isize) {
        if !self.layers.contains_key(&index) {
            self.create_layer(index);
        }
        self.selected_layer_index = index;
    }

    #[inline]
    fn selected_layer(&self) -> &Buf {
        self.layers
            .get(&self.selected_layer_index)
            .expect("Selected layer is expected to exist.")
    }

    #[inline]
    fn selected_layer_mut(&mut self) -> &mut Buf {
        self.layers
            .get_mut(&self.selected_layer_index)
            .expect("Selected layer is expected to exist.")
    }

    #[inline]
    fn create_layer(&mut self, index: isize) {
        let size = self.size;
        let factory = &self.buf_factory;
        self.layers.insert(index, factory(size));
    }
}

impl<Buf, F> ResizableBuffer for LayeredBuffer<Buf, F>
where
    Buf: Buffer + ResizableBuffer,
    F: Fn(Size) -> Buf,
{
    fn resize(&mut self, size: Size) {
        self.size = size;
        self.composed_buffer.resize(size);
        for layer_buf in self.layers.values_mut() {
            layer_buf.resize(size);
        }
    }
}

impl<Buf, F> Buffer for LayeredBuffer<Buf, F>
where
    Buf: Buffer,
    F: Fn(Size) -> Buf,
{
    fn size(&self) -> Size {
        self.size
    }

    /// Sets a cell of a currently selected layer.
    fn set_cell_checked(
        &mut self,
        pos: Position,
        cell: Cell,
    ) -> Result<(), super::ErrorOutOfBoundsAxises> {
        self.selected_layer_mut().set_cell_checked(pos, cell)
    }

    fn get_cell_checked(&self, pos: Position) -> Result<&Cell, super::ErrorOutOfBoundsAxises> {
        self.selected_layer().get_cell_checked(pos)
    }

    fn get_cell_mut_checked(
        &mut self,
        pos: Position,
    ) -> Result<&mut Cell, super::ErrorOutOfBoundsAxises> {
        self.selected_layer_mut().get_cell_mut_checked(pos)
    }
}

impl<Buf, F> Drawer for LayeredBuffer<Buf, F>
where
    Buf: Buffer + Drawer,
    F: Fn(Size) -> Buf,
{
    fn draw(&mut self) -> impl Iterator<Item = DrawCall<'_>> {
        let width = self.size.width;
        let height = self.size.height;

        self.composed_buffer.fill(Cell::CLEAR);

        for buf in self.layers.values_mut() {
            for call in buf.draw() {
                let bottom = self.composed_buffer.get_cell_mut(call.pos);
                compose_cell(bottom, call.cell);
            }
        }

        let buf: &FlatBuffer = &self.composed_buffer;
        (0..height).flat_map(move |y| {
            (0..width).map(move |x| {
                let pos = Position::new(x, y);
                DrawCall {
                    cell: buf.get_cell(pos),
                    pos,
                }
            })
        })
    }
}
