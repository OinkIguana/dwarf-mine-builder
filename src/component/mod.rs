use game_engine::prelude::*;

mod cube_info;
mod cube_position;
mod grid_position;
mod grid_size;

pub use self::{
    cube_info::CubeInfo,
    grid_position::GridPosition,
    grid_size::GridSize,
    cube_position::CubePosition,
};

crate fn register(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.register_component::<CubeInfo>()
        .register_component::<GridPosition>()
        .register_component::<GridSize>()
        .register_component::<CubePosition>()
}
