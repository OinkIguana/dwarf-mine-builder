use game_engine::prelude::*;

pub mod hovered_cube;
pub mod task_list;
pub mod visible_range;

pub use self::{
    hovered_cube::*,
    task_list::*,
    visible_range::*,
};

crate fn register(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.add_resource(HoveredCube::default())
        .add_resource(TaskList::default())
        .add_resource(VisibleRange::default())
}
