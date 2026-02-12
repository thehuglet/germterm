use crate::{engine::Engine, frame::DrawCall};

pub fn create_layer(engine: &mut Engine, index: usize) -> LayerIndex {
    engine.max_layer_index = engine.max_layer_index.max(index);
    LayerIndex(index)
}

#[derive(Copy, Clone)]
pub struct LayerIndex(pub(crate) usize);

pub struct Layer(pub(crate) Vec<DrawCall>);

impl Layer {
    pub const fn new() -> Self {
        Layer(Vec::new())
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self::new()
    }
}
