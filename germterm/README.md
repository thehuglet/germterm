<img src="https://github.com/thehuglet/germterm/blob/main/assets/octad-particles-preview.gif" width="32%"> <img src="https://github.com/thehuglet/germterm/blob/main/assets/logo-middle.png" width="32%"> <img src="https://github.com/thehuglet/germterm/blob/main/assets/twoxel-snake-preview.gif" width="32%">

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-gray?logo=rust&logoColor=orange)](https://www.rust-lang.org/)
[![Crate](https://img.shields.io/crates/v/germterm?logo=rust&color=orange)](https://crates.io/crates/germterm/)
[![License](https://img.shields.io/badge/license-MIT-blue)](https://mit-license.org/)
[![Build status](https://img.shields.io/github/actions/workflow/status/thehuglet/germterm/.github%2Fworkflows%2Frelease.yml)](https://github.com/thehuglet/germterm/actions)

</div>

A high-performance terminal graphics library.

It renders in real time, adds support for the alpha channel, adds multiple drawing formats and has a built-in particle system, all through a simple to use, `raylib` inspired API.

## Features

- Full alpha blending support with true color RGBA encoded colors
- A performance-first rendering pipeline with minimal allocations, especially optimized around CPU cache and LLVM Auto-Vectorization
- Supports drawing with depth using layers
- Supports multiple drawing formats
  - Standard - Full control over the `char`, `fg`, `bg` and `attributes`
  - Twoxel - Allows drawing 2 independent pixels inside a single terminal cell
  - Octad - Allows drawing in 8 distinct sub-pixel positions using braille characters
- Built-in particle system with approximated physics
- Built-in FPS limiter with support for delta timing
- Crossplatform (through a [crossterm](https://github.com/crossterm-rs/crossterm) backend)
- Simple to use API

## Getting started

See the [examples](https://github.com/thehuglet/germterm/tree/main/examples) directory for more advanced examples.

```rust,no_run
use germterm::{
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{Layer, draw_text, fill_screen, draw_fps_counter},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
};
use std::io;

fn main() -> io::Result<()> {
    let mut engine = Engine::new(40, 20);
    let mut layer = Layer::new(&mut engine, 0);

    // Initialize engine and layers
    init(&mut engine)?;

    'update_loop: loop {
        // Start the frame
        start_frame(&mut engine);

        // 'q' to exit the program
        for event in poll_input() {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) = event
            {
                break 'update_loop;
            }
        }

        // Draw contents
        draw_text(&mut layer, 14, 9, "Hello world!");
        draw_fps_counter(&mut layer, 0, 0);

        // End the frame
        end_frame(&mut engine)?;
    }

    // Restore terminal before exiting
    exit_cleanup(&mut engine)?;
    Ok(())
}
```
