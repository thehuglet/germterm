use germterm::{
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{Pos, Size, draw_rect, draw_text, fill_screen},
    engine::Engine,
    engine::{end_frame, exit_cleanup, init, start_frame},
    fps_counter::draw_fps_counter,
    input::poll_input,
    rich_text::{Attributes, RichText},
};

use std::io;

pub const TERM_COLS: u16 = 40;
pub const TERM_ROWS: u16 = 20;

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("blending-showcase")
        // Uncapped FPS
        .limit_fps(0);

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

        fill_screen(&mut engine, Color::new(60, 60, 70, 255));
        draw_fps_counter(&mut engine, Pos::new(0, 0));

        // --- Opaque drawing ---
        // Black square
        draw_rect(
            &mut engine,
            Pos::square(1, 4),
            Size::square(2, 2),
            Color::BLACK,
        );
        draw_text(
            &mut engine,
            Pos::square(1, 5),
            RichText::new("ab")
                .fg(Color::WHITE)
                .attributes(Attributes::BOLD),
        );

        // White square
        draw_rect(
            &mut engine,
            Pos::square(2, 3),
            Size::square(2, 2),
            Color::WHITE,
        );

        draw_text(
            &mut engine,
            Pos::square(2, 4),
            RichText::new("ab")
                .fg(Color::BLACK)
                .attributes(Attributes::BOLD),
        );

        // --- Background to background blending ---
        draw_rect(
            &mut engine,
            Pos::square(5, 4),
            Size::square(2, 2),
            Color::CYAN.with_alpha(127),
        );
        draw_rect(
            &mut engine,
            Pos::square(6, 3),
            Size::square(2, 2),
            Color::RED.with_alpha(127),
        );

        // --- Background over text blending ---
        draw_rect(
            &mut engine,
            Pos::square(9, 4),
            Size::square(2, 2),
            Color::WHITE,
        );
        draw_text(
            &mut engine,
            Pos::square(9, 4),
            RichText::new("1234").fg(Color::RED),
        );
        draw_rect(
            &mut engine,
            Pos::square(10, 3),
            Size::square(2, 2),
            Color::BLACK.with_alpha(155),
        );

        // --- Opaque background covering text (letters "yz" here) ---
        draw_rect(
            &mut engine,
            Pos::square(13, 4),
            Size::square(2, 2),
            Color::RED,
        );
        draw_text(
            &mut engine,
            Pos::square(13, 4),
            RichText::new("wxyz")
                .fg(Color::YELLOW)
                .attributes(Attributes::BOLD),
        );
        draw_rect(
            &mut engine,
            Pos::square(14, 3),
            Size::square(2, 2),
            Color::BLUE,
        );

        // --- "a" fg should blend with the `bg` here as there's no `fg` to blend with ---
        draw_text(
            &mut engine,
            Pos::square(17, 4),
            RichText::new("a")
                .fg(Color::RED.with_alpha(127))
                .attributes(Attributes::BOLD),
        );

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
