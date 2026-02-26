use germterm::{
    color::Color,
    coord_space::{Position, native::NativePosition},
    draw::draw_text,
    engine::Engine,
    impl_coord_space_position_arithmetic,
    layer::LayerIndex,
    rich_text::RichText,
};

#[derive(Clone, Copy)]
pub struct BigBlockPosition {
    pub x: i16,
    pub y: i16,
}

impl Position for BigBlockPosition {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    fn x(&self) -> i16 {
        self.x
    }

    fn y(&self) -> i16 {
        self.y
    }
}

// Impls for basic arithmetic ops
impl_coord_space_position_arithmetic!(BigBlockPosition);

impl BigBlockPosition {
    pub fn to_native(self) -> NativePosition {
        NativePosition::new(self.x * 2, self.y)
    }
}

pub fn draw_big_block(
    engine: &mut Engine,
    layer: LayerIndex,
    position: BigBlockPosition,
    color: Color,
) {
    let rich_text = RichText::new(" ").with_bg(color);

    draw_text(engine, layer, position.to_native(), rich_text.clone());
    draw_text(
        engine,
        layer,
        position.to_native().offset_x(1),
        rich_text.clone(),
    );
}
