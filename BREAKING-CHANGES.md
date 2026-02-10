# 0.4.0

- `Layer` is now reserved for internal use, the new public API uses `LayerIndex` and the `create_layer()` factory
- Changed drawing function first args from `&mut Layer` to `&mut Engine` and `LayerIndex`
- Removed `Color::NO_COLOR` in favor of `Attributes::NO_FG_COLOR` and `Attributes::NO_BG_COLOR`

# 0.3.0

- Moved `draw_fps_counter` function from the `fps_counter` module to `draw`
- Changed visibility of `FpsCounter` from `pub` to `pub(crate)`. Please use the new `get_fps` function if you need to read the current FPS.

# 0.2.0

- Drawing functions now take a `&mut Layer` as the first argument instead of `&mut Engine` to account for the new layer system
