use game_engine::system;
use crate::resource::VisibleRange;
use crate::component::{CubeInfo, GridPosition};

#[derive(Default, Debug)]
pub struct UpdateFloorHeights;

system! {
    impl UpdateFloorHeights {
        fn run(
            &mut self,
            visible_range: &mut Resource<VisibleRange>,
            cube_info: &Component<CubeInfo>,
            grid_position: &Component<GridPosition>,
        ) {
            let mut floor_map = (&cube_info, &grid_position).join()
                .filter(|(_, position)| visible_range.contains((*position).into()))
                .map(|(info, position)| (visible_range.cube_index((*position).into()), info.floor_map()))
                .collect::<Vec<_>>();
            floor_map.sort_by(|(a, _), (b, _)| a.cmp(b));
            visible_range.update_floor_map(
                floor_map
                    .into_iter()
                    .map(|(_, map)| map)
                    .collect()
            );
        }
    }
}
