use game_engine::prelude::*;

pub mod visible_range;
pub mod task_list;

pub use self::{
    visible_range::*,
    task_list::*,
};

crate fn register(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.add_resource(VisibleRange::default())
        .add_resource(TaskList::default())
}
