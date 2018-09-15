use specs_derive::Component;
use game_engine::prelude::*;
use super::TargetPosition;

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

impl Into<Point> for &CubePosition {
    fn into(self) -> Point {
        self.point
    }
}

impl PartialEq<TargetPosition> for CubePosition {
    fn eq(&self, target: &TargetPosition) -> bool {
        self.point == target.into()
    }
}
