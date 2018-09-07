use game_engine::prelude::*;
use std::fmt::{self, Formatter, Debug};
use crate::constant::TILE_SIZE;

/// A height map of the floor of a cube.
#[derive(Copy, Clone)]
pub struct FloorMap([u8; (TILE_SIZE * TILE_SIZE) as usize]);

impl FloorMap {
    pub const fn new(map: [u8; (TILE_SIZE * TILE_SIZE) as usize]) -> Self {
        FloorMap(map)
    }

    /// The height at a position
    pub fn height(&self, position: Point) -> u8 {
        self.0.get(position.y as usize * TILE_SIZE as usize + position.x as usize)
            .cloned()
            .unwrap_or_default()
    }

    /// The offset caused by the floor map
    pub fn offset(&self, position: Point) -> Point {
        Point {
            x: 0,
            y: -(self.height(position) as i32),
        }
    }
}

impl Default for FloorMap {
    fn default() -> Self {
        FloorMap([0; (TILE_SIZE * TILE_SIZE) as usize])
    }
}

impl Debug for FloorMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..TILE_SIZE {
            for x in 0..TILE_SIZE {
                write!(f, "{:3}", self.0[(y * TILE_SIZE + x) as usize])?;
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}
