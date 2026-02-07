# Contributing

Your quality contributions, however small, are greatly appreciated.

I highly recommend reading the [Rendering Pipeline - Architecture & Design](rendering-pipeline.md) document, as it goes in-depth into the internal workings of the renderer.

Thank you for considering contributing to the project!

## Brief module overview

- `engine.rs` - Public API glue module, home for `Engine` which groups the state of core internals like frame timing and rendering
- `frame.rs` - Internal module responsible for most of the rendering pipeline
- `fps_limiter.rs` - Frame timing logic
- `draw.rs` - All public API drawing functions should go here
- `rich_text.rs` - Everything related to stylized text
- `color.rs` - Anything to do with colors goes here, this includes conversions, operations, etc.
- `input.rs` - Anything and everything input related
- `particle.rs` - Anything related to the particle system
- `fps_counter.rs` - Small builtin FPS counter

## Branch workflow

Always work on a separate branch in order to keep `main` stable. Open a PR from your branch, and it will be merged once it passes the review.

## Code style

The codebase generally follows a procedural style, but this is not a hard rule. The main goal here is keeping the code explicit and unambiguous.

Try to minimize indirection and operate on data directly. This keeps logic explicit and predictable.

Nesting should generally top out at 4 levels, but this is not a hard rule. Just don't make a pyramid of doom!

### Functions over methods

The project prefers top-level functions over methods, unless a method reduces ambiguity.

```rust
// Top level function by default
start_frame(&mut engine)

// Example of where using a method would be preferred
//
// Here an `a()` function for retrieving the alpha
// value would have been more ambiguous
color.a()
```

### Linting and formatting

The project uses [Clippy](https://github.com/rust-lang/rust-clippy) for linting and [rustfmt](https://github.com/rust-lang/rustfmt) for formatting with the default config.

Please ensure the code is formatted using `cargo fmt` and has no warnings before committing.
