/// Demonstrates the [`Line`] and [`Span`] text widgets provided by germterm.
///
/// Layout (each bullet is one terminal row):
///
/// - Header bar: bold title + subtitle using two differently-styled Spans
/// - Foreground colors: one Span per named color, all on one Line
/// - Text attributes: Bold / Italic / Underlined / combined, each a Span
/// - Background colors: colored backgrounds with contrasting foreground text
/// - Base style: the Line's own style fills cells that no Span covers
/// - Animated: three Spans cycling through offset phases of an RGB wave
/// - Footer: dim help text with a highlighted key Span
///
/// Press `q` to exit.
use std::{io, ops::ControlFlow};

use germterm::{
    color::Color,
    core::{
        Engine,
        buffer::{Buffer, paired::PairedBuffer},
        draw::{Rect, Size},
        renderer::crossterm::CrosstermRenderer,
        timer::DefaultTimer,
        widget::text::{line::Line, span::Span},
    },
    span,
    style::{Attributes, Stylable, Style},
};

fn main() -> io::Result<()> {
    let (cols, rows) = germterm::crossterm::terminal::size()?;

    let mut engine = Engine::new(
        DefaultTimer::new(),
        PairedBuffer::new(Size::new(cols, rows)),
    );
    let mut renderer = CrosstermRenderer::new(io::stdout());

    engine.run(&mut renderer, move |engine| {
        use germterm::crossterm::event::{self, Event, KeyCode, KeyEvent};

        // Drain all pending events; quit on 'q'.
        while event::poll(std::time::Duration::ZERO).unwrap_or(false) {
            if let Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            })) = event::read()
            {
                return ControlFlow::Break(());
            }
        }

        let t = engine.total_time();
        let sz = engine.buffer().size();
        let (width, height) = (sz.width, sz.height);

        // Row 0: Header
        //
        // The Line's base style fills the full row with a navy background.
        // Two Spans override the foreground: bold yellow for the title and
        // light-gray for the subtitle.  Any cells after the last Span still
        // receive the navy background from the base style.
        {
            let navy = Color::new(20, 20, 60, 255);

            let spans = [
                span!("  Line & Span Demo  ")
                    .with_fg(Color::YELLOW)
                    .with_bold(true),
                span!("— germterm widget showcase").with_fg(Color::LIGHT_GRAY),
            ];
            let line = Line::new(&spans).with_bg(navy);
            engine.draw(Rect::from_xywh(0, 0, width, 1), line);
        }

        // Row 2: Foreground colors section header
        {
            let section = Style::new(Color::WHITE, None, Attributes::BOLD);
            engine.draw(
                Rect::from_xywh(0, 2, width, 1),
                Line::new([span!("  Foreground Colors").with_style(section)].as_slice()),
            );
        }

        // Row 3: One Span per named foreground color
        //
        // Each Span carries its own Style; the Line receives Style::EMPTY as
        // its base so it does not impose any color on gaps between spans.
        {
            let spans = [
                span!("  "),
                span!("Red ").with_fg(Color::RED),
                span!("Green ").with_fg(Color::GREEN),
                span!("Blue ").with_fg(Color::BLUE),
                span!("Yellow ").with_fg(Color::YELLOW),
                span!("Cyan ").with_fg(Color::CYAN),
                span!("Pink ").with_fg(Color::PINK),
                span!("Orange ").with_fg(Color::ORANGE),
                span!("Violet ").with_fg(Color::VIOLET),
                span!("Teal").with_fg(Color::TEAL),
            ];
            engine.draw(
                Rect::from_xywh(0, 3, width, 1),
                Line::new(spans.as_slice()),
            );
        }

        // Row 5: Text attributes section header
        {
            let section = Style::new(Color::WHITE, None, Attributes::BOLD);
            let spans = [span!("  Text Attributes").with_style(section)];
            engine.draw(
                Rect::from_xywh(0, 5, width, 1),
                Line::new(spans.as_slice()),
            );
        }

        // Row 6: Attribute samples
        //
        // The Line uses a white base style so every span inherits white text.
        // Each Span then only needs to specify its Attributes, demonstrating
        // how Line::style and Span::style merge: the Span wins on conflicts.
        {
            let spans = [
                span!("  "),
                span!("Bold  ").with_bold(true),
                span!("Italic  ").with_italic(true),
                span!("Underlined").with_underlined(true),
                span!("  "),
                span!("Bold+Italic").with_italic(true).with_bold(true),
                span!("  "),
                span!("All Three")
                    .with_bold(true)
                    .with_italic(true)
                    .with_underlined(true),
            ];
            engine.draw(
                Rect::from_xywh(0, 6, width, 1),
                Line::new(spans.as_slice()).with_fg(Color::WHITE),
            );
        }

        // Row 8: Background colors section header
        {
            let section = Style::new(Color::WHITE, None, Attributes::BOLD);
            let spans = [span!("  Background Colors").with_style(section)];
            engine.draw(
                Rect::from_xywh(0, 8, width, 1),
                Line::new(spans.as_slice()),
            );
        }

        // Row 9: Background color badges
        {
            let spans = [
                span!("  "),
                span!(" Red ").with_colors(Color::BLACK, Color::RED),
                span!("  "),
                span!(" Green ").with_colors(Color::BLACK, Color::GREEN),
                span!("  "),
                span!(" Blue ").with_colors(Color::WHITE, Color::BLUE),
                span!("  "),
                span!(" Yellow ").with_colors(Color::BLACK, Color::YELLOW),
                span!("  "),
                span!(" Cyan ").with_colors(Color::BLACK, Color::CYAN),
                span!("  "),
                span!(" Violet ").with_colors(Color::WHITE, Color::VIOLET),
            ];
            engine.draw(
                Rect::from_xywh(0, 9, width, 1),
                Line::new(spans.as_slice()),
            );
        }

        // Row 11: Animated wave section header
        {
            let section = Style::new(Color::WHITE, None, Attributes::BOLD);
            let spans = [span!("  Animated Spans").with_style(section)];
            engine.draw(
                Rect::from_xywh(0, 11, width, 1),
                Line::new(spans.as_slice()),
            );
        }

        // Row 12: RGB color wave
        //
        // Three Spans share the same block-character text but each gets its own
        // Style built from a phase-shifted sine wave, making the color sweep
        // from red → green → blue across the line over time.
        {
            let phase = t * 2.0;
            let tau_third = std::f32::consts::TAU / 3.0;

            // Build an RGB color from three sine waves offset by 120° each.
            let wave_fg = |offset: f32| {
                let r = (((phase + offset).sin() * 0.5 + 0.5) * 255.0) as u8;
                let g = (((phase + offset + tau_third).sin() * 0.5 + 0.5) * 255.0) as u8;
                let b = (((phase + offset + 2.0 * tau_third).sin() * 0.5 + 0.5) * 255.0) as u8;
                Color::new(r, g, b, 255)
            };

            let spans = [
                Span::new("  ".to_string()).unwrap(),
                span!("████████").with_fg(wave_fg(0.0)),
                span!("████████").with_fg(wave_fg(tau_third)),
                span!("████████").with_fg(wave_fg(2.0 * tau_third)),
                span!("  ← three Spans, phase-shifted RGB wave").with_fg(Color::DARK_GRAY),
            ];
            engine.draw(
                Rect::from_xywh(0, 12, width, 1),
                Line::new(spans.as_slice()),
            );
        }

        // Last row: Footer
        if height > 14 {
            let dim = Style::EMPTY.with_fg(Color::DARK_GRAY);
            let key = Style::EMPTY.with_fg(Color::YELLOW);
            let spans = [
                span!("  Press ").with_style(dim),
                span!("q").with_style(key),
                span!(" to quit").with_style(dim),
            ];
            engine.draw(
                Rect::from_xywh(0, height - 1, width, 1),
                Line::new(spans.as_slice()),
            );
        }

        ControlFlow::Continue(()) // keep running
    })?;

    Ok(())
}
