use game_engine::prelude::*;

pub mod visible_range;

pub use self::visible_range::*;

crate fn register(game: Game<'a, 'b>) -> Game<'a, 'b> {
    game.add_resource(VisibleRange::default())
}
