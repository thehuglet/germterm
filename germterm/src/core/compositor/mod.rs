use crate::{cell::Cell, core::buffer::flat::FlatBuffer};

pub fn compose_cell(bottom: &mut Cell, top: &Cell) {
    *bottom = *top;
}

pub fn compose_buffers(bottom: &mut FlatBuffer, top: &FlatBuffer) {
    todo!();
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
