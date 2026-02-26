# 0.4.0

### 🍀 Added

- Added a new **Blocktad** drawing format. It works similarly to **Octads**, but uses blocks instead of braille characters

### ⚒️ Changed

- Reworked the layer API into a more memory safe form
- Improved stability and performance of the diffing step ([@airblast-dev](https://github.com/airblast-dev))
- Removed unnecessary allocations ([@airblast-dev](https://github.com/airblast-dev))

### 💥 Breaking

- `Layer` is now reserved for internal use, the new public API uses `LayerIndex` and the `create_layer()` factory
- Changed drawing function first args from `&mut Layer` to `&mut Engine` and `LayerIndex`
- Removed `Color::NO_COLOR` in favor of `Attributes::NO_FG_COLOR` and `Attributes::NO_BG_COLOR`

# 0.3.0

### 🍀 Added

- Added support for alpha blending with the default terminal background
- Added `Color::NO_COLOR` sentinel constant, which erases the specified fg or bg channel.
- Added `ColorRgb` struct, which specifies a color value without the alpha channel
- Added `erase_rect` function, which erases the `fg` and `bg` channels in the specified rectangular area
- Added `override_blending_color` function, which overrides the default auto-detected terminal background color used for alpha blending
- Added `get_fps` function

### ⚒️ Changed

- Improved performance of alpha blending by switching to a LUT-based approach instead of floating-point arithmetic
- Terminal line wrapping now gets restored upon calling `exit_cleanup`

### 💥 Breaking

- Moved `draw_fps_counter` function from the `fps_counter` module to `draw`
- Changed visibility of `FpsCounter` from `pub` to `pub(crate)`. Please use the new `get_fps` function if you need to read the current FPS
