use std::{io, ops::ControlFlow};

use germterm::{
    core::{
        Engine,
        buffer::{Buffer, diffed::DiffedBuffers, flat::FlatBuffer},
        draw::{Position, Size, gfx::text::draw_string},
        timer::DefaultTimer,
    },
    style::Style,
};

pub const TERM_COLS: u16 = 40;
pub const TERM_ROWS: u16 = 20;

fn main() -> io::Result<()> {
    germterm::core::run(
        &mut io::stdout(),
        Size::new(TERM_COLS, TERM_ROWS),
        update_loop,
    )
}

fn update_loop(
    engine: &mut Engine<DefaultTimer, DiffedBuffers<FlatBuffer>>,
) -> ControlFlow<Result<(), io::Error>> {
    let buf = engine.buffer_mut();

    draw_string(
        buf.layer(0),
        Position::new(1, 5),
        "--------------",
        Style::default(),
    );
    draw_string(
        buf.layer(0),
        Position::new(5, 5),
        "hello!",
        Style::default(),
    );

    ControlFlow::Continue(())
}
