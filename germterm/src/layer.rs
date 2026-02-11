use crate::{engine::Engine, frame::DrawCall};

/// Creates a layer that matches the dimensions of the screen
pub fn create_layer(engine: &mut Engine, index: usize) -> LayerIndex {
    let width = engine.frame.width as i16;
    let height = engine.frame.height as i16;
    create_layer_with_bounds(engine, index, 0, 0, width, height)
}

pub fn create_layer_with_bounds(
    engine: &mut Engine,
    index: usize,
    x: i16,
    y: i16,
    width: i16,
    height: i16,
) -> LayerIndex {
    // Resize layers vec if needed
    let layer_count: usize = engine.frame.layered_draw_queue.len();
    if layer_count < index + 1 {
        engine
            .frame
            .layered_draw_queue
            .resize_with(index + 1, || -> Layer { Layer::new(x, y, width, height) });
    }

    LayerIndex(index)
}

#[derive(Copy, Clone)]
pub struct LayerIndex(pub(crate) usize);

pub struct Layer {
    pub x: i16,
    pub y: i16,
    pub width: i16,
    pub height: i16,
    pub(crate) draw_queue: Vec<DrawCall>,
}

impl Layer {
    pub(crate) const fn new(x: i16, y: i16, width: i16, height: i16) -> Self {
        Layer {
            x,
            y,
            width,
            height,
            draw_queue: Vec::new(),
        }
    }
}
