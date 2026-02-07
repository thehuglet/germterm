//! Drawing primitives and helpers.
//!
//! This module contains high-level drawing functions used
//! to enqueue render draw calls for the current frame.
//!
//! Every drawing function takes in a mutable reference to a [`Layer`], allowing
//! for fine control over the order at which drawn elements will be rendered.
//!
//! ## Layers
//!
//! A [`Layer`] is a lightweight handle identifying
//! a specific draw layer within an [`Engine`].
//!
//! Multiple layers may coexist and be drawn independently.
//!
//! Layers are sorted during rendering by their specified index, where index 1 would be drawn above index 0, etc.
//!
//! You can define as many layers as you need to, they are fairly cheap.
//!
//! ## Coordinate space
//!
//! All drawing functions operate in the same coordinate space, where `x` and `y` refer to terminal columns and rows.
//!
//! Some primitives (such as [`draw_octad`] and [`draw_twoxel`]) allow for a higher
//! sub-cell drawing precision using floating point coordinates combined with Unicode tricks.
//!
//! Most terminal emulators use an aspect ratio close to `1:2` (`width`:`height`).
//! This can cause issues when trying to implement eg. normalized diagonal movement,
//! since the movement along the y axis appears roughly twice as fast as movement along on the x axis.
//! A useful trick that looks good on most terminals is to apply a simple transformation right before drawing,
//! by either multiplying the x coordinate by `2` or dividing the y coordinate by `2`.
//!
//! ## Rendering model
//!
//! All functions in this module are immediate-mode.
//! They do not render directly, instead they enqueue draw calls
//! that are consumed by the engine at the end of the frame.

use crate::{
    color::Color, engine::Engine, fps_counter::get_fps, frame::DrawCall, rich_text::RichText,
};

#[rustfmt::skip]
pub(crate) static BLOCKTAD_CHAR_LUT: [char; 256] = [
    ' ', '𜺨', '𜺫', '🮂', '𜴀', '▘', '𜴁', '𜴂', '𜴃', '𜴄', '▝', '𜴅', '𜴆', '𜴇', '𜴈', '▀',
    '𜴉', '𜴊', '𜴋', '𜴌', '🯦', '𜴍', '𜴎', '𜴏', '𜴐', '𜴑', '𜴒', '𜴓', '𜴔', '𜴕', '𜴖', '𜴗',
    '𜴘', '𜴙', '𜴚', '𜴛', '𜴜', '𜴝', '𜴞', '𜴟', '🯧', '𜴠', '𜴡', '𜴢', '𜴣', '𜴤', '𜴥', '𜴦',
    '𜴧', '𜴨', '𜴩', '𜴪', '𜴫', '𜴬', '𜴭', '𜴮', '𜴯', '𜴰', '𜴱', '𜴲', '𜴳', '𜴴', '𜴵', '🮅',
    '𜺣', '𜴶', '𜴷', '𜴸', '𜴹', '𜴺', '𜴻', '𜴼', '𜴽', '𜴾', '𜴿', '𜵀', '𜵁', '𜵂', '𜵃', '𜵄',
    '▖', '𜵅', '𜵆', '𜵇', '𜵈', '▌', '𜵉', '𜵊', '𜵋', '𜵌', '▞', '𜵍', '𜵎', '𜵏', '𜵐', '▛',
    '𜵑', '𜵒', '𜵓', '𜵔', '𜵕', '𜵖', '𜵗', '𜵘', '𜵙', '𜵚', '𜵛', '𜵜', '𜵝', '𜵞', '𜵟', '𜵠',
    '𜵡', '𜵢', '𜵣', '𜵤', '𜵥', '𜵦', '𜵧', '𜵨', '𜵩', '𜵪', '𜵫', '𜵬', '𜵭', '𜵮', '𜵯', '𜵰',
    '𜺠', '𜵱', '𜵲', '𜵳', '𜵴', '𜵵', '𜵶', '𜵷', '𜵸', '𜵹', '𜵺', '𜵻', '𜵼', '𜵽', '𜵾', '𜵿',
    '𜶀', '𜶁', '𜶂', '𜶃', '𜶄', '𜶅', '𜶆', '𜶇', '𜶈', '𜶉', '𜶊', '𜶋', '𜶌', '𜶍', '𜶎', '𜶏',
    '▗', '𜶐', '𜶑', '𜶒', '𜶓', '▚', '𜶔', '𜶕', '𜶖', '𜶗', '▐', '𜶘', '𜶙', '𜶚', '𜶛', '▜',
    '𜶜', '𜶝', '𜶞', '𜶟', '𜶠', '𜶡', '𜶢', '𜶣', '𜶤', '𜶥', '𜶦', '𜶧', '𜶨', '𜶩', '𜶪', '𜶫',
    '▂', '𜶬', '𜶭', '𜶮', '𜶯', '𜶰', '𜶱', '𜶲', '𜶳', '𜶴', '𜶵', '𜶶', '𜶷', '𜶸', '𜶹', '𜶺',
    '𜶻', '𜶼', '𜶽', '𜶾', '𜶿', '𜷀', '𜷁', '𜷂', '𜷃', '𜷄', '𜷅', '𜷆', '𜷇', '𜷈', '𜷉', '𜷊',
    '𜷋', '𜷌', '𜷍', '𜷎', '𜷏', '𜷐', '𜷑', '𜷒', '𜷓', '𜷔', '𜷕', '𜷖', '𜷗', '𜷘', '𜷙', '𜷚',
    '▄', '𜷛', '𜷜', '𜷝', '𜷞', '▙', '𜷟', '𜷠', '𜷡', '𜷢', '▟', '𜷣', '▆', '𜷤', '𜷥', '█',
];

