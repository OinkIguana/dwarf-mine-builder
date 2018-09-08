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
                    let kind = if z == 0 { CubeKind::GrassyDirt } else { CubeKind::Dirt };
                    builder.add_entity(Cube(point, CubeDescriptor::new(kind)));
                }
            }
        }
        builder.add_entity(Dwarf(Point3D { x: 0, y: 15, z: -1 }, Point { x: 16, y: 0 }, DwarfDescriptor::new(String::from("Eric"))));
    }
}
