use crate::coord_space::{
    Blocktad, CoordinateSpace, Octad, Twoxel, position::Position, size::Size,
};

macro_rules! impl_marker_conversions {
    ($Vec2:ident, $x_field:ident, $y_field:ident) => {
        impl $Vec2<Native> {
            pub fn to_twoxel(self) -> $Vec2<Twoxel> {
                $Vec2::new(self.$x_field, self.$y_field * 2)
            }

            pub fn to_octad(self) -> $Vec2<Octad> {
                $Vec2::new(self.$x_field * 2, self.$y_field * 4)
            }

            pub fn to_blocktad(self) -> $Vec2<Blocktad> {
                $Vec2::new(self.$x_field * 2, self.$y_field * 4)
            }
        }
    };
}

#[derive(Copy, Clone)]
pub struct Native;

impl CoordinateSpace for Native {
    type X = i16;
    type Y = i16;
}

impl_marker_conversions!(Position, x, y);
impl_marker_conversions!(Size, width, height);
