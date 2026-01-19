use germterm::{
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{Pos, Size, draw_rect, draw_text, draw_twoxel, fill_screen},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    fps_counter::draw_fps_counter,
    input::poll_input,
    rich_text::{Attributes, RichText},
};

use std::io;

pub const TERM_COLS: u16 = 80;
pub const TERM_ROWS: u16 = 24;

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("twoxel-drawing")
        .limit_fps(0);

    init(&mut engine)?;
    'game_loop: loop {
        start_frame(&mut engine);
        fill_screen(&mut engine, Color::DARK_GRAY);

        for event in poll_input() {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) = event
            {
                break 'game_loop;
            }
        }

        draw_twoxel(&mut engine, 10.0, 5.0, Color::LIME);
        draw_twoxel(&mut engine, 13.0, 5.0, Color::BLACK);

        draw_fps_counter(&mut engine, Pos::new(0, 0));

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
