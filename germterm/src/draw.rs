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
//! Each drawing function accepts a layer index, created by `layer::create_layer()`.
//! Layers are ordered and rendered by index and sorted from lowest to highest.
//! You can define as many layers as you need, they are fairly cheap.
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
    cell::CellFormat,
    color::Color,
    coord_space::{
        Position,
        blocktad::BlocktadPosition,
        native::{NativePosition, NativeSize},
        octad::OctadPosition,
        twoxel::TwoxelPosition,
    },
    engine::Engine,
    fps_counter::get_fps,
    frame::DrawCall,
    layer::LayerIndex,
    rich_text::{Attributes, RichText},
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

/// Draws text at the given coordinates.
///
/// Accepts either a `&str` or `String` or `RichText`.
///
/// # Example
/// ```rust,no_run
/// # use germterm::{draw::draw_text, layer::create_layer, engine::Engine};
/// let mut engine = Engine::new(40, 20);
/// let layer = create_layer(&mut engine, 0);
/// draw_text(&mut engine, layer, 2, 1, "Hello world!");
/// ```
pub fn draw_text(
    engine: &mut Engine,
    layer_index: LayerIndex,
    position: impl Into<NativePosition>,
    text: impl Into<RichText>,
) {
    let position: NativePosition = position.into();

    let layer = &mut engine.frame.layered_draw_queue[layer_index.0];
    let rich_text: RichText = text.into();
    let (x, y) = position.to_tuple();

    layer.0.push(DrawCall { rich_text, x, y });
}

/// Fills the entire screen with the specified [`Color`].
///
/// # Example
/// ```rust,no_run
/// # use germterm::{draw::fill_screen, layer::create_layer, engine::Engine, color::Color};
/// let mut engine = Engine::new(40, 20);
/// let layer = create_layer(&mut engine, 0);
/// fill_screen(&mut engine, layer, Color::PINK);
/// ```
pub fn fill_screen(engine: &mut Engine, layer_index: LayerIndex, color: Color) {
    let width: i16 = engine.frame.width as i16;
    let height: i16 = engine.frame.height as i16;

    draw_rect(engine, layer_index, (0, 0), (width, height), color);
}

/// Erases a rect area, restoring the default bg color and deleting the characters.
///
/// # Example
/// ```rust,no_run
/// # use germterm::{draw::erase_rect, layer::create_layer, engine::Engine};
/// let mut engine = Engine::new(40, 20);
/// let layer = create_layer(&mut engine, 0);
/// erase_rect(&mut engine, layer, 2, 2, 6, 3);
/// ```
pub fn erase_rect(
    engine: &mut Engine,
    layer_index: LayerIndex,
    position: impl Into<NativePosition>,
    size: impl Into<NativeSize>,
) {
    let position: NativePosition = position.into();
    let size: NativeSize = size.into();

    let row_text: String = " ".repeat(size.width as usize);
    let row_rich_text = RichText::new(row_text)
        .with_fg(Color::CLEAR)
        .with_bg(Color::CLEAR)
        .with_attributes(Attributes::NO_FG_COLOR | Attributes::NO_BG_COLOR);

    for row in 0..size.height {
        draw_text(
            engine,
            layer_index,
            position.offset_y(row),
            row_rich_text.clone(),
        )
    }
}

