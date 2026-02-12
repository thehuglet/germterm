use crate::engine2::buffer::ErrorOutOfBoundsAxises;

use super::Position;

/// A 2D size representing width and height in terminal cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    /// A zero-sized `Size` with both dimensions set to 0.
    pub const ZERO: Self = Size::new(0, 0);

    /// Creates a new `Size` with the given width and height.
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    /// Checks whether a position falls within this size's bounds.
    ///
    /// Returns `Ok(())` if `pos.x < width` and `pos.y < height`.
    /// Otherwise returns an error indicating which axes are out of bounds.
    pub fn contains(&self, pos: Position) -> Result<(), ErrorOutOfBoundsAxises> {
        let err = match pos {
            Position { x, y } if x >= self.width && y >= self.height => ErrorOutOfBoundsAxises::XY,
            Position { x, y } if y >= self.height => ErrorOutOfBoundsAxises::Y,
            Position { x, y } if x >= self.width => ErrorOutOfBoundsAxises::X,
            _ => return Ok(()),
        };

        Err(err)
    }

    /// Returns `true` if the position is strictly within bounds.
    ///
    /// This is a boolean convenience alternative to [`contains`](Self::contains).
    pub fn is_within(self, pos: Position) -> bool {
        self.width > pos.x && self.height > pos.y
    }

    /// Returns `true` if `self` is large enough to fully contain `other`.
    ///
    /// Both width and height of `self` must be greater than or equal to
    /// the corresponding dimensions of `other`.
    pub fn fits(self, other: Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    /// Returns the total number of cells as `width * height`.
    pub fn area(&self) -> u32 {
        self.width as u32 * self.height as u32
    }

    /// Returns a new `Size` with both dimensions multiplied by `by`.
    ///
    /// # Panics
    ///
    /// Panics on overflow in debug mode.
    pub fn scale(self, by: u16) -> Self {
        Self {
            width: self.width * by,
            height: self.height * by,
        }
    }

    /// Clamps both dimensions to be at most those of `other`.
    pub fn clamp(self, other: Self) -> Self {
        Self {
            width: self.width.clamp(0, other.width),
            height: self.height.clamp(0, other.height),
        }
    }

    /// Swaps width and height.
    pub fn transpose(self) -> Self {
        Self {
            width: self.height,
            height: self.width,
        }
    }

    /// Component-wise saturating addition.
    ///
    /// Each dimension is added independently, capping at [`u16::MAX`].
    pub fn saturating_add(self, other: Self) -> Self {
        Self {
            width: self.width.saturating_add(other.width),
            height: self.height.saturating_add(other.height),
        }
    }

    /// Component-wise saturating subtraction.
    ///
    /// Each dimension is subtracted independently, flooring at 0.
    pub fn saturating_sub(self, other: Self) -> Self {
        Self {
            width: self.width.saturating_sub(other.width),
            height: self.height.saturating_sub(other.height),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine2::{buffer::ErrorOutOfBoundsAxises, draw::Position};

    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(Size::ZERO, Size::new(0, 0));
        assert_eq!(Size::ZERO.area(), 0);
    }

    #[test]
    fn test_new() {
        let s = Size::new(10, 20);
        assert_eq!(s.width, 10);
        assert_eq!(s.height, 20);
    }

    // -- contains --

    #[test]
    fn test_contains_within_bounds() {
        let s = Size::new(10, 5);
        assert!(s.contains(Position::new(0, 0)).is_ok());
        assert!(s.contains(Position::new(9, 4)).is_ok());
        assert!(s.contains(Position::new(5, 2)).is_ok());
    }

    #[test]
    fn test_contains_x_out_of_bounds() {
        let s = Size::new(10, 5);
        assert_eq!(
            s.contains(Position::new(10, 0)).unwrap_err(),
            ErrorOutOfBoundsAxises::X
        );
        assert_eq!(
            s.contains(Position::new(100, 4)).unwrap_err(),
            ErrorOutOfBoundsAxises::X
        );
    }

    #[test]
    fn test_contains_y_out_of_bounds() {
        let s = Size::new(10, 5);
        assert_eq!(
            s.contains(Position::new(0, 5)).unwrap_err(),
            ErrorOutOfBoundsAxises::Y
        );
        assert_eq!(
            s.contains(Position::new(9, 100)).unwrap_err(),
            ErrorOutOfBoundsAxises::Y
        );
    }

    #[test]
    fn test_contains_xy_out_of_bounds() {
        let s = Size::new(10, 5);
        assert_eq!(
            s.contains(Position::new(10, 5)).unwrap_err(),
            ErrorOutOfBoundsAxises::XY
        );
        assert_eq!(
            s.contains(Position::new(100, 100)).unwrap_err(),
            ErrorOutOfBoundsAxises::XY
        );
    }

    #[test]
    fn test_contains_zero_size() {
        let s = Size::ZERO;
        assert_eq!(
            s.contains(Position::new(0, 0)).unwrap_err(),
            ErrorOutOfBoundsAxises::XY
        );
    }

    // -- is_within --

    #[test]
    fn test_is_within() {
        let s = Size::new(10, 5);
        assert!(s.is_within(Position::new(0, 0)));
        assert!(s.is_within(Position::new(9, 4)));
        assert!(!s.is_within(Position::new(10, 0)));
        assert!(!s.is_within(Position::new(0, 5)));
        assert!(!s.is_within(Position::new(10, 5)));
    }

    #[test]
    fn test_is_within_agrees_with_contains() {
        let s = Size::new(8, 6);
        for x in 0..12 {
            for y in 0..10 {
                let pos = Position::new(x, y);
                assert_eq!(s.is_within(pos), s.contains(pos).is_ok());
            }
        }
    }

    // -- fits --

    #[test]
    fn test_fits() {
        let big = Size::new(10, 10);
        let small = Size::new(5, 5);
        assert!(big.fits(small));
        assert!(!small.fits(big));
        assert!(big.fits(big));
    }

    #[test]
    fn test_fits_partial() {
        let a = Size::new(10, 5);
        let b = Size::new(5, 10);
        assert!(!a.fits(b));
        assert!(!b.fits(a));
    }

    #[test]
    fn test_fits_zero() {
        assert!(Size::new(1, 1).fits(Size::ZERO));
        assert!(Size::ZERO.fits(Size::ZERO));
    }

    // -- area --

    #[test]
    fn test_area() {
        assert_eq!(Size::new(10, 5).area(), 50);
        assert_eq!(Size::new(1, 1).area(), 1);
        assert_eq!(Size::ZERO.area(), 0);
        assert_eq!(Size::new(0, 100).area(), 0);
    }

    #[test]
    fn test_area_large() {
        // u16::MAX * u16::MAX fits in u32
        let s = Size::new(u16::MAX, u16::MAX);
        assert_eq!(s.area(), u16::MAX as u32 * u16::MAX as u32);
    }

    // -- scale --

    #[test]
    fn test_scale() {
        let s = Size::new(3, 4);
        assert_eq!(s.scale(2), Size::new(6, 8));
        assert_eq!(s.scale(1), s);
        assert_eq!(s.scale(0), Size::ZERO);
    }

    #[test]
    #[should_panic]
    fn test_scale_overflow_panics_in_debug() {
        Size::new(u16::MAX, 1).scale(2);
    }

    // -- clamp --

    #[test]
    fn test_clamp_smaller() {
        let s = Size::new(10, 20);
        assert_eq!(s.clamp(Size::new(5, 15)), Size::new(5, 15));
    }

    #[test]
    fn test_clamp_already_within() {
        let s = Size::new(3, 4);
        assert_eq!(s.clamp(Size::new(10, 10)), Size::new(3, 4));
    }

    #[test]
    fn test_clamp_to_zero() {
        assert_eq!(Size::new(10, 10).clamp(Size::ZERO), Size::ZERO);
    }

    // -- transpose --

    #[test]
    fn test_transpose() {
        assert_eq!(Size::new(3, 7).transpose(), Size::new(7, 3));
    }

    #[test]
    fn test_transpose_is_involution() {
        let s = Size::new(5, 10);
        assert_eq!(s.transpose().transpose(), s);
    }

    #[test]
    fn test_transpose_square() {
        let s = Size::new(4, 4);
        assert_eq!(s.transpose(), s);
    }

    // -- saturating_add --

    #[test]
    fn test_saturating_add() {
        let a = Size::new(3, 4);
        let b = Size::new(2, 5);
        assert_eq!(a.saturating_add(b), Size::new(5, 9));
    }

    #[test]
    fn test_saturating_add_caps_at_max() {
        let a = Size::new(u16::MAX, 100);
        let b = Size::new(1, u16::MAX);
        assert_eq!(a.saturating_add(b), Size::new(u16::MAX, u16::MAX));
    }

    #[test]
    fn test_saturating_add_identity() {
        let s = Size::new(10, 20);
        assert_eq!(s.saturating_add(Size::ZERO), s);
    }

    // -- saturating_sub --

    #[test]
    fn test_saturating_sub() {
        let a = Size::new(10, 20);
        let b = Size::new(3, 5);
        assert_eq!(a.saturating_sub(b), Size::new(7, 15));
    }

    #[test]
    fn test_saturating_sub_floors_at_zero() {
        let a = Size::new(3, 4);
        let b = Size::new(10, 10);
        assert_eq!(a.saturating_sub(b), Size::ZERO);
    }

    #[test]
    fn test_saturating_sub_identity() {
        let s = Size::new(10, 20);
        assert_eq!(s.saturating_sub(Size::ZERO), s);
    }
}
