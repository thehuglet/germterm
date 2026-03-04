use crate::{
    cell::Cell,
    color::{Color, blend_source_over},
    core::buffer::flat::FlatBuffer,
    style::Attributes,
};

#[inline]
pub fn compose_cell(bottom: &mut Cell, top: &Cell) {
    // Cyan is a placeholder here, it should NEVER be used.
    //
    // If you ever see cyan where there shouldn't be cyan,
    // it means something clearly went wrong.
    let top_fg = top
        .style
        .fg()
        .unwrap_or(Color::CYAN)
        .to_premultiplied_alpha();
    let top_bg = top
        .style
        .bg()
        .unwrap_or(Color::CYAN)
        .to_premultiplied_alpha();

    if top.style.has_fg() {
        let bottom_fg: Color = bottom.style.fg().unwrap_or(Color::TRANSPARENT);
        let new_color: Color = blend_source_over(bottom_fg, top_fg);
        bottom.ch = top.ch;
        bottom.style = bottom.style.with_fg(new_color);
    }

    if top.style.has_bg() {
        let bottom_bg: Color = bottom.style.bg().unwrap_or(Color::TRANSPARENT);
        let new_color = blend_source_over(bottom_bg, top_bg);

        bottom.ch = top.ch;
        bottom.style = bottom.style.with_bg(new_color);
    }
}

#[inline]
pub fn compose_buffers(bottom_buf: &mut FlatBuffer, top_buf: &FlatBuffer) {
    for (bottom, top) in bottom_buf.cells_mut().zip(top_buf.cells()) {
        compose_cell(bottom, top);
    }
}

#[inline]
fn merge_octad(a: char, b: char) -> char {
    let mask_a = (a as u32) - 0x2800;
    let mask_b = (b as u32) - 0x2800;
    std::char::from_u32(0x2800 + (mask_a | mask_b)).unwrap()
}

#[inline]
fn merge_blocktad(a: char, b: char) -> char {
    todo!();
    // let mask_a = BLOCKTAD_CHAR_LUT
    //     .iter()
    //     .position(|&c| c == a)
    //     .expect("char not in BLOCKTAD LUT") as u8;
    // let mask_b = BLOCKTAD_CHAR_LUT
    //     .iter()
    //     .position(|&c| c == b)
    //     .expect("char not in BLOCKTAD LUT") as u8;

    // let merged_mask = mask_a | mask_b;

    // BLOCKTAD_CHAR_LUT[merged_mask as usize]
}
