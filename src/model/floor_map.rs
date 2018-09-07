use game_engine::prelude::*;
use crate::constant::TILE_SIZE;

/// A height map of the floor of a cube.
pub struct FloorMap([u8; (TILE_SIZE * TILE_SIZE) as usize]);

impl FloorMap {
    pub const fn new(map: [u8; (TILE_SIZE * TILE_SIZE) as usize]) -> Self {
        FloorMap(map)
    }

    /// The height at a position
    pub const fn height(&self, position: Point) -> u8 {
        self.0[position.y as usize * TILE_SIZE as usize + position.x as usize]
    }

    /// The height for a position in Point form
    pub const fn offset(&self, position: Point) -> Point {
        Point {
            x: 0,
            y: self.height(position) as i32,
        }
    }
}
