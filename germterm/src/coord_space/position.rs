use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Position<T = i16> {
    pub x: T,
    pub y: T,
}

impl<T> Position<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn to_tuple(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T> Position<T>
where
    T: ops::Add<Output = T>,
{
    pub fn offset_x(self, dx: T) -> Self {
        Self::new(self.x + dx, self.y)
    }

    pub fn offset_y(self, dy: T) -> Self {
        Self::new(self.x, self.y + dy)
    }

    pub fn offset_xy(self, dx: T, dy: T) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }
}

impl<T> ops::Add for Position<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> ops::AddAssign for Position<T>
where
    T: ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> ops::Sub for Position<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> ops::SubAssign for Position<T>
where
    T: ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T, S> ops::Mul<S> for Position<T>
where
    T: ops::Mul<S, Output = T>,
    S: Copy,
{
    type Output = Self;

    fn mul(self, rhs: S) -> Self::Output {
        Position::new(self.x * rhs, self.y * rhs)
    }
}

impl<T, S> ops::MulAssign<S> for Position<T>
where
    T: ops::MulAssign<S>,
    S: Copy,
{
    fn mul_assign(&mut self, rhs: S) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T, S> ops::Div<S> for Position<T>
where
    T: ops::Div<S, Output = T>,
    S: Copy,
{
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<T, S> ops::DivAssign<S> for Position<T>
where
    T: ops::DivAssign<S>,
    S: Copy,
{
    fn div_assign(&mut self, rhs: S) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T, S> ops::Rem<S> for Position<T>
where
    T: ops::Rem<S, Output = T>,
    S: Copy,
{
    type Output = Self;

    fn rem(self, rhs: S) -> Self::Output {
        Self::new(self.x % rhs, self.y % rhs)
    }
}

impl<T, S> ops::RemAssign<S> for Position<T>
where
    T: ops::RemAssign<S>,
    S: Copy,
{
    fn rem_assign(&mut self, rhs: S) {
        self.x %= rhs;
        self.y %= rhs;
    }
}
