use game_engine::{entity, prelude::*};
use crate::drawable::DwarfDrawable;
use crate::component::{GridPosition, CubePosition, DwarfInfo, TargetCube, TargetPosition, Task};
use crate::model::{Point3D, DwarfDescriptor};

entity! {
    pub Dwarf(cube: Point3D, position: Point, dwarf: DwarfDescriptor) {
        DwarfInfo::new(dwarf.clone()),
        GridPosition::new(cube),
        CubePosition::new(position),
        TargetCube::default(),
        TargetPosition::default(),
        Task::default(),
        DwarfDrawable::boxed(dwarf),
    }
}
