use game_engine::{scene, prelude::*};
use crate::model::*;
use crate::entity::*;

scene! {
    pub NEW_GAME {
        bounds: Rect::default(),
        entities: []
    } => |builder| {
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let point = Point3D { x, y, z };
                    builder.add_entity(Cube(point, CubeDescriptor::default()));
                }
            }
        }
        builder.add_entity(Dwarf(Point3D { x: 0, y: 15, z: 0 }, Point { x: 0, y: 16 }, DwarfDescriptor::new(String::from("Eric"))));
    }
}
