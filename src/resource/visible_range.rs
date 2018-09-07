use game_engine::prelude::*;
use crate::model::*;
use crate::constant::TILE_SIZE;

#[derive(Copy, Clone, Debug)]
pub struct VisibleRange(Tetra);

impl VisibleRange {
    pub fn shift(&mut self, offset: Point3D) {
        self.0 = self.0 + offset
    }

    pub fn expand(&mut self, expansion: Size3D) {
        self.0 = self.0 + expansion
    }

    pub fn origin(&self) -> Point3D {
        self.0.origin
    }

    pub fn size(&self) -> Size3D {
        self.0.size
    }

    /// The rendering depth of a cube in the visible range. The front most cube is at
    /// depth 0, with each cube `-TILE_SIZE` layers behind it.
    pub fn depth(&self, position: Point3D) -> i32 {
        let front = self.0.origin + Point3D::y(self.0.size.height as i32);
        -((position.distance_from(front) * TILE_SIZE * 2) as i32)
    }

    /// The rendering depth of an object inside of a cube within the visible range. Should be used
    /// relative to the [`cube depth`](VisibleRange::depth)
    pub fn inner_depth(&self, position: Point) -> i32 {
        let front = position.distance_from(&Point { x: 0, y: 16 });
        front as i32
    }
}

impl Default for VisibleRange {
    fn default() -> Self {
        VisibleRange(Tetra {
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
        })
    }
}
