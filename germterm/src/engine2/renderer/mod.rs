pub mod crossterm;

use crate::engine2::DrawCall;

/// Consumes an iterator of [`DrawCall`]s and writes them to a physical display.
///
/// A [`Renderer`] is the consumer-side counterpart to [`Drawer`](crate::engine2::buffer::Drawer),
/// which produces [`DrawCall`]s. The engine calls [`Renderer::start_frame`] before issuing any
/// draw calls, passes the full diff iterator to [`Renderer::render`], then calls
/// [`Renderer::end_frame`] to flush or finalise the output.
///
/// # Frame lifecycle
///
/// ```text
/// start_frame()
///     render(draw_calls)
/// end_frame()
/// ```
///
/// # Implementing for a new display target
///
/// Only [`render`](Renderer::render) is required. The `start_frame` and `end_frame` hooks have
/// empty default implementations and only need to be overridden when the target requires
/// setup or teardown work around the draw call stream (e.g. sending a flush command over SPI,
/// hiding a hardware cursor, or locking a framebuffer).
///
/// ```rust,ignore
/// struct MyRenderer { /* ... */ }
///
/// impl Renderer for MyRenderer {
///     type Error = std::io::Error;
///
///     fn render<'a>(
///         &mut self,
///         calls: impl Iterator<Item = DrawCall<'a>>,
///     ) -> Result<(), Self::Error> {
///         for DrawCall { pos, cell } in calls {
///             // write cell to the display at pos
///         }
///         Ok(())
///     }
/// }
/// ```
pub trait Renderer {
    /// The error type returned when a rendering operation fails.
    ///
    /// Use [`core::convert::Infallible`] for renderers backed by an in-memory framebuffer that
    /// can never fail, or an I/O / communication error type for renderers that write directly to
    /// hardware or a byte stream.
    type Error;

    fn init(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called once at the start of each frame, before any draw calls are issued.
    ///
    /// Use this hook to perform any per-frame setup required by the display target, such as
    /// hiding the terminal cursor or sending a display-on command.
    ///
    /// The default implementation does nothing.
    fn start_frame(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Renders all changed cells for the current frame.
    ///
    /// The iterator yields only the cells that differ from the previous frame (the diff), so
    /// implementations should not assume that every cell in the grid is present. Cells are
    /// yielded in an unspecified order.
    ///
    /// Each [`DrawCall`] contains:
    /// - `pos`: the grid-space [`Position`](crate::engine2::draw::Position) of the cell.
    /// - `cell`: a reference to the [`Cell`](crate::cell::Cell) to display at that position.
    fn render<'a>(&mut self, calls: impl Iterator<Item = DrawCall<'a>>) -> Result<(), Self::Error>;

    /// Called once at the end of each frame, after all draw calls have been issued.
    ///
    /// Use this hook to flush buffered output to the display (e.g. `stdout.flush()`, an SPI
    /// transaction commit, or a framebuffer swap).
    fn end_frame(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn restore(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
