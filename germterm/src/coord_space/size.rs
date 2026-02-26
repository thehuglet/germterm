use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Size<T = i16> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub const fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    pub fn to_tuple(self) -> (T, T) {
        (self.width, self.height)
    }
}

impl<T> Size<T>
where
    T: ops::Add<Output = T>,
{
    pub fn offset_w(self, dw: T) -> Self {
        Self::new(self.width + dw, self.height)
    }

    pub fn offset_h(self, dh: T) -> Self {
        Self::new(self.width, self.height + dh)
    }

    pub fn offset_wh(self, dw: T, dh: T) -> Self {
        Self::new(self.width + dw, self.height + dh)
    }
}

impl<T> ops::Add for Size<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl<T> ops::AddAssign for Size<T>
where
    T: ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.width += rhs.width;
        self.height += rhs.height;
    }
}

impl<T> ops::Sub for Size<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.width - rhs.width, self.height - rhs.height)
    }
}

impl<T> ops::SubAssign for Size<T>
where
    T: ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.width -= rhs.width;
        self.height -= rhs.height;
    }
}

impl<T, S> ops::Mul<S> for Size<T>
where
    T: ops::Mul<S, Output = T>,
    S: Copy,
{
    type Output = Self;

    fn mul(self, rhs: S) -> Self::Output {
        Size::new(self.width * rhs, self.height * rhs)
    }
}

impl<T, S> ops::MulAssign<S> for Size<T>
where
    T: ops::MulAssign<S>,
    S: Copy,
{
    fn mul_assign(&mut self, rhs: S) {
        self.width *= rhs;
        self.height *= rhs;
    }
}

impl<T, S> ops::Div<S> for Size<T>
where
    T: ops::Div<S, Output = T>,
    S: Copy,
{
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        Self::new(self.width / rhs, self.height / rhs)
    }
}

impl<T, S> ops::DivAssign<S> for Size<T>
where
    T: ops::DivAssign<S>,
    S: Copy,
{
    fn div_assign(&mut self, rhs: S) {
        self.width /= rhs;
        self.height /= rhs;
    }
}

impl<T, S> ops::Rem<S> for Size<T>
where
    T: ops::Rem<S, Output = T>,
    S: Copy,
{
    type Output = Self;

    fn rem(self, rhs: S) -> Self::Output {
        Self::new(self.width % rhs, self.height % rhs)
    }
}

impl<T, S> ops::RemAssign<S> for Size<T>
where
    T: ops::RemAssign<S>,
    S: Copy,
{
    fn rem_assign(&mut self, rhs: S) {
        self.width %= rhs;
        self.height %= rhs;
    }
}
