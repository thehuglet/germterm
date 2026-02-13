use crate::engine2::draw::{Position, Size};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Rect {
    pub size: Size,
    pub origin: Position,
}

impl Rect {
    /// A rectangle with zero size at the origin.
    pub const ZERO: Self = Self::new(Size::ZERO, Position::ZERO);

    /// Creates a new `Rect` from a `Size` and an origin `Position`.
    pub const fn new(size: Size, origin: Position) -> Self {
        Self { size, origin }
    }

    /// Creates a new `Rect` from x, y, width, and height.
    pub const fn from_xywh(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self::new(Size::new(width, height), Position::new(x, y))
    }

    /// Returns the x-coordinate of the origin.
    pub const fn x(&self) -> u16 {
        self.origin.x
    }

    /// Returns the y-coordinate of the origin.
    pub const fn y(&self) -> u16 {
        self.origin.y
    }

    /// Returns the width of the rectangle.
    pub const fn width(&self) -> u16 {
        self.size.width
    }

    /// Returns the height of the rectangle.
    pub const fn height(&self) -> u16 {
        self.size.height
    }

    /// Returns the left edge of the rectangle (same as `x`).
    pub const fn left(&self) -> u16 {
        self.origin.x
    }

    /// Returns the top edge of the rectangle (same as `y`).
    pub const fn top(&self) -> u16 {
        self.origin.y
    }

    /// Returns the right edge of the rectangle (`x + width`).
    pub const fn right(&self) -> u16 {
        self.origin.x.saturating_add(self.size.width)
    }

    /// Returns the bottom edge of the rectangle (`y + height`).
    pub const fn bottom(&self) -> u16 {
        self.origin.y.saturating_add(self.size.height)
    }

    /// Returns the total area covered by the rectangle.
    pub fn area(&self) -> u32 {
        self.size.area()
    }

    /// Returns `true` if the rectangle has zero width or height.
    pub fn is_empty(&self) -> bool {
        self.size.width == 0 || self.size.height == 0
    }

    /// Returns `true` if the given position is inside the rectangle.
    pub fn contains(&self, pos: Position) -> bool {
        self.left() <= pos.x && pos.x < self.right() && self.top() <= pos.y && pos.y < self.bottom()
    }

    /// Returns `true` if this rectangle intersects with another.
    pub fn intersects(&self, other: &Self) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.top() < other.bottom()
            && self.bottom() > other.top()
    }

    /// Returns the intersection of two rectangles, or `None` if they don't overlap.
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let left = self.left().max(other.left());
        let top = self.top().max(other.top());
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());

        if left < right && top < bottom {
            Some(Self::from_xywh(left, top, right - left, bottom - top))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let r = Rect::from_xywh(1, 2, 3, 4);
        assert_eq!(r.x(), 1);
        assert_eq!(r.y(), 2);
        assert_eq!(r.width(), 3);
        assert_eq!(r.height(), 4);
    }

    #[test]
    fn test_bounds() {
        let r = Rect::from_xywh(10, 20, 5, 5);
        assert_eq!(r.left(), 10);
        assert_eq!(r.top(), 20);
        assert_eq!(r.right(), 15);
        assert_eq!(r.bottom(), 25);
    }

    #[test]
    fn test_contains() {
        let r = Rect::from_xywh(10, 10, 10, 10);
        assert!(r.contains(Position::new(10, 10)));
        assert!(r.contains(Position::new(19, 19)));
        assert!(!r.contains(Position::new(9, 10)));
        assert!(!r.contains(Position::new(20, 10)));
        assert!(!r.contains(Position::new(15, 20)));
    }

    #[test]
    fn test_intersects() {
        let r1 = Rect::from_xywh(0, 0, 10, 10);
        let r2 = Rect::from_xywh(5, 5, 10, 10);
        let r3 = Rect::from_xywh(10, 10, 10, 10);
        assert!(r1.intersects(&r2));
        assert!(!r1.intersects(&r3));
    }

    #[test]
    fn test_intersection() {
        let r1 = Rect::from_xywh(0, 0, 10, 10);
        let r2 = Rect::from_xywh(5, 5, 10, 10);
        let inter = r1.intersection(&r2).unwrap();
        assert_eq!(inter, Rect::from_xywh(5, 5, 5, 5));

        let r3 = Rect::from_xywh(10, 10, 10, 10);
        assert!(r1.intersection(&r3).is_none());
    }

    #[test]
    fn test_is_empty() {
        assert!(Rect::from_xywh(0, 0, 0, 10).is_empty());
        assert!(Rect::from_xywh(0, 0, 10, 0).is_empty());
        assert!(!Rect::from_xywh(0, 0, 1, 1).is_empty());
    }
}

