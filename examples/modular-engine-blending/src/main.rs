use std::{io, ops::ControlFlow, time::Duration};

use germterm::{
    color::Color,
    core::{
        DisplayWidth, Engine,
        buffer::{
            Buffer, blended::BlendedBuffer, classic_diffed::ClassicDiffedBuffers,
            diffed::DiffedBuffers, flat::FlatBuffer, layered::LayeredBuffer,
        },
        draw::{Position, Size, gfx::text::draw_string},
        renderer::crossterm::CrosstermRenderer,
        timer::DefaultTimer,
    },
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    style::Style,
};

pub const TERM_SIZE: Size = Size::new(80, 25);

fn main() -> io::Result<()> {
    let mut engine = Engine::new(
        DefaultTimer::new(),
        ClassicDiffedBuffers::new(
            TERM_SIZE,
            LayeredBuffer::new(TERM_SIZE, |size| BlendedBuffer::new(FlatBuffer::new(size))),
        ),
        DisplayWidth::default(),
    );
    let mut renderer = CrosstermRenderer::new(io::stdout());

    engine.run(&mut renderer, |engine| -> ControlFlow<()> {
        for event in poll_input() {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) = event
            {
                return ControlFlow::Break(());
            }
        }

        let buf = engine.buffer_mut();
        let buf_size = buf.size();

        buf.current_buffer_mut().select_layer(0);

        for y in 0..buf_size.height {
            draw_string(
                buf,
                Position::new(0, y),
                &" ".repeat(buf_size.width as usize),
                Style::default().with_bg(Color::BLACK),
            );
        }

        draw_string(
            buf,
            Position::new(4, 5),
            "  ",
            Style::default().with_bg(Color::RED.with_alpha(127)),
        );

        draw_string(
            buf,
            Position::new(5, 5),
            "  ",
            Style::default().with_bg(Color::BLUE.with_alpha(127)),
        );
        ControlFlow::Continue(())
    })?;

    Ok(())
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
