use specs_derive::Component;
use game_engine::prelude::*;
use crate::model::CubeDescriptor;

#[derive(Component, Copy, Clone, Debug)]
pub struct CubeInfo(CubeDescriptor);

impl CubeInfo {
    pub fn new(descriptor: CubeDescriptor) -> Self {
        CubeInfo(descriptor)
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
