use specs_derive::Component;
use game_engine::prelude::*;

/// The position of an entity on the floor of a cube
#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct CubePosition {
    point: Point,
}

impl CubePosition {
    pub fn new(point: Point) -> Self {
        CubePosition {
            point,
        }
    }
}
