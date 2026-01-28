use germterm::{
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{Layer, draw_text, fill_screen},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
};
use std::io;

fn main() -> io::Result<()> {
    let mut engine = Engine::new(40, 20).limit_fps(60);
    let mut layer = Layer::new(&mut engine, 0);

    init(&mut engine)?;

    'update_loop: loop {
        start_frame(&mut engine);

        for event in poll_input() {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) = event
            {
                break 'update_loop;
            }
        }

        // Draw contents
        fill_screen(&mut layer, Color::CLEAR);
        // fill_screen(&mut layer, Color::RED.with_alpha(30));
        draw_text(&mut layer, engine.game_time as i16, 9, "Hello world!");

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
