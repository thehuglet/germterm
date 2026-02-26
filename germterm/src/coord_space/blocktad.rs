use crate::{
    coord_space::{
        Position, Size,
        native::{NativePosition, NativeSize},
        octad::{OctadPosition, OctadSize},
        twoxel::{TwoxelPosition, TwoxelSize},
    },
    impl_coord_space_position_arithmetic, impl_coord_space_size_arithmetic,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BlocktadPosition {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BlocktadSize {
    pub width: i16,
    pub height: i16,
}

impl Position for BlocktadPosition {
    fn new(x: i16, y: i16) -> Self {
        Self::new(x, y)
    }

    fn x(&self) -> i16 {
        self.x
    }

    fn y(&self) -> i16 {
        self.y
    }
}

impl Size for BlocktadSize {
    fn new(width: i16, height: i16) -> Self {
        Self::new(width, height)
    }

    fn width(&self) -> i16 {
        self.width
    }

    fn height(&self) -> i16 {
        self.height
    }
}

impl From<(i16, i16)> for BlocktadPosition {
    fn from((x, y): (i16, i16)) -> Self {
        Self { x, y }
    }
}

impl From<(i16, i16)> for BlocktadSize {
    fn from((width, height): (i16, i16)) -> Self {
        Self { width, height }
    }
}

impl BlocktadPosition {
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn to_native(self) -> NativePosition {
        NativePosition::new(self.x / 2, self.y / 4)
    }

    pub fn to_twoxel(self) -> TwoxelPosition {
        TwoxelPosition::new(self.x / 2, self.y / 2)
    }

    pub fn to_octad(self) -> OctadPosition {
        OctadPosition::new(self.x, self.y)
    }
}

impl BlocktadSize {
    pub const fn new(width: i16, height: i16) -> Self {
        Self { width, height }
    }

    pub fn to_native(self) -> NativeSize {
        NativeSize::new(self.width / 2, self.height / 4)
    }

    pub fn to_twoxel(self) -> TwoxelSize {
        TwoxelSize::new(self.width / 2, self.height / 2)
    }

    pub fn to_octad(self) -> OctadSize {
        OctadSize::new(self.width, self.height)
    }
}

impl_coord_space_position_arithmetic!(BlocktadPosition);
impl_coord_space_size_arithmetic!(BlocktadSize);
