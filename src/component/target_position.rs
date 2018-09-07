use specs_derive::Component;
use game_engine::prelude::*;

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