/// Draws a filled rect area with the specified [`Color`].
///
/// # Example
/// ```rust,no_run
/// # use germterm::{draw::draw_rect, layer::create_layer, engine::Engine, color::Color};
/// let mut engine = Engine::new(40, 20);
/// let layer = create_layer(&mut engine, 0);
/// draw_rect(&mut engine, layer, 10, 5, 20, 10, Color::CYAN);
/// ```
pub fn draw_rect(
    engine: &mut Engine,
    layer_index: LayerIndex,
    position: impl Into<NativePosition>,
    size: impl Into<NativeSize>,
    color: Color,
) {
    let position: NativePosition = position.into();
    let size: NativeSize = size.into();

    let row_text: String = " ".repeat(size.width as usize);
    let row_rich_text: RichText = RichText::new(&row_text)
        .with_fg(Color::CLEAR)
        .with_bg(color)
        .with_attributes(Attributes::NO_FG_COLOR);

    for row in 0..size.height {
        draw_text(
            engine,
            layer_index,
            position.offset_y(row),
            row_rich_text.clone(),
        )
    }
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
/// # use germterm::{draw::draw_octad, layer::create_layer, engine::Engine, color::Color};
/// let mut engine = Engine::new(40, 20);
/// let layer = create_layer(&mut engine, 0);
///
/// // The following octads would occupy the same cell,
/// // resulting in a merged octad cluster being drawn
/// draw_octad(&mut engine, layer, 3.0, 4.0, Color::YELLOW);
/// draw_octad(&mut engine, layer, 3.0, 4.5, Color::YELLOW);
/// ```
pub fn draw_octad(
    engine: &mut Engine,
    layer_index: LayerIndex,
    position: impl Into<OctadPosition>,
    color: Color,
) {
    let position: OctadPosition = position.into();

    let local_x = position.x.rem_euclid(2) as usize;
    let local_y = position.y.rem_euclid(4) as usize;

    // Offsets for each of the braille chars
    #[rustfmt::skip]
    const OFFSETS: [[usize; 4]; 2] = [
        [0, 1, 2, 6],
        [3, 4, 5, 7],
    ];

    let offset = OFFSETS[local_x][local_y];
    let braille_char: char = std::char::from_u32(0x2800 + (1 << offset)).unwrap();
    let rich_text: RichText = RichText::new(braille_char.to_string())
        .with_fg(color)
        .with_cell_format(CellFormat::Octad);

    draw_text(engine, layer_index, position.to_native(), rich_text);
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
/// # use germterm::{draw::draw_blocktad, layer::create_layer, engine::Engine, color::Color};
/// let mut engine = Engine::new(40, 20);
/// let layer = create_layer(&mut engine, 0);
///
/// // The following blocktads would occupy the same cell,
/// // resulting in a merged blocktad cluster being drawn
/// draw_blocktad(&mut engine, layer, 3.0, 4.0, Color::GREEN);
/// draw_blocktad(&mut engine, layer, 3.0, 4.5, Color::GREEN);
/// ```
///
/// /// # Notes
/// The characters may not show up on all fonts, as the [Symbols for Legacy Computing Supplement](https://en.wikipedia.org/wiki/Symbols_for_Legacy_Computing_Supplement)
/// Unicode block is a relatively recent addition. Use with caution.
pub fn draw_blocktad(
    engine: &mut Engine,
    layer_index: LayerIndex,
    position: impl Into<BlocktadPosition>,
    color: Color,
) {
    let position: BlocktadPosition = position.into();

    let local_x = position.x.rem_euclid(2) as usize;
    let local_y = position.y.rem_euclid(4) as usize;

    let offset: usize = local_y * 2 + local_x;
    let mask: usize = 1 << offset;
    let blocktad_char: char = BLOCKTAD_CHAR_LUT[mask];
    let rich_text: RichText = RichText::new(blocktad_char.to_string())
        .with_fg(color)
        .with_cell_format(CellFormat::Blocktad);

    draw_text(engine, layer_index, position.to_native(), rich_text);
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
/// # use germterm::{draw::draw_twoxel, layer::create_layer, engine::Engine, color::Color};
/// let mut engine = Engine::new(40, 20);
/// let layer = create_layer(&mut engine, 0);
///
/// // The following twoxels would occupy the same cell,
/// // resulting in a merged twoxel with independent colors
/// draw_twoxel(&mut engine, layer, 3.0, 4.0, Color::RED);
/// draw_twoxel(&mut engine, layer, 3.0, 4.5, Color::CYAN);
/// ```
pub fn draw_twoxel(
    engine: &mut Engine,
    layer_index: LayerIndex,
    position: impl Into<TwoxelPosition>,
    color: Color,
) {
    let position: TwoxelPosition = position.into();

    let local_y = position.y.rem_euclid(2) as usize;

    const BLOCKS: [char; 2] = ['▀', '▄'];
    let half_block = BLOCKS[local_y];

    let rich_text: RichText = RichText::new(half_block.to_string())
        .with_fg(color)
        .with_cell_format(CellFormat::Twoxel);

    draw_text(engine, layer_index, position.to_native(), rich_text)
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
/// # use germterm::{draw::draw_fps_counter, layer::create_layer, engine::Engine};
/// let mut engine = Engine::new(40, 20);
/// let layer = create_layer(&mut engine, 0);
/// draw_fps_counter(&mut engine, layer, 0, 0);
/// ```
pub fn draw_fps_counter(
    engine: &mut Engine,
    layer_index: LayerIndex,
    position: impl Into<NativePosition>,
) {
    let text: String = format!("FPS: {:2.0}", get_fps(engine));
    draw_text(engine, layer_index, position, text);
}
