#[derive(Clone, Copy, Debug, Hash)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub const ZERO: Position = Position::new(0, 0);

    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}
