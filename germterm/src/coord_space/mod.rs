pub mod blocktad;
pub mod native;
pub mod octad;
pub mod position;
pub mod size;
pub mod twoxel;

pub trait CoordinateSpace {
    type X: Copy;
    type Y: Copy;
}

#[derive(Copy, Clone)]
pub struct Twoxel;

#[derive(Copy, Clone)]
pub struct Octad;

#[derive(Copy, Clone)]
pub struct Blocktad;

impl CoordinateSpace for Twoxel {
    type X = i16;
    type Y = i16;
}

impl CoordinateSpace for Octad {
    type X = i16;
    type Y = i16;
}

impl CoordinateSpace for Blocktad {
    type X = i16;
    type Y = i16;
}
