use game_engine::entity;
use crate::component::{GridPosition, CubeInfo};
use crate::model::{Point3D, CubeDescriptor};
use crate::drawable::CubeDrawable;

entity! {
    pub Cube(position: Point3D, descriptor: CubeDescriptor) {
        CubeInfo::new(descriptor),
        GridPosition::new(position),
        CubeDrawable::boxed(descriptor),
    }
}
