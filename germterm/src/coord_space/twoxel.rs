use crate::{
    coord_space::{
        Position, Size,
        blocktad::{BlocktadPosition, BlocktadSize},
        native::{NativePosition, NativeSize},
        octad::{OctadPosition, OctadSize},
    },
    impl_coord_space_position_arithmetic, impl_coord_space_size_arithmetic,
};

#[derive(Clone, Copy)]
pub struct TwoxelPosition {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Copy)]
pub struct TwoxelSize {
    pub width: i16,
    pub height: i16,
}

impl Position for TwoxelPosition {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    fn x(&self) -> i16 {
        self.x
    }

    fn y(&self) -> i16 {
        self.y
    }
}

impl Size for TwoxelSize {
    fn new(width: i16, height: i16) -> Self {
        Self { width, height }
    }

    fn width(&self) -> i16 {
        self.width
    }

    fn height(&self) -> i16 {
        self.height
    }
}

impl From<(i16, i16)> for TwoxelPosition {
    fn from((x, y): (i16, i16)) -> Self {
        Self { x, y }
    }
}

impl From<(i16, i16)> for TwoxelSize {
    fn from((width, height): (i16, i16)) -> Self {
        Self { width, height }
    }
}

impl TwoxelPosition {
    pub fn to_native(self) -> NativePosition {
        NativePosition::new(self.x, self.y / 2)
    }
    pub fn to_octad(self) -> OctadPosition {
        OctadPosition::new(self.x * 2, self.y * 2)
    }
    pub fn to_blocktad(self) -> BlocktadPosition {
        BlocktadPosition::new(self.x * 2, self.y * 2)
    }
}

impl TwoxelSize {
    pub fn to_native(self) -> NativeSize {
        NativeSize::new(self.width, self.height / 2)
    }
    pub fn to_octad(self) -> OctadSize {
        OctadSize::new(self.width * 2, self.height * 2)
    }
    pub fn to_blocktad(self) -> BlocktadSize {
        BlocktadSize::new(self.width * 2, self.height * 2)
    }
}

impl_coord_space_position_arithmetic!(TwoxelPosition);
impl_coord_space_size_arithmetic!(TwoxelSize);
