use germterm::{
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{Layer, draw_blocktad, fill_screen},
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

        draw_blocktad(&mut layer, 0.0, 0.0, Color::RED);
        draw_blocktad(&mut layer, 0.5, 0.25, Color::RED);
        draw_blocktad(&mut layer, 0.5, 0.75, Color::RED);

        draw_blocktad(&mut layer, 1.0, 0.5, Color::CYAN);

        draw_blocktad(&mut layer, 2.5, 0.25, Color::GREEN);

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
