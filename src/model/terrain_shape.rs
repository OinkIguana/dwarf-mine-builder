/// The shapes that a cube can be made to without being built on
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum TerrainShape {
    Empty,
    Steps,
    Solid,
}

impl Default for TerrainShape {
    fn default() -> Self {
        TerrainShape::Solid
    }
}
