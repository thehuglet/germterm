[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-orange)](https://mit-license.org/)

# `germ-term`

Germterm is a lightweight, high-performance terminal graphics framework in Rust. It renders in real time and supports drawing with transparency through alpha blending, all through a simple to use API.

## Features

- Full alpha blending support, with true color RGBA encoded colors.
- High performance rendering pipeline with minimal allocations optimized for cache locality
- Supports multiple drawing formats
  - Standard - Full control over the `char`, `fg`, `bg` and `attributes`
  - Twoxel - Allows drawing 2 independent pixels inside a single terminal cell
  - Octad - Allows drawing in 8 distinct sub-pixel positions using braille characters
- Built-in particle system with approximated physics
- Built-in FPS limiter with access to delta timing
- Crossplatform ([crossterm](https://github.com/crossterm-rs/crossterm) backend)
- Simple to use API

## Getting started

See the [examples](examples/) directory for more advanced examples.

```rust
use std::io;

use germterm::{
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{draw_text, fill_screen},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    fps_counter::draw_fps_counter,
    input::poll_input,
};

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(40, 20).limit_fps(60);

    init(&mut engine)?;

    'update_loop: loop {
        for event in poll_input() {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) = event
            {
                break 'update_loop;
            }
        }

        start_frame(&mut engine);

        fill_screen(&mut engine, Color::BLACK);
        draw_text(&mut engine, 5, 5, "Hello world!");
        draw_fps_counter(&mut engine, 0, 0);

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
```

## Dependencies

This project only uses `crossterm`, `bitflags` and `rand` as its dependencies.
