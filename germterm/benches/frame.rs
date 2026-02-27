use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use germterm::{
    cell::Cell,
    color::Color,
    core::{
        buffer::{Buffer, Drawer, paired::PairedBuffer},
        draw::{Position, Size},
    },
};

fn full_cell() -> Cell {
    let mut cell = Cell::EMPTY;
    cell.ch = 'X';
    cell.fg = Color::WHITE;
    cell.bg = Color::BLACK;
    cell
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
            &Size::new(width, height),
            |b, &sz| {
                let mut buf = PairedBuffer::new(sz);
                b.iter(|| {
                    for d in black_box(&mut buf).draw() {
                        black_box(d);
                    }
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Full Changes", format!("{}x{}", width, height)),
            &Size::new(width, height),
            |b, &sz| {
                let mut buf = PairedBuffer::new(sz);

                for y in 0..sz.height {
                    for x in 0..sz.width {
                        buf.set_cell(Position::new(x, y), full_cell());
                    }
                }

                b.iter(|| {
                    for d in black_box(&mut buf).draw() {
                        black_box(d);
                    }
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Alternating Changes", format!("{}x{}", width, height)),
            &Size::new(width, height),
            |b, &sz| {
                let mut buf = PairedBuffer::new(sz);

                // Change every other cell
                for y in 0..sz.height {
                    for x in 0..sz.width {
                        if x * y % 2 == 0 {
                            buf.set_cell(Position::new(x, y), full_cell());
                        }
                    }
                }

                b.iter(|| {
                    for d in black_box(&mut buf).draw() {
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
