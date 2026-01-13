use crate::{Engine, Pos, Size, color::Color, rich_text::RichText};

pub struct DrawCall {
    pub rich_text: RichText,
    pub x: i16,
    pub y: i16,
}

pub fn draw_text(engine: &mut Engine, pos: Pos, text: impl Into<RichText>) {
    let rich_text: RichText = text.into();
    engine.draw_queue.push(DrawCall {
        rich_text,
        x: pos.x,
        y: pos.y,
    });
}

pub fn draw_rect(engine: &mut Engine, pos: Pos, size: Size, color: Color) {
    let row_text: String = " ".repeat(size.width as usize);
    let row_rich_text: RichText = RichText::new(&row_text).fg(Color::CLEAR).bg(color);

    for row in 0..size.height {
        draw_text(engine, Pos::new(pos.x, pos.y + row), row_rich_text.clone())
    }
}

pub fn draw_fps_counter(engine: &mut Engine, pos: Pos) {
    draw_text(
        engine,
        pos,
        format!("FPS: {:2.0}", engine.fps_counter.fps_ema),
    );
}

pub fn fill_screen(engine: &mut Engine, color: Color) {
    let cols: i16 = engine.screen.cols as i16;
    let rows: i16 = engine.screen.rows as i16;
    draw_rect(engine, Pos::new(0, 0), Size::new(cols, rows), color);
}

pub fn draw_braille_dot(engine: &mut Engine, x: f32, y: f32, color: Color) {
    let cell_x: i16 = x.floor() as i16;
    let cell_y: i16 = y.floor() as i16;
    let cell_pos: Pos = Pos::new(cell_x, cell_y);

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
        _ => 0,
    };

    let braille_char: char = std::char::from_u32(0x2800 + (1 << offset)).unwrap();
    let rich_text: RichText = RichText::new(braille_char.to_string()).fg(color);

    draw_text(engine, cell_pos, rich_text);
}
