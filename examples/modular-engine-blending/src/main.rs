use std::{io, ops::ControlFlow, time::Duration};

use germterm::{
    color::Color,
    core::{
        Engine,
        buffer::{Buffer, diffed::DiffedBuffers, flat::FlatBuffer},
        draw::{Position, Size, gfx::text::draw_string},
        timer::DefaultTimer,
    },
    crossterm::event::{self, Event, KeyCode, KeyEvent},
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
    for event in poll_input() {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            ..
        }) = event
        {
            return ControlFlow::Break(Ok(()));
        }
    }

    let buf = engine.buffer_mut();
    let buf_size = buf.size();

    for y in 0..buf_size.height {
        draw_string(
            buf.layer(0),
            Position::new(0, y),
            &" ".repeat(buf_size.width as usize),
            Style::default().with_bg(Color::BLACK),
        );
    }

    draw_string(
        buf.layer(0),
        Position::new(5, 5),
        "    ",
        Style::default().with_bg(Color::RED.with_alpha(90)),
    );

    draw_string(
        buf.layer(1),
        Position::new(5, 5),
        "    ",
        Style::default().with_bg(Color::BLUE.with_alpha(90)),
    );

    // draw_string(
    //     buf.layer(0),
    //     Position::new(1, 5),
    //     "--------------",
    //     Style::default().with_fg(Color::WHITE),
    // );

    ControlFlow::Continue(())
}

fn poll_input() -> impl Iterator<Item = Event> {
    std::iter::from_fn(|| {
        if event::poll(Duration::from_millis(0)).ok()? {
            event::read().ok()
        } else {
            None
        }
    })
}
