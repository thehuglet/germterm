use germterm::{
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{draw_text, draw_twoxel, fill_screen},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    fps_counter::draw_fps_counter,
    input::poll_input,
    rich_text::RichText,
};

use std::io;

pub const TERM_COLS: u16 = 80;
pub const TERM_ROWS: u16 = 24;

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("twoxel-tester")
        .limit_fps(240);

    init(&mut engine)?;
    'game_loop: loop {
        start_frame(&mut engine);
        fill_screen(&mut engine, Color::new(30, 30, 30, 255));

        for event in poll_input() {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) = event
            {
                break 'game_loop;
            }
        }

        // 1. Single twoxel (top)
        draw_text(&mut engine, 16, 3, RichText::new("1").fg(Color::DARK_GRAY));
        draw_twoxel(&mut engine, 16.0, 5.0, Color::RED);

        draw_twoxel(&mut engine, 16.0, 10.0, Color::RED.with_alpha(80));

        // 2. Single twoxel (bottom)
        draw_text(&mut engine, 18, 3, RichText::new("2").fg(Color::DARK_GRAY));
        draw_twoxel(&mut engine, 18.0, 5.5, Color::LIME);

        draw_twoxel(&mut engine, 18.0, 10.5, Color::LIME.with_alpha(80));

        // 3. Merged twoxels
        draw_text(&mut engine, 20, 3, RichText::new("3").fg(Color::DARK_GRAY));
        draw_twoxel(&mut engine, 20.0, 5.0, Color::RED);
        draw_twoxel(&mut engine, 20.0, 5.5, Color::LIME);

        draw_twoxel(&mut engine, 20.0, 10.0, Color::RED.with_alpha(80));
        draw_twoxel(&mut engine, 20.0, 10.5, Color::LIME.with_alpha(80));

        // 4. Merged twoxels + redrawing at the top position
        draw_text(&mut engine, 22, 3, RichText::new("4").fg(Color::DARK_GRAY));
        draw_twoxel(&mut engine, 22.0, 5.0, Color::RED);
        draw_twoxel(&mut engine, 22.0, 5.5, Color::LIME);
        draw_twoxel(&mut engine, 22.0, 5.0, Color::LIGHT_GRAY);

        draw_twoxel(&mut engine, 22.0, 10.0, Color::RED.with_alpha(80));
        draw_twoxel(&mut engine, 22.0, 10.5, Color::LIME.with_alpha(80));
        draw_twoxel(&mut engine, 22.0, 10.0, Color::LIGHT_GRAY.with_alpha(80));

        // 5. Merged twoxels + redrawing at the bottom position
        draw_text(&mut engine, 24, 3, RichText::new("5").fg(Color::DARK_GRAY));
        draw_twoxel(&mut engine, 24.0, 5.0, Color::RED);
        draw_twoxel(&mut engine, 24.0, 5.5, Color::LIME);
        draw_twoxel(&mut engine, 24.0, 5.5, Color::LIGHT_GRAY);

        draw_twoxel(&mut engine, 24.0, 10.0, Color::RED.with_alpha(80));
        draw_twoxel(&mut engine, 24.0, 10.5, Color::LIME.with_alpha(80));
        draw_twoxel(&mut engine, 24.0, 10.5, Color::LIGHT_GRAY.with_alpha(80));

        // 6. Same as 4. but reverse top & bottom
        draw_text(&mut engine, 26, 3, RichText::new("6").fg(Color::DARK_GRAY));
        draw_twoxel(&mut engine, 26.0, 5.5, Color::LIME);
        draw_twoxel(&mut engine, 26.0, 5.0, Color::RED);
        draw_twoxel(&mut engine, 26.0, 5.0, Color::LIGHT_GRAY);

        draw_twoxel(&mut engine, 26.0, 10.5, Color::LIME.with_alpha(80));
        draw_twoxel(&mut engine, 26.0, 10.0, Color::RED.with_alpha(80));
        draw_twoxel(&mut engine, 26.0, 10.0, Color::LIGHT_GRAY.with_alpha(80));

        // 7. Same as 5. but reverse top & bottom
        draw_text(&mut engine, 28, 3, RichText::new("7").fg(Color::DARK_GRAY));
        draw_twoxel(&mut engine, 28.0, 5.5, Color::LIME);
        draw_twoxel(&mut engine, 28.0, 5.0, Color::RED);
        draw_twoxel(&mut engine, 28.0, 5.5, Color::LIGHT_GRAY);

        draw_twoxel(&mut engine, 28.0, 10.5, Color::LIME.with_alpha(80));
        draw_twoxel(&mut engine, 28.0, 10.0, Color::RED.with_alpha(80));
        draw_twoxel(&mut engine, 28.0, 10.5, Color::LIGHT_GRAY.with_alpha(80));

        // --- Intended visual testers ---
        draw_text(
            &mut engine,
            5,
            5,
            RichText::new("Composed:").fg(Color::DARK_GRAY),
        );
        draw_text(
            &mut engine,
            5,
            7,
            RichText::new("Expected:").fg(Color::DARK_GRAY),
        );

        draw_text(
            &mut engine,
            4,
            10,
            RichText::new("Low alpha:").fg(Color::DARK_GRAY),
        );

        draw_text(
            &mut engine,
            14,
            6,
            RichText::new("-----------------").fg(Color::DARK_GRAY),
        );
        // 1.
        draw_text(&mut engine, 16, 7, RichText::new("▀").fg(Color::RED));
        // 2.
        draw_text(&mut engine, 18, 7, RichText::new("▄").fg(Color::LIME));
        // 3.
        draw_text(
            &mut engine,
            20,
            7,
            RichText::new("▀").fg(Color::RED).bg(Color::LIME),
        );
        // 4.
        draw_text(
            &mut engine,
            22,
            7,
            RichText::new("▀").fg(Color::LIGHT_GRAY).bg(Color::LIME),
        );
        // 5.
        draw_text(
            &mut engine,
            24,
            7,
            RichText::new("▀").fg(Color::RED).bg(Color::LIGHT_GRAY),
        );
        // 6.
        draw_text(
            &mut engine,
            26,
            7,
            RichText::new("▄").fg(Color::LIME).bg(Color::LIGHT_GRAY),
        );
        // 7.
        draw_text(
            &mut engine,
            28,
            7,
            RichText::new("▄").fg(Color::LIGHT_GRAY).bg(Color::RED),
        );

        draw_fps_counter(&mut engine, 0, 0);
        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
