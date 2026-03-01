use crate::{
    cell::{Cell, CellFormat},
    color::Color,
    style::Attributes,
};

pub trait Compositor {}

pub struct DefaultCompositor {}

impl DefaultCompositor {
    #[inline]
    fn compose_cell(old: Cell, new: Cell) -> Cell {
        let both_ch_equal: bool = old.ch == new.ch;

        // Cell format related
        let new_twoxel: bool = new.format == CellFormat::Twoxel;
        let new_octad: bool = new.format == CellFormat::Octad;
        let new_blocktad: bool = new.format == CellFormat::Blocktad;

        let old_twoxel: bool = old.format == CellFormat::Twoxel;
        let old_octad: bool = old.format == CellFormat::Octad;
        let old_blocktad: bool = old.format == CellFormat::Blocktad;

        // Foreground related
        let new_fg_no_color: bool = !new.style.has_fg();
        let new_fg_invisible: bool = new.style.fg().unwrap_or(Color::CLEAR).a() == 0;
        let new_fg_opaque: bool = new.style.fg().unwrap_or(Color::CLEAR).a() == 255;
        let new_ch_invisible: bool = new.ch == ' ' || new_fg_invisible;
        let new_ch_translucent: bool = new.ch != ' ' && !new_fg_opaque && !new_fg_invisible;

        let old_fg_invisible: bool = old.style.fg().unwrap_or(Color::CLEAR).a() == 0;
        let old_fg_no_color: bool = !old.style.has_fg();
        let old_ch_invisible: bool = old.ch == ' ' || old_fg_invisible;

        // Background related
        let new_bg_no_color: bool = !new.style.has_bg();
        let new_bg_invisible: bool = new.style.bg().unwrap_or(Color::CLEAR).a() == 0;
        let new_bg_opaque: bool = new.style.bg().unwrap_or(Color::CLEAR).a() == 255;
        let new_bg_translucent: bool = !new_bg_opaque && !new_bg_invisible;

        let old_bg_no_color: bool = !old.style.has_bg();
        let old_bg_opaque: bool = old.style.bg().unwrap_or(Color::CLEAR).a() == 255;

        let new_fg: Color = new.style.fg().unwrap_or(Color::CLEAR);
        let old_fg: Color = old.style.fg().unwrap_or(Color::CLEAR);
        let new_bg: Color = new.style.bg().unwrap_or(Color::CLEAR);
        let old_bg: Color = old.style.bg().unwrap_or(Color::CLEAR);

        let (ch, style, format, mut attributes, fg, no_fg_color, bg, no_bg_color) = if new_twoxel {
            let (ch, format, attributes) = if old_twoxel && !new_fg_no_color {
                (old.ch, old.format, old.style.attributes())
            } else {
                (new.ch, new.format, new.style.attributes())
            };

            let (fg, no_fg_color) = if old_twoxel && both_ch_equal {
                (blend_source_over(old.fg, new.fg), false)
            } else if old_twoxel {
                (old.fg, false)
            } else if !old_bg_no_color {
                (blend_source_over(old.bg, new.fg), false)
            } else if new_fg_invisible {
                (default_blending_color, true)
            } else {
                (blend_source_over(default_blending_color, new.fg), false)
            };

            let (bg, no_bg_color) = if old_twoxel && both_ch_equal {
                (old.bg, false)
            } else if old_twoxel && old_bg_no_color {
                if new_fg_invisible {
                    (default_blending_color, true)
                } else {
                    (blend_source_over(default_blending_color, new.fg), false)
                }
            } else if old_twoxel {
                (blend_source_over(old.bg, new.fg), false)
            } else if old_bg_no_color {
                (old.bg, true)
            } else {
                (old.bg, false)
            };

            (ch, format, attributes, fg, no_fg_color, bg, no_bg_color)
        } else {
            // This branch handles the following cell formats: [Standard, Octad, Blocktad]
            let (ch, format, attributes) = if new_fg_no_color && new_bg_opaque && !old_ch_invisible
            {
                (new.ch, new.format, new.attributes)
            } else if new_blocktad && old_blocktad {
                (merge_blocktad(old.ch, new.ch), new.format, new.attributes)
            } else if new_octad && old_octad {
                (merge_octad(old.ch, new.ch), new.format, new.attributes)
            } else if new_ch_invisible && !new_bg_no_color {
                (old.ch, old.format, old.attributes)
            } else {
                (new.ch, new.format, new.attributes)
            };

            let (fg, no_fg_color) = if new_ch_invisible && new_bg_opaque {
                (Color::CLEAR, true)
            } else if new_ch_invisible {
                if new_bg_invisible && old_bg_no_color {
                    (old.fg, false)
                } else if new_bg_translucent {
                    (blend_source_over(old.fg, new.bg), false)
                } else {
                    (old.fg, old_fg_no_color)
                }
            } else if new_ch_translucent {
                let bottom_color = if !old_ch_invisible {
                    old.fg
                } else if old_bg_no_color && new_bg_invisible {
                    default_blending_color
                } else if old_bg_no_color && new_bg_translucent {
                    blend_source_over(default_blending_color, new.bg)
                } else if new_bg_opaque {
                    new.bg
                } else if old_bg_opaque && new_bg_translucent {
                    blend_source_over(old.bg, new.bg)
                } else if old_bg_opaque {
                    old.bg
                } else {
                    Color::CLEAR
                };
                (blend_source_over(bottom_color, new.fg), new_fg_no_color)
            } else {
                (new.fg, new_fg_no_color)
            };

            let (bg, no_bg_color) = if new_bg_no_color || (old_bg_no_color && new_bg_invisible) {
                (Color::CLEAR, true)
            } else if new_bg_invisible {
                (old.bg, false)
            } else if new_bg_translucent {
                let bottom_color = if old_bg_no_color {
                    default_blending_color
                } else {
                    old.bg
                };
                (blend_source_over(bottom_color, new.bg), false)
            } else {
                (new.bg, false)
            };

            (ch, format, attributes, fg, no_fg_color, bg, no_bg_color)
        };

        // Independent NO_{FG/BG}_COLOR patched into attributes
        attributes = (attributes & !(Attributes::NO_FG_COLOR | Attributes::NO_BG_COLOR))
            | (if no_fg_color {
                Attributes::NO_FG_COLOR
            } else {
                Attributes::empty()
            })
            | (if no_bg_color {
                Attributes::NO_BG_COLOR
            } else {
                Attributes::empty()
            });

        Cell { ch, style, format }
    }
}
