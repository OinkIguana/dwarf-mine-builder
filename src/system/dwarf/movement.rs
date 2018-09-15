use game_engine::system;
use crate::component::*;

#[derive(Default, Debug)]
pub struct DwarfMovement;

system! {
    impl DwarfMovement {
        fn run(
            &mut self,
            dwarf_info: &Component<DwarfInfo>,
            grid_position: &mut Component<GridPosition>,
            cube_position: &mut Component<CubePosition>,
            target_cube: &Component<TargetCube>,
            target_position: &Component<TargetPosition>,
        ) {
            for (_, grid_position, cube_position, target_cube, target_position) in (
                &dwarf_info,
                &mut grid_position,
                &mut cube_position,
                &target_cube,
                &target_position,
            ).join() {
                if target_cube != grid_position {
                    // calculate path according to walkable cubes and attempt to walk it
                } else if target_position != cube_position {
                    // calculate path within the current cube and attempt to walk it
                }
            }
        }
    }
}
