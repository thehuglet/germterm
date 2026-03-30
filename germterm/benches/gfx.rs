use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use germterm::{
    cell::Cell,
    color::Color,
    core::{
        DisplayWidth,
        buffer::flat::FlatBuffer,
        draw::gfx::normal,
        draw::gfx::text,
        draw::{Position, Rect, Size},
        widget::FrameContext,
    },
    style::{Attributes, Style},
};

fn test_cell() -> Cell {
    Cell::new(
        "#",
        Style::new(Color::WHITE, Color::BLACK, Attributes::empty()),
    )
}

fn make_fc(buf: &mut FlatBuffer) -> FrameContext<'_, FlatBuffer, f32> {
    FrameContext::new(0.0, 0.0, buf, DisplayWidth::default())
}

fn bench_vline(c: &mut Criterion) {
    let mut group = c.benchmark_group("draw_vline");
    let cell = test_cell();
    let dimensions = [(80, 24), (120, 40), (1920, 1080)];

    for (width, height) in dimensions {
        let sz = Size::new(width, height);

        group.bench_with_input(
            BenchmarkId::new("short", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                b.iter(|| {
                    normal::draw_vline(&mut buf, Position::ZERO, 10, black_box(&cell));
                    black_box(&mut buf);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("medium", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                let len = height / 2;
                b.iter(|| {
                    normal::draw_vline(&mut buf, Position::ZERO, len, black_box(&cell));
                    black_box(&mut buf);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("full_height", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                b.iter(|| {
                    normal::draw_vline(&mut buf, Position::ZERO, height, black_box(&cell));
                    black_box(&mut buf);
                })
            },
        );
    }

    group.finish();
}

fn bench_hline(c: &mut Criterion) {
    let mut group = c.benchmark_group("draw_hline");
    let cell = test_cell();
    let dimensions = [(80, 24), (120, 40), (1920, 1080)];

    for (width, height) in dimensions {
        let sz = Size::new(width, height);

        group.bench_with_input(
            BenchmarkId::new("short", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                b.iter(|| {
                    normal::draw_hline(&mut buf, Position::ZERO, 10, black_box(&cell));
                    black_box(&mut buf);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("medium", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                let len = width / 2;
                b.iter(|| {
                    normal::draw_hline(&mut buf, Position::ZERO, len, black_box(&cell));
                    black_box(&mut buf);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("full_width", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                b.iter(|| {
                    normal::draw_hline(&mut buf, Position::ZERO, width, black_box(&cell));
                    black_box(&mut buf);
                })
            },
        );
    }

    group.finish();
}

fn bench_line(c: &mut Criterion) {
    let mut group = c.benchmark_group("draw_line");
    let cell = test_cell();
    let dimensions = [(80, 24), (120, 40), (1920, 1080)];

    for (width, height) in dimensions {
        let sz = Size::new(width, height);

        group.bench_with_input(
            BenchmarkId::new("diagonal", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                let start = Position::new(0, 0);
                let end = Position::new(width - 1, height - 1);
                b.iter(|| {
                    normal::draw_line(&mut buf, start, end, black_box(&cell));
                    black_box(&mut buf);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("horizontal", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                let start = Position::new(0, 0);
                let end = Position::new(width - 1, 0);
                b.iter(|| {
                    normal::draw_line(&mut buf, start, end, black_box(&cell));
                    black_box(&mut buf);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("vertical", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                let start = Position::new(0, 0);
                let end = Position::new(0, height - 1);
                b.iter(|| {
                    normal::draw_line(&mut buf, start, end, black_box(&cell));
                    black_box(&mut buf);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("steep_diagonal", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                let start = Position::new(0, 0);
                let end = Position::new(width / 4, height - 1);
                b.iter(|| {
                    normal::draw_line(&mut buf, start, end, black_box(&cell));
                    black_box(&mut buf);
                })
            },
        );
    }

    group.finish();
}

fn bench_style(c: &mut Criterion) {
    let mut group = c.benchmark_group("draw_style");
    let style = Style::new(Color::RED, Color::BLUE, Attributes::BOLD);
    let dimensions = [(80, 24), (120, 40), (1920, 1080)];

    for (width, height) in dimensions {
        let sz = Size::new(width, height);

        group.bench_with_input(
            BenchmarkId::new("full_area", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                let rect = Rect::new(Position::ZERO, sz);
                b.iter(|| {
                    normal::draw_style(&mut buf, rect, style);
                    black_box(&mut buf);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("partial_area", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                let rect = Rect::new(
                    Position::new(width / 4, height / 4),
                    Size::new(width / 2, height / 2),
                );
                b.iter(|| {
                    normal::draw_style(&mut buf, rect, style);
                    black_box(&mut buf);
                })
            },
        );
    }

    group.finish();
}

fn bench_draw_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("draw_string");
    let dimensions = [(80, 24), (120, 40), (1920, 1080)];

    for (width, height) in dimensions {
        let sz = Size::new(width, height);

        group.bench_with_input(
            BenchmarkId::new("ascii_short", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                b.iter(|| {
                    let fc = make_fc(&mut buf);
                    text::draw_string(fc, Position::ZERO, "hello world");
                    black_box(&mut buf);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("ascii_medium", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                let text = "The quick brown fox jumps over the lazy dog";
                b.iter(|| {
                    let fc = make_fc(&mut buf);
                    text::draw_string(fc, Position::ZERO, text);
                    black_box(&mut buf);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("emoji", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                let text = "😀😎🎉🚀💡";
                b.iter(|| {
                    let fc = make_fc(&mut buf);
                    text::draw_string(fc, Position::ZERO, text);
                    black_box(&mut buf);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("cjk", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                let text = "中日韩文字测试";
                b.iter(|| {
                    let fc = make_fc(&mut buf);
                    text::draw_string(fc, Position::ZERO, text);
                    black_box(&mut buf);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("mixed_unicode", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                let text = "Hello 世界! 🌍 Test 中文 mixed 😀";
                b.iter(|| {
                    let fc = make_fc(&mut buf);
                    text::draw_string(fc, Position::ZERO, text);
                    black_box(&mut buf);
                })
            },
        );
    }

    group.finish();
}

fn bench_draw_text(c: &mut Criterion) {
    let mut group = c.benchmark_group("draw_text");
    let style = Style::new(Color::GREEN, Color::BLACK, Attributes::UNDERLINED);
    let dimensions = [(80, 24), (120, 40), (1920, 1080)];

    for (width, height) in dimensions {
        let sz = Size::new(width, height);

        group.bench_with_input(
            BenchmarkId::new("ascii_styled", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                let text = "styled text example";
                b.iter(|| {
                    let fc = make_fc(&mut buf);
                    text::draw_text(fc, Position::ZERO, text, style, u16::MAX);
                    black_box(&mut buf);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("unicode_styled", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                let text = "Styled 世界 🎨";
                b.iter(|| {
                    let fc = make_fc(&mut buf);
                    text::draw_text(fc, Position::ZERO, text, style, u16::MAX);
                    black_box(&mut buf);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("long_text_limited", format!("{}x{}", width, height)),
            &sz,
            |b, &sz| {
                let mut buf = FlatBuffer::new(sz);
                let text =
                    "This is a much longer piece of text that would normally exceed buffer width";
                b.iter(|| {
                    let fc = make_fc(&mut buf);
                    text::draw_text(fc, Position::ZERO, text, style, width);
                    black_box(&mut buf);
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_vline,
    bench_hline,
    bench_line,
    bench_style,
    bench_draw_string,
    bench_draw_text,
);
criterion_main!(benches);
