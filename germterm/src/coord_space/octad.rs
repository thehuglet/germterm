use crate::{
    coord_space::{
        Position, Size,
        blocktad::{BlocktadPosition, BlocktadSize},
        native::{NativePosition, NativeSize},
        twoxel::{TwoxelPosition, TwoxelSize},
    },
    impl_coord_space_position_arithmetic, impl_coord_space_size_arithmetic,
};

#[derive(Clone, Copy)]
pub struct OctadPosition {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Copy)]
pub struct OctadSize {
    pub width: i16,
    pub height: i16,
}

impl Position for OctadPosition {
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

impl Size for OctadSize {
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

impl From<(i16, i16)> for OctadPosition {
    fn from((x, y): (i16, i16)) -> Self {
        Self { x, y }
    }
}

impl From<(i16, i16)> for OctadSize {
    fn from((width, height): (i16, i16)) -> Self {
        Self { width, height }
    }
}

impl OctadPosition {
    pub fn to_native(self) -> NativePosition {
        NativePosition::new(self.x / 2, self.y / 4)
    }

    pub fn to_twoxel(self) -> TwoxelPosition {
        TwoxelPosition::new(self.x / 2, self.y / 2)
    }

    pub fn to_blocktad(self) -> BlocktadPosition {
        BlocktadPosition::new(self.x, self.y)
    }
}

impl OctadSize {
    pub fn to_native(self) -> NativeSize {
        NativeSize::new(self.width / 2, self.height / 4)
    }

    pub fn to_twoxel(self) -> TwoxelSize {
        TwoxelSize::new(self.width / 2, self.height / 2)
    }

    pub fn to_blocktad(self) -> BlocktadSize {
        BlocktadSize::new(self.width, self.height)
    }
}

impl_coord_space_position_arithmetic!(OctadPosition);
impl_coord_space_size_arithmetic!(OctadSize);