/// A handle to a drawing layer.
///
/// Passed into drawing functions, specifies to which layer the contents will be drawn.
///
/// Multiple layers can coexist and be passed around freely.
///
/// # Notes
/// Layer index can not be negative, the lowest layer index is 0.
///
/// # SAFETY
/// A [`Layer`] must not outlive the [`Engine`] it references due to the `&mut Engine` pointer.
#[derive(Clone, Copy)]
pub struct Layer {
    pub(crate) engine_ptr: *mut Engine,
    pub(crate) index: usize,
}

impl Layer {
    /// Creates a drawing layer handle.
    ///
    /// # Examples
    /// ```rust,no_run
    /// # use germterm::{draw::Layer, engine::Engine};
    /// let mut engine = Engine::new(40, 20);
    /// let mut layer = Layer::new(&mut engine, 0);
    /// ```
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn new(engine_ptr: *mut Engine, layer_index: usize) -> Self {
        let engine: &mut Engine = unsafe { &mut *engine_ptr };
        engine.max_layer_index = engine.max_layer_index.max(layer_index);

        Self {
            engine_ptr,
            index: layer_index,
        }
    }
}

/// Draws text at the given coordinates on a layer.
///
/// Accepts either a `&str` or `String`, which are converted into [`RichText`].
///
/// # Example
/// ```rust,no_run
/// # use germterm::{draw::{Layer, draw_text}, engine::Engine};
/// let mut engine = Engine::new(40, 20);
/// let mut layer = Layer::new(&mut engine, 0);
/// draw_text(&mut layer, 2, 1, "Hello world!");
/// ```
pub fn draw_text(layer: &mut Layer, x: i16, y: i16, text: impl Into<RichText>) {
    let engine: &mut Engine = unsafe { &mut *layer.engine_ptr };
    let draw_queue: &mut Vec<DrawCall> = &mut engine.frame.layered_draw_queue[layer.index];
    internal::draw_text(draw_queue, x, y, text);
}

/// Fills the entire screen with the specified [`Color`].
///
/// # Example
/// ```rust,no_run
/// # use germterm::{draw::{Layer, fill_screen}, engine::Engine, color::Color};
/// let mut engine = Engine::new(40, 20);
/// let mut layer = Layer::new(&mut engine, 0);
/// fill_screen(&mut layer, Color::PINK);
/// ```
pub fn fill_screen(layer: &mut Layer, color: Color) {
    let engine: &mut Engine = unsafe { &mut *layer.engine_ptr };
    let draw_queue: &mut Vec<DrawCall> = &mut engine.frame.layered_draw_queue[layer.index];
    let cols: i16 = engine.frame.cols as i16;
    let rows: i16 = engine.frame.rows as i16;
    internal::fill_screen(draw_queue, cols, rows, color);
}

