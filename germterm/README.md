<img src="https://raw.githubusercontent.com/thehuglet/germterm/refs/heads/main/assets/octad-particles-preview.gif" width="32%"> <img src="https://raw.githubusercontent.com/thehuglet/germterm/refs/heads/main/assets/logo-middle.png" width="32%"> <img src="https://raw.githubusercontent.com/thehuglet/germterm/refs/heads/main/assets/twoxel-snake-preview.gif" width="32%">

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-gray?logo=rust&logoColor=orange)](https://www.rust-lang.org/)
[![Crate](https://img.shields.io/crates/v/germterm?logo=rust&color=orange)](https://crates.io/crates/germterm/)
[![Docs](https://img.shields.io/docsrs/germterm)](https://docs.rs/germterm)
[![License](https://img.shields.io/badge/license-MIT-blue)](https://mit-license.org/)
[![Build status](https://img.shields.io/github/actions/workflow/status/thehuglet/germterm/.github%2Fworkflows%2Frelease.yml)](https://github.com/thehuglet/germterm/actions)

</div>

A high-performance terminal graphics library.

It renders in real time, adds support for the alpha channel, adds multiple drawing formats and has a built-in particle system, all through a simple to use, `raylib` inspired API.

## Features

- Full alpha blending support with true color RGBA encoded colors
- A performance-first rendering pipeline with minimal allocations, especially optimized around CPU cache and LLVM Auto-Vectorization
- Supports drawing with depth using layers
- Supports multiple drawing formats, including subpixel drawing
- Built-in particle system with approximated physics
- Built-in FPS limiter with support for delta timing
- Crossplatform (through a [crossterm](https://github.com/crossterm-rs/crossterm) backend)
- Simple to use API

## Getting started

Add `germterm` as a dependency:

```plain_text,ignore
cargo add germterm
```

```rust,ignore
use germterm::{
    crossterm::event::{Event, KeyCode, KeyEvent},
    draw::{draw_fps_counter, draw_text},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
    layer::create_layer,
};
use std::io;

fn main() -> io::Result<()> {
    let mut engine = Engine::new(40, 20);
    let layer = create_layer(&mut engine, 0);

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
        draw_text(&mut engine, layer, 14, 9, "Hello, Ferris!");
        draw_fps_counter(&mut engine, layer, 0, 0);

        // End the frame
        end_frame(&mut engine)?;
    }

    // Restore terminal before exiting
    exit_cleanup(&mut engine)?;
    Ok(())
}
```

See the [examples](https://github.com/thehuglet/germterm/tree/main/examples) directory for more advanced examples.
