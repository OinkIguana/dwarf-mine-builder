use game_engine::prelude::*;
use std::fmt::{self, Debug, Formatter};
use crate::model::*;
use crate::constant::TILE_SIZE;

#[derive(Clone)]
pub struct VisibleRange {
    area: Tetra,
    floor_heights: Vec<FloorMap>,
}

impl VisibleRange {
    pub fn shift(&mut self, offset: Point3D) {
        self.area = self.area + offset
    }

    pub fn expand(&mut self, expansion: Size3D) {
        self.area = self.area + expansion
    }

    pub fn origin(&self) -> Point3D {
        self.area.origin
    }

    pub fn size(&self) -> Size3D {
        self.area.size
    }

    pub fn floor_offset(&self, cube: Point3D, point: Point) -> Point {
        self.floor_heights
            .get(self.cube_index(cube))
            .cloned()
            .unwrap_or_default()
            .offset(point)
    }

    /// The rendering depth of a cube in the visible range. The front most cube is at
    /// depth 0, with each cube `-TILE_SIZE` layers behind it.
    pub fn depth(&self, position: Point3D) -> i32 {
        let front = self.area.origin + Point3D::y(self.area.size.height as i32);
        -((position.distance_from(front) * TILE_SIZE * 2) as i32)
    }

    /// The rendering depth of an object inside of a cube within the visible range. Should be used
    /// relative to the [`cube depth`](VisibleRange::depth)
    pub fn inner_depth(&self, position: Point) -> i32 {
        let front = position.distance_from(&Point { x: 0, y: 16 });
        front as i32
    }

    /// Checks if a cube is within the visible range
    pub fn contains(&self, cube: Point3D) -> bool {
        cube.x >= self.area.origin.x
        && cube.y >= self.area.origin.y
        && cube.z >= self.area.origin.z
        && cube.x <= self.area.origin.x + self.area.size.width as i32
        && cube.y <= self.area.origin.y + self.area.size.height as i32
        && cube.z <= self.area.origin.z + self.area.size.depth as i32
    }

    pub fn cube_index(&self, cube: Point3D) -> usize {
        (
            (cube.z - self.area.origin.z) * (self.area.size.width * self.area.size.height) as i32
            + (cube.y - self.area.origin.y) * self.area.size.width as i32
            + (cube.x - self.area.origin.x)
        ) as usize
    }

    pub fn update_floor_map(&mut self, floor_heights: Vec<FloorMap>) {
        self.floor_heights = floor_heights;
    }
}

impl Default for VisibleRange {
    fn default() -> Self {
        VisibleRange {
            area: Tetra {
                origin: Point3D {
                    x: 0,
                    y: 0,
                    z: 0,
                },
                size: Size3D {
                    width: 16,
                    height: 16,
                    depth: 16,
                }
            },
            floor_heights: vec![],
        }
    }
}

impl Debug for VisibleRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "VisibleRange {{ area: {:?} }}", self.area)
    }
}
