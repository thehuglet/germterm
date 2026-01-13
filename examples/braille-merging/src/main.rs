use germterm::{
    Engine,
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{draw_braille_dot, fill_screen},
    end_frame, exit_cleanup, init,
    input::poll_input,
    start_frame,
};

use std::io;

pub const TERM_COLS: u16 = 40;
pub const TERM_ROWS: u16 = 20;

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("braille-merging")
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

        fill_screen(&mut engine, Color::BLACK);

        // Those 3 should all merge into a single braille char in the cell
        draw_braille_dot(&mut engine, 0.1, 0.0, Color::GREEN);
        draw_braille_dot(&mut engine, 0.9, 0.0, Color::GREEN);
        draw_braille_dot(&mut engine, 0.9, 0.9, Color::GREEN);

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
