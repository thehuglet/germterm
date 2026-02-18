macro_rules! basic_position_impls {
    ($name:ident) => {
        impl $name {
            pub fn new(x: i16, y: i16) -> Self {
                Self { x, y }
            }

            pub fn as_tuple(self) -> (i16, i16) {
                (self.x, self.y)
            }

            pub fn offset_x(self, x: i16) -> Self {
                Self::new(self.x + x, self.y)
            }

            pub fn offset_y(self, y: i16) -> Self {
                Self::new(self.x, self.y + y)
            }

            pub fn offset_xy(self, x: i16, y: i16) -> Self {
                Self::new(self.x + x, self.y + y)
            }
        }
    };
}

macro_rules! basic_size_impls {
    ($name:ident) => {
        impl $name {
            pub fn new(width: i16, height: i16) -> Self {
                Self { width, height }
            }

            pub fn as_tuple(self) -> (i16, i16) {
                (self.width, self.height)
            }

            pub fn offset_w(self, w: i16) -> Self {
                Self::new(self.width + w, self.height)
            }

            pub fn offset_h(self, h: i16) -> Self {
                Self::new(self.width, self.height + h)
            }

            pub fn offset_wh(self, w: i16, h: i16) -> Self {
                Self::new(self.width + w, self.height + h)
            }
        }
    };
}

// Represents a position in the native terminal coordinate space.
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

#[derive(Clone, Copy)]
pub struct BlocktadPosition {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Copy)]
pub struct BlocktadSize {
    pub width: i16,
    pub height: i16,
}

basic_position_impls!(NativePosition);
basic_size_impls!(NativeSize);

basic_position_impls!(TwoxelPosition);
basic_size_impls!(TwoxelSize);

basic_position_impls!(OctadPosition);
basic_size_impls!(OctadSize);

basic_position_impls!(BlocktadPosition);
basic_size_impls!(BlocktadSize);

impl From<(i16, i16)> for NativePosition {
    fn from((x, y): (i16, i16)) -> Self {
        NativePos::new(x, y)
    }
}

impl From<(i16, i16)> for NativeSize {
    fn from((width, height): (i16, i16)) -> Self {
        NativePos::new(width, height)
    }
}

impl From<OctadPosition> for NativePosition {
    fn from(pos: OctadPosition) -> Self {
        NativePosition::new(pos.x / 2, pos.y / 4)
    }
}
