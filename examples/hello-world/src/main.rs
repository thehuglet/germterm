use germterm::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{draw_fps_counter, draw_text},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
    layer::create_layer,
};
use std::io;

fn main() -> io::Result<()> {
    let mut engine = Engine::new(40, 20);
    let layer = create_layer(&mut engine, 0);

    // Initialize engine and layers
    init(&mut engine)?;

    'update_loop: loop {
        // Start the frame
        start_frame(&mut engine);

        // 'q' to exit the program
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
        draw_text(&mut engine, layer, (14, 9), "Hello, Ferris!");
        draw_fps_counter(&mut engine, layer, (0, 0));

        // End the frame
        end_frame(&mut engine)?;
    }

    // Restore terminal before exiting
    exit_cleanup(&mut engine)?;
    Ok(())
}
