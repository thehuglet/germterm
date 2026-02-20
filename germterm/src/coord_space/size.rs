use std::{marker, ops};

use crate::coord_space::CoordinateSpace;

#[derive(Copy, Clone)]
pub struct Size<S: CoordinateSpace> {
    pub width: S::X,
    pub height: S::Y,
    _coord_space: marker::PhantomData<S>,
}

impl<S: CoordinateSpace> Size<S> {
    pub const fn new(x: S::X, y: S::Y) -> Self {
        Self {
            width: x,
            height: y,
            _coord_space: marker::PhantomData,
        }
    }

    pub fn offset_x(self, dx: S::X) -> Self {
        Self::new(self.width + dx, self.height)
    }

    pub fn offset_y(self, dy: S::Y) -> Self {
        Self::new(self.width, self.height + dy)
    }

    pub fn offset_xy(self, dx: S::X, dy: S::Y) -> Self {
        Self::new(self.width + dx, self.height + dy)
    }

    pub fn to_tuple(self) -> (S::X, S::Y) {
        (self.width, self.height)
    }
}

impl<S: CoordinateSpace> From<(S::X, S::Y)> for Size<S> {
    fn from(t: (S::X, S::Y)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl<S> ops::Add for Size<S>
where
    S: CoordinateSpace,
    S::X: ops::Add<Output = S::X>,
    S::Y: ops::Add<Output = S::Y>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl<S> ops::AddAssign for Size<S>
where
    S: CoordinateSpace,
    S::X: ops::AddAssign,
    S::Y: ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.width += rhs.width;
        self.height += rhs.height;
    }
}

impl<S> ops::Sub for Size<S>
where
    S: CoordinateSpace,
    S::X: ops::Sub<Output = S::X>,
    S::Y: ops::Sub<Output = S::Y>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.width - rhs.width, self.height - rhs.height)
    }
}

impl<S> ops::SubAssign for Size<S>
where
    S: CoordinateSpace,
    S::X: ops::SubAssign,
    S::Y: ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.width -= rhs.width;
        self.height -= rhs.height;
    }
}

impl<S, T> ops::Mul<T> for Size<S>
where
    S: CoordinateSpace,
    S::X: ops::Mul<T, Output = S::X>,
    S::Y: ops::Mul<T, Output = S::Y>,
    T: Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.width * rhs, self.height * rhs)
    }
}

impl<S, T> ops::MulAssign<T> for Size<S>
where
    S: CoordinateSpace,
    S::X: ops::MulAssign<T>,
    S::Y: ops::MulAssign<T>,
    T: Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.width *= rhs;
        self.height *= rhs;
    }
}

impl<S, T> ops::Div<T> for Size<S>
where
    S: CoordinateSpace,
    S::X: ops::Div<T, Output = S::X>,
    S::Y: ops::Div<T, Output = S::Y>,
    T: Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.width / rhs, self.height / rhs)
    }
}

impl<S, T> ops::DivAssign<T> for Size<S>
where
    S: CoordinateSpace,
    S::X: ops::DivAssign<T>,
    S::Y: ops::DivAssign<T>,
    T: Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.width /= rhs;
        self.height /= rhs;
    }
}

impl<S, T> ops::Rem<T> for Size<S>
where
    S: CoordinateSpace,
    S::X: ops::Rem<T, Output = S::X>,
    S::Y: ops::Rem<T, Output = S::Y>,
    T: Copy,
{
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Self::new(self.width % rhs, self.height % rhs)
    }
}

impl<S, T> ops::RemAssign<T> for Size<S>
where
    S: CoordinateSpace,
    S::X: ops::RemAssign<T>,
    S::Y: ops::RemAssign<T>,
    T: Copy,
{
    fn rem_assign(&mut self, rhs: T) {
        self.width %= rhs;
        self.height %= rhs;
    }
}
