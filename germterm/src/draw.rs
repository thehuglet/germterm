use crate::{color::Color, engine::Engine, frame::DrawCall, rich_text::RichText};

#[derive(Clone, Copy)]
pub struct Layer {
    pub(crate) engine_ptr: *mut Engine,
    pub(crate) index: usize,
}

impl Layer {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn new(engine_ptr: *mut Engine, layer_index: usize) -> Self {
        unsafe {
            let engine: &mut Engine = &mut *engine_ptr;
            engine.max_layer_index = engine.max_layer_index.max(layer_index);
        }

        Self {
            engine_ptr,
            index: layer_index,
        }
    }
}

// /// Get mutable reference to the layer in the draw queue, auto-creating layers if needed
// ///
// /// # Safety
// /// The unsafe shenanigans here are a result of wanting a clean, ergonomic API,
// /// and a direct result of multiple immutable references existing when creating multiple layers.
// /// We only ever access the `draw_queue` here by pushing to it, this is entirely safe.
// pub unsafe fn layer_mut(layer: &mut Layer) -> &mut Vec<DrawCall> {
//     let engine: &mut Engine = unsafe { &mut *layer.engine_ptr };
//     if engine.frame.draw_queue.len() <= layer.index {
//         engine
//             .frame
//             .draw_queue
//             .resize_with(layer.index + 1, Vec::new);
//     }
//     &mut engine.frame.draw_queue[layer.index]
// }

pub fn fill_screen(layer: &mut Layer, color: Color) {
    let engine: &mut Engine = unsafe { &mut *layer.engine_ptr };
    let draw_queue: &mut Vec<DrawCall> = &mut engine.frame.draw_queue[layer.index];
    let cols: i16 = engine.frame.cols as i16;
    let rows: i16 = engine.frame.rows as i16;
    internal::fill_screen(draw_queue, cols, rows, color);
}

pub fn draw_text(layer: &mut Layer, x: i16, y: i16, text: impl Into<RichText>) {
    let engine: &mut Engine = unsafe { &mut *layer.engine_ptr };
    let draw_queue: &mut Vec<DrawCall> = &mut engine.frame.draw_queue[layer.index];
    internal::draw_text(draw_queue, x, y, text);
}

pub fn draw_rect(layer: &mut Layer, x: i16, y: i16, width: i16, height: i16, color: Color) {
    let engine: &mut Engine = unsafe { &mut *layer.engine_ptr };
    let draw_queue: &mut Vec<DrawCall> = &mut engine.frame.draw_queue[layer.index];
    internal::draw_rect(draw_queue, x, y, width, height, color);
}

pub fn draw_octad(layer: &mut Layer, x: f32, y: f32, color: Color) {
    let engine: &mut Engine = unsafe { &mut *layer.engine_ptr };
    let draw_queue: &mut Vec<DrawCall> = &mut engine.frame.draw_queue[layer.index];
    internal::draw_octad(draw_queue, x, y, color);
}

pub fn draw_twoxel(layer: &mut Layer, x: f32, y: f32, color: Color) {
    let engine: &mut Engine = unsafe { &mut *layer.engine_ptr };
    let draw_queue: &mut Vec<DrawCall> = &mut engine.frame.draw_queue[layer.index];
    internal::draw_twoxel(draw_queue, x, y, color);
}

pub(crate) mod internal {
    use crate::{
        color::Color,
        frame::DrawCall,
        rich_text::{Attributes, RichText},
    };

    pub fn fill_screen(draw_queue: &mut Vec<DrawCall>, cols: i16, rows: i16, color: Color) {
        draw_rect(draw_queue, 0, 0, cols, rows, color);
    }

    pub fn draw_text(draw_queue: &mut Vec<DrawCall>, x: i16, y: i16, text: impl Into<RichText>) {
        let rich_text: RichText = text.into();
        draw_queue.push(DrawCall { rich_text, x, y });
    }

    pub fn draw_rect(
        draw_queue: &mut Vec<DrawCall>,
        x: i16,
        y: i16,
        width: i16,
        height: i16,
        color: Color,
    ) {
        let row_text: String = " ".repeat(width as usize);
        let row_rich_text: RichText = RichText::new(&row_text).fg(Color::BLACK).bg(color);

        for row in 0..height {
            draw_text(draw_queue, x, y + row, row_rich_text.clone())
        }
    }

    pub fn draw_octad(draw_queue: &mut Vec<DrawCall>, x: f32, y: f32, color: Color) {
        let cell_x: i16 = x.floor() as i16;
        let cell_y: i16 = y.floor() as i16;

        let sub_x: u8 = ((x - cell_x as f32) * 2.0).clamp(0.0, 1.0) as u8;
        let sub_y_float: f32 = (y - cell_y as f32) * 4.0;
        let sub_y: usize = sub_y_float.floor().clamp(0.0, 3.0) as usize;

        let offset: usize = match (sub_x, sub_y) {
            (0, 0) => 0,
            (0, 1) => 1,
            (0, 2) => 2,
            (0, 3) => 6,
            (1, 0) => 3,
            (1, 1) => 4,
            (1, 2) => 5,
            (1, 3) => 7,
            _ => panic!(
                "Octad sub-position ({sub_x}, {sub_y}) falls out of expected ranges (0..1, 0..3)"
            ),
        };

        let braille_char: char = std::char::from_u32(0x2800 + (1 << offset)).unwrap();
        let rich_text: RichText = RichText::new(braille_char.to_string())
            .fg(color)
            .attributes(Attributes::OCTAD);

        draw_text(draw_queue, cell_x, cell_y, rich_text);
    }

    pub fn draw_twoxel(draw_queue: &mut Vec<DrawCall>, x: f32, y: f32, color: Color) {
        let cell_x: i16 = x.floor() as i16;
        let cell_y: i16 = y.floor() as i16;

        let sub_y_float: f32 = (y - cell_y as f32) * 2.0;
        let sub_y: usize = sub_y_float.floor().clamp(0.0, 1.0) as usize;

        let half_block: char = match sub_y {
            0 => '▀',
            1 => '▄',
            _ => panic!("Twoxel 'sub_y': {sub_y} falls out of the expected 0..1 range"),
        };

        let rich_text: RichText = RichText::new(half_block.to_string())
            .fg(color)
            .attributes(Attributes::TWOXEL);

        draw_text(draw_queue, cell_x, cell_y, rich_text)
    }
}
