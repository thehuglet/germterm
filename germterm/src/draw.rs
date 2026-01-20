use crate::{color::Color, engine::Engine, rich_text::RichText};

pub fn fill_screen(engine: &mut Engine, color: Color) {
    let cols: i16 = engine.frame.cols as i16;
    let rows: i16 = engine.frame.rows as i16;
    internal::fill_screen(&mut engine.frame.draw_queue, cols, rows, color);
}

pub fn draw_text(engine: &mut Engine, x: i16, y: i16, text: impl Into<RichText>) {
    internal::draw_text(&mut engine.frame.draw_queue, x, y, text);
}

pub fn draw_rect(engine: &mut Engine, x: i16, y: i16, width: i16, height: i16, color: Color) {
    internal::draw_rect(&mut engine.frame.draw_queue, x, y, width, height, color);
}

pub fn draw_octad(engine: &mut Engine, x: f32, y: f32, color: Color) {
    internal::draw_octad(&mut engine.frame.draw_queue, x, y, color);
}

pub fn draw_twoxel(engine: &mut Engine, x: f32, y: f32, color: Color) {
    internal::draw_twoxel(&mut engine.frame.draw_queue, x, y, color);
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
