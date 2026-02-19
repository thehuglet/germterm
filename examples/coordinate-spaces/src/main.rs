use germterm::{
    color::Color,
    coord_space::{Position, native::NativePosition, octad::OctadPosition, twoxel::TwoxelPosition},
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{draw_blocktad, draw_octad, draw_text, draw_twoxel, fill_screen},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
    layer::create_layer,
};

use std::io;

pub const TERM_COLS: u16 = 40;
pub const TERM_ROWS: u16 = 20;

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("blocktad-merging")
        .limit_fps(240);

    let layer = create_layer(&mut engine, 0);

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

        // Drawing the same shape in different coordinate spaces,
        // spread evenly by the same NativePosition y value
        for pos in [(1, 0), (0, 1), (2, 1), (1, 2)] {
            // Native
            draw_text(&mut engine, layer, pos, "A");

            // Twoxel
            let native_pos = NativePosition::new(0, 7).to_twoxel();
            let local_pos = TwoxelPosition::from(pos);
            draw_twoxel(&mut engine, layer, native_pos + local_pos, Color::WHITE);

            // Octad
            let native_pos = NativePosition::new(0, 14).to_octad();
            let local_pos = OctadPosition::from(pos);
            draw_octad(&mut engine, layer, native_pos + local_pos, Color::WHITE);
        }

        // Cross-coordinate space ops
        let start_pos = NativePosition::new(10, 5);
        let top_left = start_pos;
        let top_right = start_pos.offset_x(1);
        let bottom_left = start_pos.offset_y(1);
        let bottom_right = start_pos.offset_xy(1, 1);

        draw_twoxel(&mut engine, layer, top_left.to_twoxel(), Color::GREEN);
        draw_text(&mut engine, layer, top_right, "^");
        draw_text(&mut engine, layer, bottom_left, "^");
        draw_twoxel(&mut engine, layer, bottom_right.to_twoxel(), Color::GREEN);

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
