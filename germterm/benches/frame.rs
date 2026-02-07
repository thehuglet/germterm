use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use germterm::{cell::Cell, color::Color, frame::FramePair, rich_text::Attributes};

fn full_cell() -> Cell {
    Cell {
        ch: 'X',
        fg: Color::WHITE,
        bg: Color::BLACK,
        attributes: Attributes::empty(),
    }
}

fn bench_frame_diff(c: &mut Criterion) {
    let mut group = c.benchmark_group("Frame Diff");

    // Dimensions to test
    let dimensions = vec![
        (80, 24),     // Standard terminal
        (120, 40),    // Large terminal
        (1920, 1080), // Full HD (stress test, though unlikely for terminal)
    ];

    for (width, height) in dimensions {
        group.bench_with_input(
            BenchmarkId::new("No Changes", format!("{}x{}", width, height)),
            &(width, height),
            |b, &(w, h)| {
                let frame = FramePair::new(w as u16, h as u16);
                b.iter(|| {
                    for d in black_box(&frame).diff() {
                        black_box(d);
                    }
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Full Changes", format!("{}x{}", width, height)),
            &(width, height),
            |b, &(w, h)| {
                let mut frame = FramePair::new(w as u16, h as u16);

                // Fill current frame with something different from default
                // Default is ' ', NO_COLOR, NO_COLOR
                // We change char to 'A'
                let mut current = frame.current_mut();
                for i in 0..(w as usize * h as usize) {
                    current[i] = full_cell();
                }

                // Note: FramePair initialized with OldCurrent order, so 'current' is the second buffer.
                // The first buffer is 'old' and initialized to default.
                // So setting current to 'A' makes it different from old.

                b.iter(|| {
                    for d in black_box(&frame).diff() {
                        black_box(d);
                    }
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Alternating Changes", format!("{}x{}", width, height)),
            &(width, height),
            |b, &(w, h)| {
                let mut frame = FramePair::new(w as u16, h as u16);

                // Change every other cell
                let mut current = frame.current_mut();
                for i in 0..(w as usize * h as usize) {
                    if i % 2 == 0 {
                        current[i] = full_cell();
                    }
                }

                b.iter(|| {
                    for d in black_box(&frame).diff() {
                        black_box(d);
                    }
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_frame_diff);
criterion_main!(benches);