/// Erases a rect area, restoring the default bg color and deleting the characters.
///
/// # Example
/// ```rust,no_run
/// # use germterm::{draw::{Layer, erase_rect}, engine::Engine};
/// let mut engine = Engine::new(40, 20);
/// let mut layer = Layer::new(&mut engine, 0);
/// erase_rect(&mut layer, 2, 2, 6, 3);
/// ```
pub fn erase_rect(layer: &mut Layer, x: i16, y: i16, width: i16, height: i16) {
    let engine: &mut Engine = unsafe { &mut *layer.engine_ptr };
    let draw_queue: &mut Vec<DrawCall> = &mut engine.frame.layered_draw_queue[layer.index];
    internal::erase_rect(draw_queue, x, y, width, height)
}

/// Draws a filled rect area with the specified [`Color`].
///
/// # Example
/// ```rust,no_run
/// # use germterm::{draw::{Layer, draw_rect}, engine::Engine, color::Color};
/// let mut engine = Engine::new(40, 20);
/// let mut layer = Layer::new(&mut engine, 0);
/// draw_rect(&mut layer, 10, 5, 20, 10, Color::CYAN);
/// ```
pub fn draw_rect(layer: &mut Layer, x: i16, y: i16, width: i16, height: i16, color: Color) {
    let engine: &mut Engine = unsafe { &mut *layer.engine_ptr };
    let draw_queue: &mut Vec<DrawCall> = &mut engine.frame.layered_draw_queue[layer.index];
    internal::draw_rect(draw_queue, x, y, width, height, color);
}

/// Draws a single octad at the specified sub-cell position.
///
/// A single octad is represented by a single [braille dot character](https://en.wikipedia.org/wiki/Braille_Patterns)
/// from the 8-dot set (⣿).
/// The character will be drawn in one of the 8 possible sub-positions of a cell,
/// based on the passed floating point coordinates.
///
/// The coordinate space is based on cols and rows (`x` and `y`), just like the rest of the drawing API.
///
/// When drawing multiple octads to the same cell, at differing sub-positions, the octads will merge into a single multi-dot character.
/// Merged octads possess a technical limitation of having to share the same `fg` color.
/// Because of this, the entire merged octad cluster inherits the `fg` color of the last drawn octad in the cell.
///
/// # Example
/// ```rust,no_run
/// # use germterm::{draw::{Layer, draw_octad}, engine::Engine, color::Color};
/// let mut engine = Engine::new(40, 20);
/// let mut layer = Layer::new(&mut engine, 0);
///
/// // The following octads would occupy the same cell,
/// // resulting in a merged octad cluster being drawn
/// draw_octad(&mut layer, 3.0, 4.0, Color::YELLOW);
/// draw_octad(&mut layer, 3.0, 4.5, Color::YELLOW);
/// ```
pub fn draw_octad(layer: &mut Layer, x: f32, y: f32, color: Color) {
    let engine: &mut Engine = unsafe { &mut *layer.engine_ptr };
    let draw_queue: &mut Vec<DrawCall> = &mut engine.frame.layered_draw_queue[layer.index];
    internal::draw_octad(draw_queue, x, y, color);
}

