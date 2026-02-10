use germterm::{
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{draw_fps_counter, draw_rect, draw_text, draw_twoxel},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
    layer::{LayerIndex, create_layer},
    rich_text::RichText,
};
use std::io;

pub const TERM_COLS: u16 = 40;
pub const TERM_ROWS: u16 = 30;

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS).title("twoxel-tester");

    let layer = create_layer(&mut engine, 0);

    init(&mut engine)?;
    'game_loop: loop {
        start_frame(&mut engine);

        for event in poll_input() {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) = event
            {
                break 'game_loop;
            }
        }

        draw_rect(
            &mut engine,
            layer,
            0,
            9,
            TERM_COLS as i16,
            9,
            Color::BLACK.with_alpha(127),
        );
        draw_rect(&mut engine, layer, 0, 18, TERM_COLS as i16, 9, Color::BLACK);

        draw_test_case(&mut engine, layer, 15.0, 1.0);
        draw_test_case(&mut engine, layer, 15.0, 10.0);
        draw_test_case(&mut engine, layer, 15.0, 19.0);

        draw_fps_counter(&mut engine, layer, 0, 0);
        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}

fn draw_test_case(engine: &mut Engine, layer: LayerIndex, x: f32, y: f32) {
    let alpha_value: u8 = 60;

    // 1. Single twoxel (top)
    draw_text(
        engine,
        layer,
        x as i16,
        y as i16,
        RichText::new("1").with_fg(Color::DARK_GRAY),
    );

    draw_twoxel(engine, layer, x, y + 2.0, Color::RED);

    draw_twoxel(
        engine,
        layer,
        x,
        y + 4.0,
        Color::RED.with_alpha(alpha_value),
    );

    // 2. Single twoxel (bottom)
    draw_text(
        engine,
        layer,
        x as i16 + 2,
        y as i16,
        RichText::new("2").with_fg(Color::DARK_GRAY),
    );

    draw_twoxel(engine, layer, x + 2.0, y + 2.5, Color::GREEN);

    draw_twoxel(
        engine,
        layer,
        x + 2.0,
        y + 4.5,
        Color::GREEN.with_alpha(alpha_value),
    );

    // 3. Merged twoxels
    draw_text(
        engine,
        layer,
        x as i16 + 4,
        y as i16,
        RichText::new("3").with_fg(Color::DARK_GRAY),
    );

    draw_twoxel(engine, layer, x + 4.0, y + 2.0, Color::RED);
    draw_twoxel(engine, layer, x + 4.0, y + 2.5, Color::GREEN);

    draw_twoxel(
        engine,
        layer,
        x + 4.0,
        y + 4.0,
        Color::RED.with_alpha(alpha_value),
    );
    draw_twoxel(
        engine,
        layer,
        x + 4.0,
        y + 4.5,
        Color::GREEN.with_alpha(alpha_value),
    );

    // 4. Merged twoxels + redrawing at the top position
    draw_text(
        engine,
        layer,
        x as i16 + 6,
        y as i16,
        RichText::new("4").with_fg(Color::DARK_GRAY),
    );

    draw_twoxel(engine, layer, x + 6.0, y + 2.0, Color::RED);
    draw_twoxel(engine, layer, x + 6.0, y + 2.5, Color::GREEN);
    draw_twoxel(engine, layer, x + 6.0, y + 2.0, Color::LIGHT_GRAY);

    draw_twoxel(
        engine,
        layer,
        x + 6.0,
        y + 4.0,
        Color::RED.with_alpha(alpha_value),
    );
    draw_twoxel(
        engine,
        layer,
        x + 6.0,
        y + 4.5,
        Color::GREEN.with_alpha(alpha_value),
    );
    draw_twoxel(
        engine,
        layer,
        x + 6.0,
        y + 4.0,
        Color::LIGHT_GRAY.with_alpha(alpha_value),
    );

    // 5. Merged twoxels + redrawing at the bottom position
    draw_text(
        engine,
        layer,
        x as i16 + 8,
        y as i16,
        RichText::new("5").with_fg(Color::DARK_GRAY),
    );

    draw_twoxel(engine, layer, x + 8.0, y + 2.0, Color::RED);
    draw_twoxel(engine, layer, x + 8.0, y + 2.5, Color::GREEN);
    draw_twoxel(engine, layer, x + 8.0, y + 2.5, Color::LIGHT_GRAY);

    draw_twoxel(
        engine,
        layer,
        x + 8.0,
        y + 4.0,
        Color::RED.with_alpha(alpha_value),
    );
    draw_twoxel(
        engine,
        layer,
        x + 8.0,
        y + 4.5,
        Color::GREEN.with_alpha(alpha_value),
    );
    draw_twoxel(
        engine,
        layer,
        x + 8.0,
        y + 4.5,
        Color::LIGHT_GRAY.with_alpha(alpha_value),
    );

    // 6. Same as 4. but reverse top & bottom
    draw_text(
        engine,
        layer,
        x as i16 + 10,
        y as i16,
        RichText::new("6").with_fg(Color::DARK_GRAY),
    );

    draw_twoxel(engine, layer, x + 10.0, y + 2.5, Color::GREEN);
    draw_twoxel(engine, layer, x + 10.0, y + 2.0, Color::RED);
    draw_twoxel(engine, layer, x + 10.0, y + 2.0, Color::LIGHT_GRAY);

    draw_twoxel(
        engine,
        layer,
        x + 10.0,
        y + 4.5,
        Color::GREEN.with_alpha(alpha_value),
    );
    draw_twoxel(
        engine,
        layer,
        x + 10.0,
        y + 4.0,
        Color::RED.with_alpha(alpha_value),
    );
    draw_twoxel(
        engine,
        layer,
        x + 10.0,
        y + 4.0,
        Color::LIGHT_GRAY.with_alpha(alpha_value),
    );

    // 7. Same as 5. but reverse top & bottom
    draw_text(
        engine,
        layer,
        x as i16 + 12,
        y as i16,
        RichText::new("7").with_fg(Color::DARK_GRAY),
    );

    draw_twoxel(engine, layer, x + 12.0, y + 2.5, Color::GREEN);
    draw_twoxel(engine, layer, x + 12.0, y + 2.0, Color::RED);
    draw_twoxel(engine, layer, x + 12.0, y + 2.5, Color::LIGHT_GRAY);

    draw_twoxel(
        engine,
        layer,
        x + 12.0,
        y + 4.5,
        Color::GREEN.with_alpha(alpha_value),
    );
    draw_twoxel(
        engine,
        layer,
        x + 12.0,
        y + 4.0,
        Color::RED.with_alpha(alpha_value),
    );
    draw_twoxel(
        engine,
        layer,
        x + 12.0,
        y + 4.5,
        Color::LIGHT_GRAY.with_alpha(alpha_value),
    );

    // --- Intended visual testers ---
    draw_text(
        engine,
        layer,
        x as i16 - 10,
        y as i16 + 2,
        RichText::new("Composed:").with_fg(Color::DARK_GRAY),
    );

    draw_text(
        engine,
        layer,
        x as i16 - 11,
        y as i16 + 4,
        RichText::new("Low alpha:").with_fg(Color::DARK_GRAY),
    );

    draw_text(
        engine,
        layer,
        x as i16 - 10,
        y as i16 + 6,
        RichText::new("Expected:").with_fg(Color::DARK_GRAY),
    );

    draw_text(
        engine,
        layer,
        x as i16 - 2,
        y as i16 + 5,
        RichText::new("-----------------").with_fg(Color::DARK_GRAY),
    );
    // 1.
    draw_text(
        engine,
        layer,
        x as i16,
        y as i16 + 6,
        RichText::new("▀").with_fg(Color::RED),
    );
    // 2.
    draw_text(
        engine,
        layer,
        x as i16 + 2,
        y as i16 + 6,
        RichText::new("▄").with_fg(Color::GREEN),
    );
    // 3.
    draw_text(
        engine,
        layer,
        x as i16 + 4,
        y as i16 + 6,
        RichText::new("▀").with_fg(Color::RED).with_bg(Color::GREEN),
        // 4.
    );
    draw_text(
        engine,
        layer,
        x as i16 + 6,
        y as i16 + 6,
        RichText::new("▀")
            .with_fg(Color::LIGHT_GRAY)
            .with_bg(Color::GREEN),
    );
    // 5.
    draw_text(
        engine,
        layer,
        x as i16 + 8,
        y as i16 + 6,
        RichText::new("▀")
            .with_fg(Color::RED)
            .with_bg(Color::LIGHT_GRAY),
    );
    // 6.
    draw_text(
        engine,
        layer,
        x as i16 + 10,
        y as i16 + 6,
        RichText::new("▄")
            .with_fg(Color::GREEN)
            .with_bg(Color::LIGHT_GRAY),
    );
    // 7.
    draw_text(
        engine,
        layer,
        x as i16 + 12,
        y as i16 + 6,
        RichText::new("▄")
            .with_fg(Color::LIGHT_GRAY)
            .with_bg(Color::RED),
    );
}
