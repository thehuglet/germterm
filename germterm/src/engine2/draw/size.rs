use crate::engine2::buffer::ErrorOutOfBoundsAxises;

use super::Position;

#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn contains(&self, pos: Position) -> Result<(), ErrorOutOfBoundsAxises> {
        let err = match pos {
            Position { x, y } if x >= self.width && y >= self.height => ErrorOutOfBoundsAxises::XY,
            Position { x, y } if y >= self.height => ErrorOutOfBoundsAxises::Y,
            Position { x, y } if x >= self.width => ErrorOutOfBoundsAxises::X,
            _ => return Ok(()),
        };

        Err(err)
    }
}
