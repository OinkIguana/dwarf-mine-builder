use game_engine::prelude::*;
use std::fmt::{self, Formatter, Debug};
use crate::constant::TILE_SIZE;

/// A height map of the accessible areas of a cube
#[derive(Copy, Clone)]
pub struct CollisionMap([bool; (TILE_SIZE * TILE_SIZE) as usize]);

impl CollisionMap {
    pub const SOLID: CollisionMap = CollisionMap([true; (TILE_SIZE * TILE_SIZE) as usize]);

    pub const fn new(map: [bool; (TILE_SIZE * TILE_SIZE) as usize]) -> Self {
        CollisionMap(map)
    }

    /// The height at a position
    pub fn solid_at(&self, position: Point) -> bool {
        self.0.get(position.y as usize * TILE_SIZE as usize + position.x as usize)
            .cloned()
            .unwrap_or(true)
    }
}

impl Default for CollisionMap {
    fn default() -> Self {
        CollisionMap([false; (TILE_SIZE * TILE_SIZE) as usize])
    }
}

impl Debug for CollisionMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..TILE_SIZE {
            for x in 0..TILE_SIZE {
                write!(f, "{}", if self.0[(y * TILE_SIZE + x) as usize] { 1 } else { 0 })?;
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}
