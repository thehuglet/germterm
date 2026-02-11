use germterm::{
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{draw_fps_counter, draw_rect, draw_text, fill_screen},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
    layer::{create_layer, create_layer_with_bounds},
};
use std::io;

pub const TERM_WIDTH: u16 = 40;
pub const TERM_HEIGHT: u16 = 20;

pub const HALF_WIDTH: u16 = TERM_WIDTH / 2;
pub const HALF_HEIGHT: u16 = TERM_HEIGHT / 2;

fn main() -> io::Result<()> {
    let mut engine = Engine::new(TERM_WIDTH, TERM_HEIGHT);

    let layer = create_layer(&mut engine, 0);
    let layer_partial =
        create_layer_with_bounds(&mut engine, 1, 10, 5, HALF_WIDTH as i16, HALF_HEIGHT as i16);

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

        fill_screen(&mut engine, layer, Color::RED);
        draw_rect(
            &mut engine,
            layer,
            0,
            0,
            TERM_WIDTH as i16,
            TERM_HEIGHT as i16,
            Color::GREEN,
        );

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
