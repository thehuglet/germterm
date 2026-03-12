#[macro_export]
macro_rules! def_coord_space_conversion_fn {
    ($name:ident, $xclosure:expr, $yclosure:expr) => {
        pub fn $name<C>(position: C) -> C
        where
            C: CoordinateSpaceConvert,
            C::Scalar: Copy
                + From<i16>
                + std::ops::Mul<Output = C::Scalar>
                + std::ops::Div<Output = C::Scalar>,
        {
            position.convert($xclosure, $yclosure)
        }
    };
}

use crate::coord_space::Position;

pub trait CoordinateSpaceConvert: Sized {
    type Scalar;

    fn x(&self) -> Self::Scalar;
    fn y(&self) -> Self::Scalar;
    fn from_xy(x: Self::Scalar, y: Self::Scalar) -> Self;

    fn convert<F, G>(self, fx: F, fy: G) -> Self
    where
        F: FnOnce(Self::Scalar) -> Self::Scalar,
        G: FnOnce(Self::Scalar) -> Self::Scalar,
    {
        Self::from_xy(fx(self.x()), fy(self.y()))
    }
}

impl<T: Copy> CoordinateSpaceConvert for Position<T> {
    type Scalar = T;

    fn x(&self) -> T {
        self.x
    }
    fn y(&self) -> T {
        self.y
    }
    fn from_xy(x: T, y: T) -> Self {
        Position::new(x, y)
    }
}

#[rustfmt::skip]
pub mod builtin_conversion_defs {
    use super::*;

    // Native -> Other
    // ------------------------------
    def_coord_space_conversion_fn!(
        native_to_twoxel,
        |x| x,
        |y| y * C::Scalar::from(2)
    );
    def_coord_space_conversion_fn!(
        native_to_octad,
        |x| x * C::Scalar::from(2),
        |y| y * C::Scalar::from(4)
    );
    def_coord_space_conversion_fn!(
        native_to_blocktad,
        |x| x * C::Scalar::from(2),
        |y| y * C::Scalar::from(4)
    );

    // Twoxel -> Other
    // ------------------------------
    def_coord_space_conversion_fn!(
        twoxel_to_native,
        |x| x,
        |y| y / C::Scalar::from(2)
    );
    def_coord_space_conversion_fn!(
        twoxel_to_octad,
        |x| x * C::Scalar::from(2),
        |y| y * C::Scalar::from(2)
    );
    def_coord_space_conversion_fn!(
        twoxel_to_blocktad,
        |x| x * C::Scalar::from(2),
        |y| y * C::Scalar::from(2)
    );
}

// pub fn native_to_twoxel<C>(comp: C) -> C
// where
//     C: CoordinateSpaceConvert,
//     C::Item: Copy + ops::Mul<Output = C::Item> + From<i16>,
// {
//     let x = comp.x();
//     let y = comp.y();
//     let two = C::Item::from(2);
//     C::from_xy(x, y * two)
// }

// pub fn native_to_native<C>(comp: C) -> C
// where
//     C: CoordinateSpaceConvert,
//     C::Item: Copy + ops::Div<Output = C::Item> + From<i16>,
// {
//     let x = comp.x();
//     let y = comp.y();
//     let two = C::Item::from(2);
//     C::from_xy(x, y / two)
// }
