use specs_derive::Component;
use game_engine::prelude::*;
use super::CubePosition;

/// The position in the target cube this entity should be moving towards
#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct TargetPosition {
    point: Point,
}

impl TargetPosition {
    pub fn new(point: Point) -> Self {
        TargetPosition {
            point,
        }
    }
}

impl PartialEq<CubePosition> for TargetPosition {
    fn eq(&self, position: &CubePosition) -> bool {
        self.point == position.into()
    }
}

impl Into<Point> for &TargetPosition {
    fn into(self) -> Point {
        self.point
    }
}
