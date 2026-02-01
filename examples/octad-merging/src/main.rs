use germterm::{
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{Layer, draw_octad, fill_screen},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
};

use std::io;

pub const TERM_COLS: u16 = 40;
pub const TERM_ROWS: u16 = 20;

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("octad-merging")
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

        fill_screen(&mut layer, Color::BLACK);

        // Those 3 should all merge into a single braille char in the cell
        // The color should be GREEN as it's set of the topmost merge's color value
        draw_octad(&mut layer, 0.1, 0.0, Color::RED);
        draw_octad(&mut layer, 0.9, 0.0, Color::BLUE);
        draw_octad(&mut layer, 0.9, 0.25, Color::GREEN);

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
