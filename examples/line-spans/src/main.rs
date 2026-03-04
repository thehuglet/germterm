/// Demonstrates the [`Line`], [`Span`], and [`Block`] widgets provided by germterm.
///
/// Each content section is wrapped in a [`Block`] with a different border style
/// and a [`Title`] drawn on the top edge:
///
/// - **Foreground colors** (rounded border): one Span per named color
/// - **Text attributes** (single border): Bold / Italic / Underlined / combined
/// - **Background colors** (double border): colored backgrounds with contrasting text
/// - **Animated spans** (bold border): three Spans cycling through RGB wave phases
///
/// A header bar and footer sit outside any block.
///
/// Press `q` to exit.
use std::{io, ops::ControlFlow};

use germterm::{
    color::Color,
    core::{
        DisplayWidth, Engine,
        buffer::{Buffer, paired::PairedBuffer},
        draw::{Rect, Size},
        renderer::crossterm::CrosstermRenderer,
        timer::{DefaultTimer, Delta},
        widget::{
            block::{Block, set::SimpleBorderSet, title::Title},
            text::line::Line,
        },
    },
    span,
    style::{Stylable, Style},
};

fn main() -> io::Result<()> {
    let (cols, rows) = germterm::crossterm::terminal::size()?;

    let mut engine = Engine::new(
        DefaultTimer::new(),
        PairedBuffer::new(Size::new(cols, rows)),
        DisplayWidth::default(),
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

        // Row 0: Header bar
        //
        // The Line's base style fills the full row with a navy background.
        // Two Spans override the foreground: bold yellow for the title and
        // light-gray for the subtitle.
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

        // Rows 2-4: Foreground Colors (rounded border)
        //
        // A Block with ROUNDED corners wraps one row of colored Spans.
        // The Title on the top edge replaces part of the border line.
        {
            let title_spans = [span!(" Foreground Colors ")
                .with_fg(Color::WHITE)
                .with_bold(true)];
            let titles = [Title::new(Line::new(title_spans.as_slice()))];
            let block = Block::<Delta, _>::new(SimpleBorderSet::ROUNDED).with_titles(&titles);
            engine.draw(Rect::from_xywh(0, 2, width, 3), block);

            let spans = [
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
                Rect::from_xywh(1, 3, width.saturating_sub(2), 1),
                Line::new(spans.as_slice()),
            );
        }

        // Rows 6-8: Text Attributes (single border)
        //
        // Each Span enables a different text attribute.  The Line's base style
        // supplies a white foreground so every span inherits white text.
        {
            let title_spans = [span!(" Text Attributes ")
                .with_fg(Color::WHITE)
                .with_bold(true)];
            let titles = [Title::new(Line::new(title_spans.as_slice()))];
            let block = Block::<f32, _>::new(SimpleBorderSet::SINGLE).with_titles(&titles);
            engine.draw(Rect::from_xywh(0, 6, width, 3), block);

            let spans = [
                span!("Bold  ").with_bold(true),
                span!("Italic  ").with_italic(true),
                span!("Underlined  ").with_underlined(true),
                span!("Bold+Italic  ").with_italic(true).with_bold(true),
                span!("All Three")
                    .with_bold(true)
                    .with_italic(true)
                    .with_underlined(true),
            ];
            engine.draw(
                Rect::from_xywh(1, 7, width.saturating_sub(2), 1),
                Line::new(spans.as_slice()).with_fg(Color::WHITE),
            );
        }

        // Rows 10-12: Background Colors (double border)
        //
        // Each Span uses `with_colors` to set both foreground and background,
        // producing colored badges separated by gaps.
        {
            let title_spans = [span!(" Background Colors ")
                .with_fg(Color::WHITE)
                .with_bold(true)];
            let titles = [Title::new(Line::new(title_spans.as_slice()))];
            let block = Block::<f32, _>::new(SimpleBorderSet::DOUBLE).with_titles(&titles);
            engine.draw(Rect::from_xywh(0, 10, width, 3), block);

            let spans = [
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
                Rect::from_xywh(1, 11, width.saturating_sub(2), 1),
                Line::new(spans.as_slice()),
            );
        }

        // Rows 14-16: Animated Spans (bold border)
        //
        // Three Spans share the same block-character text but each gets its own
        // color built from a phase-shifted sine wave, producing a cycling RGB
        // sweep over time.
        {
            let title_spans = [span!(" Animated Spans ")
                .with_fg(Color::WHITE)
                .with_bold(true)];
            let titles = [Title::new(Line::new(title_spans.as_slice()))];
            let block = Block::<f32, _>::new(SimpleBorderSet::BOLD).with_titles(&titles);
            engine.draw(Rect::from_xywh(0, 14, width, 3), block);

            let phase = t * 2.0;
            let tau_third = std::f32::consts::TAU / 3.0;

            let wave_fg = |offset: f32| {
                let r = (((phase + offset).sin() * 0.5 + 0.5) * 255.0) as u8;
                let g = (((phase + offset + tau_third).sin() * 0.5 + 0.5) * 255.0) as u8;
                let b = (((phase + offset + 2.0 * tau_third).sin() * 0.5 + 0.5) * 255.0) as u8;
                Color::new(r, g, b, 255)
            };

            let spans = [
                span!("████████").with_fg(wave_fg(0.0)),
                span!("████████").with_fg(wave_fg(tau_third)),
                span!("████████").with_fg(wave_fg(2.0 * tau_third)),
                span!("  ← phase-shifted RGB wave").with_fg(Color::DARK_GRAY),
            ];
            engine.draw(
                Rect::from_xywh(1, 15, width.saturating_sub(2), 1),
                Line::new(spans.as_slice()),
            );
        }

        // Last row: Footer
        if height > 17 {
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
