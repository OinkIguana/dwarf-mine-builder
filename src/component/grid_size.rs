use specs_derive::Component;
use game_engine::prelude::*;
use crate::model::Size3D;

/// The span of cubes in the game grid this entity takes up
#[derive(Component, Copy, Clone, Eq, PartialEq, PartialOrd, Debug)]
pub struct GridSize {
    size: Size3D,
}

impl GridSize {
    pub fn new(size: Size3D) -> Self {
        GridSize {
            size,
        }
    }
}

impl Default for GridSize {
    fn default() -> Self {
        GridSize {
            size: Size3D {
                width: 1,
                height: 1,
                depth: 1,
            }
        }
    }
}
