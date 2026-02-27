#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub const ZERO: Position = Position::new(0, 0);

    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// Clamps each coordinate to the given maximum values.
    pub fn clamp(self, max_x: u16, max_y: u16) -> Self {
        Self {
            x: self.x.min(max_x),
            y: self.y.min(max_y),
        }
    }

    /// Returns the position as a `(x, y)` tuple.
    pub fn as_tuple(self) -> (u16, u16) {
        (self.x, self.y)
    }

    /// Returns `true` if the position is inside a `width x height` bounds.
    pub fn is_within(self, width: u16, height: u16) -> bool {
        self.x < width && self.y < height
    }

    pub(crate) fn to_index(self, width: u16) -> usize {
        self.y as usize * width as usize + self.x as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_reduces_to_max() {
        let pos = Position::new(10, 20);
        let clamped = pos.clamp(5, 15);
        assert_eq!(clamped.x, 5);
        assert_eq!(clamped.y, 15);
    }

    #[test]
    fn clamp_leaves_smaller_values_unchanged() {
        let pos = Position::new(3, 7);
        let clamped = pos.clamp(10, 10);
        assert_eq!(clamped.x, 3);
        assert_eq!(clamped.y, 7);
    }

    #[test]
    fn clamp_equal_values() {
        let pos = Position::new(5, 5);
        let clamped = pos.clamp(5, 5);
        assert_eq!(clamped.x, 5);
        assert_eq!(clamped.y, 5);
    }

    #[test]
    fn as_tuple_returns_correct_pair() {
        let pos = Position::new(4, 9);
        assert_eq!(pos.as_tuple(), (4, 9));
    }

    #[test]
    fn as_tuple_zero() {
        assert_eq!(Position::ZERO.as_tuple(), (0, 0));
    }

    #[test]
    fn is_within_inside_bounds() {
        let pos = Position::new(3, 4);
        assert!(pos.is_within(10, 10));
    }

    #[test]
    fn is_within_at_origin() {
        let pos = Position::ZERO;
        assert!(pos.is_within(1, 1));
    }

    #[test]
    fn is_within_at_edge_is_outside() {
        let pos = Position::new(10, 10);
        assert!(!pos.is_within(10, 10));
    }

    #[test]
    fn is_within_beyond_bounds() {
        let pos = Position::new(15, 20);
        assert!(!pos.is_within(10, 10));
    }

    #[test]
    fn is_within_zero_dimensions() {
        let pos = Position::ZERO;
        assert!(!pos.is_within(0, 0));
    }
}
