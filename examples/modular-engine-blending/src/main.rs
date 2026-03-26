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
    let term_bg: Color = match termbg::rgb(Duration::from_millis(100)) {
        Ok(rgb) => Color::new(rgb.r as u8, rgb.g as u8, rgb.b as u8, 255),
        Err(_) => Color::BLACK,
    };
    let mut engine = Engine::new(
        DefaultTimer::new(),
        LayeredBuffer::new(TERM_SIZE, |size| {
            BlendedBuffer::new(FlatBuffer::new(size)).with_bg_fallback(term_bg)
        })
        .with_bg_fallback(term_bg),
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

        draw_string(
            buf,
            Position::new(30, 5),
            "    ",
            Style::default().with_bg(Color::BLACK),
        );

        draw_string(
            buf,
            Position::new(30, 5),
            "test",
            Style::default().with_fg(Color::WHITE).with_bg(None),
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
