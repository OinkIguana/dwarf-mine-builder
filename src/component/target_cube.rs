use specs_derive::Component;
use game_engine::prelude::*;
use crate::model::Point3D;
use super::GridPosition;

/// The cube that this entity should be moving to
#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct TargetCube {
    point: Point3D,
}

impl TargetCube {
    pub fn new(point: Point3D) -> Self {
        TargetCube {
            point,
        }
    }
}

impl Into<Point3D> for &TargetCube {
    fn into(self) -> Point3D {
        self.point
    }
}

impl PartialEq<GridPosition> for TargetCube {
    fn eq(&self, position: &GridPosition) -> bool {
        self.point == position.into()
    }
}
