# 0.2.0

- Drawing functions now take a `&mut Layer` as the first argument instead of `&mut Engine` to account for the new layer system

# 0.3.0

- Moved `draw_fps_counter` function from the `fps_counter` module to `draw`
- Changed visibility of `FpsCounter` from `pub` to `pub(crate)`. Please use the new `get_fps` function if you need to read the current FPS.
