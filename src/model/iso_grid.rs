use game_engine::prelude::*;
use super::{Point3D, Size3D};

/// A visual isometric 3D grid
#[derive(Copy, Clone, Default, Debug)]
pub struct IsoGrid {
    size: Size3D,
}

impl IsoGrid {
    /// Creates a new `IsoGrid` with the provided dimensional scales.
    pub const fn new(size: Size3D) -> Self {
        IsoGrid {
            size,
        }
    }

    /// The top left corner of the 2D rectangle that surrounds this cube.
    pub const fn bounds_top_left(&self, point: Point3D) -> Point {
        Point {
            x: (point.x - point.y) * self.size.width as i32 / 2,
            y: point.z * self.size.depth as i32 +
                (point.y + point.x - 1) * self.size.height as i32 / 2,
        }
    }

    /// The center coordinate of this cube (top front corner)
    pub const fn center(&self, point: Point3D) -> Point {
        Point {
            x: (point.x - point.y + 1) * self.size.width as i32 / 2,
            y: point.z * self.size.depth as i32 +
                (point.y + point.x + 1) * self.size.height as i32 / 2,
        }
    }

    /// The top left corner of the cube.
    pub const fn origin(&self, point: Point3D) -> Point {
        Point {
            x: (point.x - point.y) * self.size.width as i32 / 2,
            y: point.z * self.size.depth as i32 +
                (point.y + point.x) * self.size.height as i32 / 2,
        }
    }

    pub fn position_in_cube(&self, cube: Point3D, point: Point) -> Point {
        // TODO: this one is wrong pretty sure
        self.origin(cube) + Point {
            x: point.x * 2,
            y: point.y,
        }
    }
}
