use crate::coord_space::blocktad::{BlocktadPosition, BlocktadSize};
use crate::coord_space::octad::{OctadPosition, OctadSize};
use crate::coord_space::twoxel::{TwoxelPosition, TwoxelSize};
use crate::coord_space::{Position, Size};
use crate::{impl_coord_space_position_arithmetic, impl_coord_space_size_arithmetic};

#[derive(Clone, Copy)]
pub struct NativePosition {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Copy)]
pub struct NativeSize {
    pub width: i16,
    pub height: i16,
}

impl Position for NativePosition {
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

impl Size for NativeSize {
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

impl From<(i16, i16)> for NativePosition {
    fn from((x, y): (i16, i16)) -> Self {
        Self { x, y }
    }
}

impl From<(i16, i16)> for NativeSize {
    fn from((width, height): (i16, i16)) -> Self {
        Self { width, height }
    }
}

impl NativePosition {
    pub fn to_twoxel(self) -> TwoxelPosition {
        TwoxelPosition::new(self.x, self.y * 2)
    }

    pub fn to_octad(self) -> OctadPosition {
        OctadPosition::new(self.x * 2, self.y * 4)
    }

    pub fn to_blocktad(self) -> BlocktadPosition {
        BlocktadPosition::new(self.x * 2, self.y * 4)
    }
}

impl NativeSize {
    pub fn to_twoxel(self) -> TwoxelSize {
        TwoxelSize::new(self.width, self.height * 2)
    }

    pub fn to_octad(self) -> OctadSize {
        OctadSize::new(self.width * 2, self.height * 4)
    }

    pub fn to_blocktad(self) -> BlocktadSize {
        BlocktadSize::new(self.width * 2, self.height * 4)
    }
}

impl_coord_space_position_arithmetic!(NativePosition);
impl_coord_space_size_arithmetic!(NativeSize);
