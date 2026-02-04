# 0.3.0

### 🍀 Added

- Added support for alpha blending with the default terminal background
- Added `Color::NO_COLOR` sentinel constant, which erases the specified fg or bg channel.
- Added `ColorRgb` struct, which specifies a color value without the alpha channel
- Added `erase_rect` function, which erases the `fg` and `bg` channels in the specified rectangular area
- Added `override_blending_color` function, which overrides the default auto-detected terminal background color used for alpha blending
- Added `get_fps` function

### 🛠 Changed

- Improved performance of alpha blending by switching to a LUT-based approach instead of floating-point arithmetic
- Terminal line wrapping now gets restored upon calling `exit_cleanup`

### ⚠ Breaking

- Moved `draw_fps_counter` function from the `fps_counter` module to `draw`
- Changed visibility of `FpsCounter` from `pub` to `pub(crate)`. Please use the new `get_fps` function if you need to read the current FPS
