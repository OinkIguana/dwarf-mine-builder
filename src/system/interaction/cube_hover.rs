use game_engine::{system, prelude::*};
use crate::resource::{VisibleRange, HoveredCube};

#[derive(Default, Debug)]
pub struct CubeHover;

system! {
    impl CubeHover {
        fn run(
            &mut self,
            mouse_state: &Resource<MouseState>,
            camera: &Resource<Camera>,
            visible_range: &Resource<VisibleRange>,
            hovered_cube: &mut Resource<HoveredCube>,
        ) {
            let point = camera.input.point() + mouse_state.position();

            let point_3d = visible_range.target_cube(point);
            hovered_cube.set(point_3d);
        }
    }
}
