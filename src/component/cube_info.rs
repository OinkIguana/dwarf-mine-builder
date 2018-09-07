use specs_derive::Component;
use game_engine::prelude::*;
use crate::model::{CubeDescriptor, FloorMap};

#[derive(Component, Copy, Clone, Debug)]
pub struct CubeInfo(CubeDescriptor);

impl CubeInfo {
    pub fn new(descriptor: CubeDescriptor) -> Self {
        CubeInfo(descriptor)
    }

    pub fn floor_map(&self) -> FloorMap {
        self.0.floor_map()
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
