use specs_derive::Component;
use game_engine::prelude::*;
use crate::model::{CubeDescriptor, FloorMap, CollisionMap};

#[derive(Component, Copy, Clone, Debug)]
pub struct CubeInfo(CubeDescriptor);

impl CubeInfo {
    pub fn new(descriptor: CubeDescriptor) -> Self {
        CubeInfo(descriptor)
    }

    /// A map of how high the floors are at each position
    pub fn floor_map(&self) -> FloorMap {
        self.0.floor_map()
    }

    /// A map of tiles which cannot be walked on
    pub fn collision_map(&self) -> CollisionMap {
        self.0.collision_map()
    }

    /// A map of tiles which can be walked on if the cube above has no floor
    pub fn ceiling_map(&self) -> CollisionMap {
        self.0.ceiling_map()
    }
}

impl Into<CubeDescriptor> for CubeInfo {
    fn into(self) -> CubeDescriptor {
        self.0
    }
}

impl Into<CubeDescriptor> for &CubeInfo {
    fn into(self) -> CubeDescriptor {
        self.0
    }
}
