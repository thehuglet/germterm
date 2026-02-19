pub mod blocktad;
pub mod native;
pub mod octad;
pub mod twoxel;

#[macro_export]
macro_rules! impl_coord_space_position_arithmetic {
    ($type:ty) => {
        impl std::ops::Add for $type {
            type Output = $type;

            fn add(self, rhs: $type) -> $type {
                <$type>::new(self.x + rhs.x, self.y + rhs.y())
            }
        }

        impl std::ops::AddAssign for $type {
            fn add_assign(&mut self, rhs: $type) {
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }

        impl std::ops::Sub for $type {
            type Output = $type;
            fn sub(self, rhs: $type) -> $type {
                <$type>::new(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl std::ops::SubAssign for $type {
            fn sub_assign(&mut self, rhs: $type) {
                self.x -= rhs.x;
                self.y -= rhs.y;
            }
        }

        impl std::ops::Mul<i16> for $type {
            type Output = $type;
            fn mul(self, rhs: i16) -> $type {
                <$type>::new(self.x * rhs, self.y * rhs)
            }
        }

        impl std::ops::MulAssign<i16> for $type {
            fn mul_assign(&mut self, rhs: i16) {
                self.x *= rhs;
                self.y *= rhs;
            }
        }

        impl std::ops::Div<i16> for $type {
            type Output = $type;
            fn div(self, rhs: i16) -> $type {
                <$type>::new(self.x / rhs, self.y / rhs)
            }
        }

        impl std::ops::DivAssign<i16> for $type {
            fn div_assign(&mut self, rhs: i16) {
                self.x /= rhs;
                self.y /= rhs;
            }
        }
    };
}

#[macro_export]
macro_rules! impl_coord_space_size_arithmetic {
    ($type:ty) => {
        impl std::ops::Add for $type {
            type Output = $type;

            fn add(self, rhs: $type) -> $type {
                <$type>::new(self.width + rhs.width, self.height + rhs.height)
            }
        }

        impl std::ops::AddAssign for $type {
            fn add_assign(&mut self, rhs: $type) {
                self.width += rhs.width;
                self.height += rhs.height;
            }
        }

        impl std::ops::Sub for $type {
            type Output = $type;
            fn sub(self, rhs: $type) -> $type {
                <$type>::new(self.width - rhs.width, self.height - rhs.height)
            }
        }

        impl std::ops::SubAssign for $type {
            fn sub_assign(&mut self, rhs: $type) {
                self.width -= rhs.width;
                self.height -= rhs.height;
            }
        }

        impl std::ops::Mul<i16> for $type {
            type Output = $type;
            fn mul(self, rhs: i16) -> $type {
                <$type>::new(self.width * rhs, self.height * rhs)
            }
        }

        impl std::ops::MulAssign<i16> for $type {
            fn mul_assign(&mut self, rhs: i16) {
                self.width *= rhs;
                self.height *= rhs;
            }
        }

        impl std::ops::Div<i16> for $type {
            type Output = $type;
            fn div(self, rhs: i16) -> $type {
                <$type>::new(self.width / rhs, self.height / rhs)
            }
        }

        impl std::ops::DivAssign<i16> for $type {
            fn div_assign(&mut self, rhs: i16) {
                self.width /= rhs;
                self.height /= rhs;
            }
        }
    };
}

pub trait Position: Copy + Clone {
    fn new(x: i16, y: i16) -> Self;
    fn x(&self) -> i16;
    fn y(&self) -> i16;

    fn offset_x(self, dx: i16) -> Self {
        Self::new(self.x() + dx, self.y())
    }

    fn offset_y(self, dy: i16) -> Self {
        Self::new(self.x(), self.y() + dy)
    }

    fn offset_xy(self, dx: i16, dy: i16) -> Self {
        Self::new(self.x() + dx, self.y() + dy)
    }

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x() - rhs.x(), self.y() - rhs.y())
    }
    fn mul(self, scalar: i16) -> Self {
        Self::new(self.x() * scalar, self.y() * scalar)
    }
    fn div(self, scalar: i16) -> Self {
        Self::new(self.x() / scalar, self.y() / scalar)
    }

    fn to_tuple(self) -> (i16, i16) {
        (self.x(), self.y())
    }
}

pub trait Size: Copy + Clone {
    fn new(width: i16, height: i16) -> Self;
    fn width(&self) -> i16;
    fn height(&self) -> i16;

    fn offset_w(self, dw: i16) -> Self {
        Self::new(self.width() + dw, self.height())
    }

    fn offset_h(self, dh: i16) -> Self {
        Self::new(self.width(), self.height() + dh)
    }

    fn offset_wh(self, dw: i16, dh: i16) -> Self {
        Self::new(self.width() + dw, self.height() + dh)
    }

    fn add(self, rhs: Self) -> Self {
        Self::new(self.width() + rhs.width(), self.height() + rhs.height())
    }
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.width() - rhs.width(), self.height() - rhs.height())
    }
    fn mul(self, scalar: i16) -> Self {
        Self::new(self.width() * scalar, self.height() * scalar)
    }
    fn div(self, scalar: i16) -> Self {
        Self::new(self.width() / scalar, self.height() / scalar)
    }

    fn to_tuple(self) -> (i16, i16) {
        (self.width(), self.height())
    }
}
