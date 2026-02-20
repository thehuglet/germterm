use germterm::{
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{draw_fps_counter, draw_rect, draw_text, erase_rect},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
    layer::{LayerIndex, create_layer},
    rich_text::{Attributes, RichText},
};
use std::io;

pub const TERM_COLS: u16 = 80;
pub const TERM_ROWS: u16 = 25;

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("standard-blending")
        .limit_fps(0);

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
            (0, 0),
            (TERM_COLS as i16, 9),
            Color::CYAN.with_alpha(170),
        );
        erase_rect(&mut engine, layer, (0, 0), (TERM_COLS as i16, 9));

        draw_rect(
            &mut engine,
            layer,
            (0, 9),
            (TERM_COLS as i16, 8),
            Color::DARK_GREEN.with_alpha(127),
        );
        draw_rect(
            &mut engine,
            layer,
            (0, 17),
            (TERM_COLS as i16, 8),
            Color::DARK_GREEN,
        );

        let game_time = engine.game_time;
        draw_test_cases(&mut engine, layer, 0, 1, game_time);
        draw_test_cases(&mut engine, layer, 0, 9, game_time);
        draw_test_cases(&mut engine, layer, 0, 17, game_time);

        draw_test_cases(&mut engine, layer, 40, 1, game_time);
        draw_test_cases(&mut engine, layer, 40, 9, game_time);
        draw_test_cases(&mut engine, layer, 40, 17, game_time);

        // Should do nothing
        draw_rect(&mut engine, layer, (40, 0), (40, 25), Color::CLEAR);

        draw_fps_counter(&mut engine, layer, (0, 0));
        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}

fn draw_test_cases(engine: &mut Engine, layer: LayerIndex, x: i16, y: i16, game_time: f32) {
    // --- Opaque drawing ---
    // Black square
    draw_rect(engine, layer, (x + 2, y + 2), (4, 2), Color::BLACK);
    draw_text(
        engine,
        layer,
        (x + 2, y + 3),
        RichText::new("ab")
            .with_fg(Color::WHITE)
            .with_attributes(Attributes::BOLD),
    );

    // White square
    draw_rect(engine, layer, (x + 4, y + 1), (4, 2), Color::WHITE);

    draw_text(
        engine,
        layer,
        (x + 4, y + 2),
        RichText::new("ab")
            .with_fg(Color::BLACK)
            .with_attributes(Attributes::BOLD),
    );

    // --- Background to background blending ---
    draw_rect(
        engine,
        layer,
        (x + 10, y + 2),
        (4, 2),
        Color::CYAN.with_alpha(66),
    );
    draw_rect(
        engine,
        layer,
        (x + 12, y + 1),
        (4, 2),
        Color::RED.with_alpha(66),
    );

    // --- Background over text blending ---
    draw_rect(engine, layer, (x + 18, y + 2), (4, 2), Color::WHITE);
    draw_text(
        engine,
        layer,
        (x + 18, y + 2),
        RichText::new("1234").with_fg(Color::RED),
    );
    draw_rect(
        engine,
        layer,
        (x + 20, y + 1),
        (4, 2),
        Color::BLACK.with_alpha(155),
    );

    // --- Opaque background covering text (letters "yz" here) ---
    draw_rect(engine, layer, (x + 26, y + 2), (4, 2), Color::RED);
    draw_text(
        engine,
        layer,
        (x + 26, y + 2),
        RichText::new("wxyz")
            .with_fg(Color::GREEN)
            .with_attributes(Attributes::BOLD),
    );
    draw_rect(engine, layer, (x + 28, y + 1), (4, 2), Color::BLUE);

    // --- bottom red "abcd" fg should blend with the `bg` to form purple here as there's no `fg` to blend with ---
    draw_rect(engine, layer, (x + 34, y + 2), (4, 2), Color::BLUE);
    draw_text(
        engine,
        layer,
        (x + 34, y + 2),
        RichText::new("abcd")
            .with_fg(Color::RED)
            .with_attributes(Attributes::BOLD),
    );
    draw_text(
        engine,
        layer,
        (x + 34, y + 3),
        RichText::new("abcd")
            .with_fg(Color::RED.with_alpha(127))
            .with_attributes(Attributes::BOLD),
    );

    let freq: f32 = 1.0;
    let t: f32 = ((game_time * freq).sin() + 1.0) * 0.5;
    let t: f32 = t * 1.5 - 0.5;
    let t_byte: u8 = (t * 255.0).round().clamp(0.0, 255.0) as u8;

    // --- fg to fg blending test ---
    draw_text(
        engine,
        layer,
        (x + 2, y + 6),
        RichText::new("xxxx")
            .with_fg(Color::RED)
            .with_attributes(Attributes::BOLD),
    );
    draw_text(
        engine,
        layer,
        (x + 2, y + 6),
        RichText::new("o o")
            .with_fg(Color::GREEN.with_alpha(t_byte))
            .with_attributes(Attributes::BOLD),
    );

    // --- fg to no fg color + no bg color blending test ---
    draw_text(
        engine,
        layer,
        (x + 10, y + 6),
        RichText::new("boop")
            .with_fg(Color::VIOLET.with_alpha(t_byte))
            .with_attributes(Attributes::BOLD),
    );

    // --- Drawing opaque text with a solid bg ---
    draw_text(
        engine,
        layer,
        (x + 18, y + 6),
        RichText::new("bonk")
            .with_fg(Color::GREEN.with_alpha(t_byte))
            .with_bg(Color::DARK_GREEN)
            .with_attributes(Attributes::BOLD),
    );

    // --- Drawing opaque text with a solid bg ---
    draw_text(
        engine,
        layer,
        (x + 26, y + 6),
        RichText::new("bang")
            .with_fg(Color::RED.with_alpha(t_byte))
            .with_bg(Color::GREEN.with_alpha(30))
            .with_attributes(Attributes::BOLD),
    );

    // --- Drawing a clear rect ---
    draw_rect(engine, layer, (x + 2, y + 10), (4, 2), Color::CLEAR);

    // --- Drawing a translucent rect + opaque text on top of it ---
    draw_rect(engine, layer, (x + 2, y + 10), (4, 2), Color::CLEAR);

    // --- Drawing a translucent fg on top of an oscillating alpha fg
    draw_text(
        engine,
        layer,
        (x + 34, y + 6),
        RichText::new("xxxx")
            .with_fg(Color::RED.with_alpha(127))
            .with_attributes(Attributes::BOLD),
    );
    draw_text(
        engine,
        layer,
        (x + 34, y + 6),
        RichText::new("o o")
            .with_fg(Color::WHITE.with_alpha(t_byte))
            .with_attributes(Attributes::BOLD),
    );
}
