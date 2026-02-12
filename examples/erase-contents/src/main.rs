use germterm::{
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{draw_text, erase_rect, fill_screen},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
    layer::create_layer,
    rich_text::RichText,
};
use std::io;

const TERM_COLS: u16 = 40;
const TERM_ROWS: u16 = 20;

fn main() -> io::Result<()> {
    let mut engine = Engine::new(TERM_COLS, TERM_ROWS);
    let layer = create_layer(&mut engine, 0);

    init(&mut engine)?;

    'update_loop: loop {
        start_frame(&mut engine);
        fill_screen(&mut engine, layer, Color::BLACK);

        for event in poll_input() {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) = event
            {
                break 'update_loop;
            }
        }

        for y_offset in 0..TERM_ROWS {
            let text = if y_offset.is_multiple_of(2) {
                "-/"
            } else {
                "/-"
            };
            draw_text(
                &mut engine,
                layer,
                0,
                y_offset as i16,
                RichText::new(text.repeat(TERM_COLS as usize / 2))
                    .with_fg(Color::new(80, 80, 80, 255)),
            );
        }

        erase_rect(&mut engine, layer, 10, 5, 20, 10);

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
