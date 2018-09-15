use specs_derive::Component;
use game_engine::prelude::*;
use crate::model::Point3D;
use super::TargetCube;

/// The position in the game grid of the cube this entity is in.
#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct GridPosition {
    point: Point3D,
}

impl GridPosition {
    pub fn new(point: Point3D) -> Self {
        GridPosition {
            point,
        }
    }
}

impl Into<Point3D> for &GridPosition {
    fn into(self) -> Point3D {
        self.point
    }
}

impl PartialEq<TargetCube> for GridPosition {
    fn eq(&self, target: &TargetCube) -> bool {
        self.point == target.into()
    }
}
