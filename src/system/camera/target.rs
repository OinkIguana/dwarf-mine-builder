use game_engine::{system, prelude::*};
use crate::model::Point3D;
use crate::resource::VisibleRange;
use crate::constant::ISO_GRID;

#[derive(Default, Debug)]
pub struct CameraTarget;

system! {
    impl CameraTarget {
        fn run(
            &mut self,
            visible_range: &Resource<VisibleRange>,
            camera: &mut Resource<Camera>,
        ) {
            let target = ISO_GRID.origin(visible_range.origin() - Point3D { x: 0, y: 0, z: visible_range.size().depth as i32 / 2 });
            camera.set_origin(target);
        }
    }
}