/// Draws a single blocktad at the specified sub-cell position.
///
/// Blocktads are represented by the 2x4 square blocky characters from the
/// [Symbols for Legacy Computing Supplement](https://en.wikipedia.org/wiki/Symbols_for_Legacy_Computing_Supplement) Unicode block.
/// The character will be drawn in one of the 8 possible sub-positions of a cell,
/// based on the passed floating point coordinates.
///
/// The coordinate space is based on cols and rows (`x` and `y`), just like the rest of the drawing API.
///
/// When drawing multiple blocktads to the same cell, at differing sub-positions, the blocktads will merge into a single character representing both.
/// Merged blocktads possess a technical limitation of having to share the same `fg` color.
/// Because of this, the entire merged blocktad cluster inherits the `fg` color of the last drawn blocktad in the cell.
///
/// # Example
/// ```rust,no_run
/// # use germterm::{draw::{Layer, draw_blocktad}, engine::Engine, color::Color};
/// let mut engine = Engine::new(40, 20);
/// let mut layer = Layer::new(&mut engine, 0);
///
/// // The following blocktads would occupy the same cell,
/// // resulting in a merged blocktad cluster being drawn
/// draw_blocktad(&mut layer, 3.0, 4.0, Color::GREEN);
/// draw_blocktad(&mut layer, 3.0, 4.5, Color::GREEN);
/// ```
///
/// /// # Notes
/// The characters may not show up on all fonts, as the [Symbols for Legacy Computing Supplement](https://en.wikipedia.org/wiki/Symbols_for_Legacy_Computing_Supplement)
/// Unicode block is a relatively recent addition. Use with caution.
pub fn draw_blocktad(layer: &mut Layer, x: f32, y: f32, color: Color) {
    let engine: &mut Engine = unsafe { &mut *layer.engine_ptr };
    let draw_queue: &mut Vec<DrawCall> = &mut engine.frame.layered_draw_queue[layer.index];
    internal::draw_blocktad(draw_queue, x, y, color);
}

/// Draws a single twoxel at the specified sub-cell position.
///
/// A single twoxel is represented by one of the half block characters (`▀` or `▄`) from the [Block Elements unicode block](https://en.wikipedia.org/wiki/Block_Elements).
///
/// /// The character will be drawn in one of the 2 possible vertical sub-positions of a cell,
/// based on the passed floating point coordinates.
///
/// The coordinate space is based on cols and rows (`x` and `y`), just like the rest of the drawing API.
///
/// When drawing a twoxel on top of an opposing twoxel occupying the same cell, both twoxels will be merged into the same cell.
/// Merged twoxels display their color fully independently on one another within the same cell.
/// This operation utilizes both the `fg` and `bg` channels, contrary to a single non-merged twoxel only utilizing the `fg` channel.
///
/// # Example
/// ```rust,no_run
/// # use germterm::{draw::{Layer, draw_twoxel}, engine::Engine, color::Color};
/// let mut engine = Engine::new(40, 20);
/// let mut layer = Layer::new(&mut engine, 0);
///
/// // The following twoxels would occupy the same cell,
/// // resulting in a merged twoxel with independent colors
/// draw_twoxel(&mut layer, 3.0, 4.0, Color::RED);
/// draw_twoxel(&mut layer, 3.0, 4.5, Color::CYAN);
/// ```
pub fn draw_twoxel(layer: &mut Layer, x: f32, y: f32, color: Color) {
    let engine: &mut Engine = unsafe { &mut *layer.engine_ptr };
    let draw_queue: &mut Vec<DrawCall> = &mut engine.frame.layered_draw_queue[layer.index];
    internal::draw_twoxel(draw_queue, x, y, color);
}

/// Draws the current FPS.
///
/// The retrieved value is an EMA (Exponential Moving Average).
///
/// This is purely a convenience helper that draws with the default style.
/// If you wish to display the FPS in a more stylized way, look into [`get_fps`].
///
/// # Example
/// ```rust,no_run
/// # use germterm::{draw::{Layer, draw_fps_counter}, engine::Engine};
/// let mut engine = Engine::new(40, 20);
/// let mut layer = Layer::new(&mut engine, 0);
/// draw_fps_counter(&mut layer, 0, 0);
/// ```
pub fn draw_fps_counter(layer: &mut Layer, x: i16, y: i16) {
    let engine: &mut Engine = unsafe { &mut *layer.engine_ptr };
    draw_text(layer, x, y, format!("FPS: {:2.0}", get_fps(engine)));
}

pub(crate) mod internal {
    use std::sync::Arc;

    use crate::{
        color::Color,
        draw::BLOCKTAD_CHAR_LUT,
        frame::DrawCall,
        rich_text::{Attributes, RichText},
    };

    pub fn fill_screen(draw_queue: &mut Vec<DrawCall>, cols: i16, rows: i16, color: Color) {
        draw_rect(draw_queue, 0, 0, cols, rows, color);
    }

