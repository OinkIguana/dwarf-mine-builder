use crate::model::{Size3D, IsoGrid};

/// The side length of one of the faces on the cubes, in terms of mappable positions
/// This is half the size of a tile because everything is flattened a bit...
pub const TILE_SIZE: u32 = 16;

/// The size of the cubes
pub const CUBE_SIZE: Size3D = Size3D {
    width: TILE_SIZE * 4,
    height: TILE_SIZE * 2,
    depth: TILE_SIZE * 2,
};

/// The IsoGrid used for game scale
pub const ISO_GRID: IsoGrid = IsoGrid::new(CUBE_SIZE);
