use crate::engine::{Engine, Pos, Size, color::Color, rich_text::RichText};

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

/// Draws a single braille dot at the sub-cell position (0..1) inside the 2x3 cell.
/// dot_x: 0 or 1 (left/right)
/// dot_y: 0..2 (top/middle/bottom)
pub fn draw_braille_dot(engine: &mut Engine, pos: Pos, dot_x: u8, dot_y: usize, alpha: f32) {
    // Braille unicode pattern: 0x2800 is empty, dots are bits 0..5 in 2x3 grid:
    // Bit positions:
    // 0 3
    // 1 4
    // 2 5
    let bit = match dot_y {
        0 => 0, // top row
        1 => 1, // middle row
        2 => 2, // bottom row
        _ => 0,
    } + if dot_x == 1 { 3 } else { 0 }; // right column offset

    let braille_char: char = std::char::from_u32(0x2800 + (1 << bit)).unwrap();
    let rich_text: RichText = RichText::new(braille_char.to_string()).fg(Color::new(
        255,
        255,
        255,
        (alpha.clamp(0.0, 1.0) * 255.0) as u8,
    ));

    draw_text(engine, pos, rich_text);
}

// /// Draw a single dot in 2x4 braille
// pub fn draw_braille_dot(engine: &mut Engine, pos: Pos, dot_x: u8, dot_y: usize) {
//     // Bits in 8-dot braille:
//     // 0 4
//     // 1 5
//     // 2 6
//     // 3 7
//     let bit = match dot_y {
//         0 => 0,
//         1 => 1,
//         2 => 2,
//         3 => 3,
//         _ => 0,
//     } + if dot_x == 1 { 4 } else { 0 };

//     let braille_char = std::char::from_u32(0x2800 + (1 << bit)).unwrap();

//     draw_text(engine, pos, braille_char.to_string());
// }
