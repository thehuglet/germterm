use std::{marker, ops};

use crate::coord_space::CoordinateSpace;

#[derive(Copy, Clone)]
pub struct Position<S: CoordinateSpace> {
    pub x: S::X,
    pub y: S::Y,
    _coord_space: marker::PhantomData<S>,
}

impl<S: CoordinateSpace> Position<S> {
    pub const fn new(x: S::X, y: S::Y) -> Self {
        Self {
            x,
            y,
            _coord_space: marker::PhantomData,
        }
    }

    pub fn offset_x(self, dx: S::X) -> Self {
        Self::new(self.x + dx, self.y)
    }

    pub fn offset_y(self, dy: S::Y) -> Self {
        Self::new(self.x, self.y + dy)
    }

    pub fn offset_xy(self, dx: S::X, dy: S::Y) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }

    pub fn to_tuple(self) -> (S::X, S::Y) {
        (self.x, self.y)
    }
}

impl<S: CoordinateSpace> From<(S::X, S::Y)> for Position<S> {
    fn from(t: (S::X, S::Y)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl<S> ops::Add for Position<S>
where
    S: CoordinateSpace,
    S::X: ops::Add<Output = S::X>,
    S::Y: ops::Add<Output = S::Y>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<S> ops::AddAssign for Position<S>
where
    S: CoordinateSpace,
    S::X: ops::AddAssign,
    S::Y: ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<S> ops::Sub for Position<S>
where
    S: CoordinateSpace,
    S::X: ops::Sub<Output = S::X>,
    S::Y: ops::Sub<Output = S::Y>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<S> ops::SubAssign for Position<S>
where
    S: CoordinateSpace,
    S::X: ops::SubAssign,
    S::Y: ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<S, T> ops::Mul<T> for Position<S>
where
    S: CoordinateSpace,
    S::X: ops::Mul<T, Output = S::X>,
    S::Y: ops::Mul<T, Output = S::Y>,
    T: Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<S, T> ops::MulAssign<T> for Position<S>
where
    S: CoordinateSpace,
    S::X: ops::MulAssign<T>,
    S::Y: ops::MulAssign<T>,
    T: Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<S, T> ops::Div<T> for Position<S>
where
    S: CoordinateSpace,
    S::X: ops::Div<T, Output = S::X>,
    S::Y: ops::Div<T, Output = S::Y>,
    T: Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<S, T> ops::DivAssign<T> for Position<S>
where
    S: CoordinateSpace,
    S::X: ops::DivAssign<T>,
    S::Y: ops::DivAssign<T>,
    T: Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<S, T> ops::Rem<T> for Position<S>
where
    S: CoordinateSpace,
    S::X: ops::Rem<T, Output = S::X>,
    S::Y: ops::Rem<T, Output = S::Y>,
    T: Copy,
{
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Self::new(self.x % rhs, self.y % rhs)
    }
}

impl<S, T> ops::RemAssign<T> for Position<S>
where
    S: CoordinateSpace,
    S::X: ops::RemAssign<T>,
    S::Y: ops::RemAssign<T>,
    T: Copy,
{
    fn rem_assign(&mut self, rhs: T) {
        self.x %= rhs;
        self.y %= rhs;
    }
}
