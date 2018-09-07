use game_engine::{system, prelude::*};
use crate::drawable::CubeDrawable;
use crate::component::{CubeInfo, GridPosition};
use crate::resource::VisibleRange;
use crate::constant::ISO_GRID;

#[derive(Default, Debug)]
pub struct MaintainCubeDrawable;

system! {
    impl MaintainCubeDrawable {
        fn run(
            &mut self,
            visible_range: &Resource<VisibleRange>,
            cube_info: &Component<CubeInfo>,
            grid_position: &Component<GridPosition>,
            drawable: &mut Component<Box<dyn Drawable>>,
        ) {
            for (grid_position, cube_info, drawable) in (&grid_position, &cube_info, &mut drawable).join() {
                if let Some(drawable) = drawable.as_any_mut().downcast_mut::<CubeDrawable>() {
                    drawable.cube = cube_info.into();
                    drawable.position = ISO_GRID.bounds_top_left(grid_position.into());
                    drawable.depth = visible_range.depth(grid_position.into());
                    drawable.visible = visible_range.contains(grid_position.into());
                }
            }
        }
    }
}
