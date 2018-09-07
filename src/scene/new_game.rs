use game_engine::{scene, prelude::*};
use crate::model::*;
use crate::entity::Cube;

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
    }
}
