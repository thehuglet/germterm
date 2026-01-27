use germterm::{
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{Layer, draw_rect, draw_text, fill_screen},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    fps_counter::draw_fps_counter,
    input::poll_input,
    rich_text::{Attributes, RichText},
};

use std::io;

pub const TERM_COLS: u16 = 40;
pub const TERM_ROWS: u16 = 20;

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("standard-blending")
        .limit_fps(240);

    let mut layer = Layer::new(&mut engine, 0);

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

        fill_screen(&mut layer, Color::new(60, 60, 70, 255));

        // --- Opaque drawing ---
        // Black square
        draw_rect(&mut layer, 2, 4, 4, 2, Color::BLACK);
        draw_text(
            &mut layer,
            2,
            5,
            RichText::new("ab")
                .fg(Color::WHITE)
                .attributes(Attributes::BOLD),
        );

        // White square
        draw_rect(&mut layer, 4, 3, 4, 2, Color::WHITE);

        draw_text(
            &mut layer,
            4,
            4,
            RichText::new("ab")
                .fg(Color::BLACK)
                .attributes(Attributes::BOLD),
        );

        // --- Background to background blending ---
        draw_rect(&mut layer, 10, 4, 4, 2, Color::CYAN.with_alpha(127));
        draw_rect(&mut layer, 12, 3, 4, 2, Color::RED.with_alpha(127));

        // --- Background over text blending ---
        draw_rect(&mut layer, 18, 4, 4, 2, Color::WHITE);
        draw_text(&mut layer, 18, 4, RichText::new("1234").fg(Color::RED));
        draw_rect(&mut layer, 20, 3, 4, 2, Color::BLACK.with_alpha(155));

        // --- Opaque background covering text (letters "yz" here) ---
        draw_rect(&mut layer, 26, 4, 4, 2, Color::RED);
        draw_text(
            &mut layer,
            26,
            4,
            RichText::new("wxyz")
                .fg(Color::YELLOW)
                .attributes(Attributes::BOLD),
        );
        draw_rect(&mut layer, 28, 3, 4, 2, Color::BLUE);

        // --- bottom red "abcd" fg should blend with the `bg` to form purple here as there's no `fg` to blend with ---
        draw_rect(&mut layer, 34, 4, 4, 2, Color::BLUE);
        draw_text(
            &mut layer,
            34,
            4,
            RichText::new("abcd")
                .fg(Color::RED)
                .attributes(Attributes::BOLD),
        );
        draw_text(
            &mut layer,
            34,
            5,
            RichText::new("abcd")
                .fg(Color::RED.with_alpha(127))
                .attributes(Attributes::BOLD),
        );

        draw_fps_counter(&mut layer, 0, 0);
        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