    pub fn draw_text(draw_queue: &mut Vec<DrawCall>, x: i16, y: i16, text: impl Into<RichText>) {
        let rich_text: RichText = text.into();
        draw_queue.push(DrawCall { rich_text, x, y });
    }

    pub fn draw_rect(
        draw_queue: &mut Vec<DrawCall>,
        x: i16,
        y: i16,
        width: i16,
        height: i16,
        color: Color,
    ) {
        let row_text: String = " ".repeat(width as usize);
        let row_rich_text: RichText = RichText::new(&row_text).fg(Color::CLEAR).bg(color);

        for row in 0..height {
            draw_text(draw_queue, x, y + row, row_rich_text.clone())
        }
    }

    pub fn erase_rect(draw_queue: &mut Vec<DrawCall>, x: i16, y: i16, width: i16, height: i16) {
        let row_text: String = " ".repeat(width as usize);
        let row_rich_text: RichText = RichText {
            text: Arc::new(row_text),
            fg: Color::NO_COLOR,
            bg: Color::NO_COLOR,
            attributes: Attributes::empty(),
        };

        for row in 0..height {
            draw_text(draw_queue, x, y + row, row_rich_text.clone())
        }
    }

    pub fn draw_blocktad(draw_queue: &mut Vec<DrawCall>, x: f32, y: f32, color: Color) {
        let cell_x: i16 = x.floor() as i16;
        let cell_y: i16 = y.floor() as i16;

        let sub_x: usize = (((x - cell_x as f32) * 2.0).floor().clamp(0.0, 1.0)) as usize;
        let sub_y: usize = (((y - cell_y as f32) * 4.0).floor().clamp(0.0, 3.0)) as usize;

        let offset: usize = sub_y * 2 + sub_x;
        let mask: usize = 1 << offset;

        let blocktad_char: char = BLOCKTAD_CHAR_LUT[mask];

        let rich_text: RichText = RichText::new(blocktad_char.to_string())
            .fg(color)
            .attributes(Attributes::BLOCKTAD);

        draw_text(draw_queue, cell_x, cell_y, rich_text);
    }

    pub fn draw_octad(draw_queue: &mut Vec<DrawCall>, x: f32, y: f32, color: Color) {
        let cell_x: i16 = x.floor() as i16;
        let cell_y: i16 = y.floor() as i16;

        let sub_x: u8 = ((x - cell_x as f32) * 2.0).clamp(0.0, 1.0) as u8;
        let sub_y_float: f32 = (y - cell_y as f32) * 4.0;
        let sub_y: usize = sub_y_float.floor().clamp(0.0, 3.0) as usize;

        let offset: usize = match (sub_x, sub_y) {
            (0, 0) => 0,
            (0, 1) => 1,
            (0, 2) => 2,
            (0, 3) => 6,
            (1, 0) => 3,
            (1, 1) => 4,
            (1, 2) => 5,
            (1, 3) => 7,
            _ => panic!(
                "Octad sub-position ({sub_x}, {sub_y}) falls out of expected ranges (0..1, 0..3)"
            ),
        };

        let braille_char: char = std::char::from_u32(0x2800 + (1 << offset)).unwrap();
        let rich_text: RichText = RichText::new(braille_char.to_string())
            .fg(color)
            .attributes(Attributes::OCTAD);

        draw_text(draw_queue, cell_x, cell_y, rich_text);
    }

    pub fn draw_twoxel(draw_queue: &mut Vec<DrawCall>, x: f32, y: f32, color: Color) {
        let cell_x: i16 = x.floor() as i16;
        let cell_y: i16 = y.floor() as i16;

        let sub_y_float: f32 = (y - cell_y as f32) * 2.0;
        let sub_y: usize = sub_y_float.floor().clamp(0.0, 1.0) as usize;

        let half_block: char = match sub_y {
            0 => '▀',
            1 => '▄',
            _ => panic!("Twoxel 'sub_y': {sub_y} falls out of the expected 0..1 range"),
        };

        let rich_text: RichText = RichText::new(half_block.to_string())
            .fg(color)
            .attributes(Attributes::TWOXEL);

        draw_text(draw_queue, cell_x, cell_y, rich_text)
    }
}
