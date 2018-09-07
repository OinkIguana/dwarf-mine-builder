use game_engine::{system, prelude::*};
use crate::drawable::DwarfDrawable;
use crate::component::{DwarfInfo, GridPosition, CubePosition};
use crate::resource::VisibleRange;
use crate::constant::ISO_GRID;

#[derive(Default, Debug)]
pub struct MaintainDwarfDrawable;

system! {
    impl MaintainDwarfDrawable {
        fn run(
            &mut self,
            visible_range: &Resource<VisibleRange>,
            dwarf_info: &Component<DwarfInfo>,
            grid_position: &Component<GridPosition>,
            cube_position: &Component<CubePosition>,
            drawable: &mut Component<Box<dyn Drawable>>,
        ) {
            for (grid_position, cube_position, dwarf_info, drawable) in (&grid_position, &cube_position, &dwarf_info, &mut drawable).join() {
                if let Some(drawable) = drawable.as_any_mut().downcast_mut::<DwarfDrawable>() {
                    drawable.dwarf = dwarf_info.clone().into();
                    drawable.position = ISO_GRID.position_in_cube(grid_position.into(), cube_position.into());
                    drawable.depth = visible_range.depth(grid_position.into()) + visible_range.inner_depth(cube_position.into());
                }
            }
        }
    }
}
