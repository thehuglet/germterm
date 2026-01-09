#[derive(Clone)]
pub struct PackedRgba(u32);

impl PackedRgba {
    pub const WHITE: Self = PackedRgba(0xFF_FF_FF_FF);
    pub const BLACK: Self = PackedRgba(0x00_00_00_FF);
    pub const RED: PackedRgba = PackedRgba(0xFF_00_00_FF);
    pub const GREEN: PackedRgba = PackedRgba(0x00_FF_00_FF);
    pub const BLUE: PackedRgba = PackedRgba(0x00_00_FF_FF);

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        PackedRgba(((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32))
    }
}
