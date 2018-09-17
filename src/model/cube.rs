use game_engine::prelude::*;
use super::{FloorMap, TerrainShape, CollisionMap};
use crate::sprite;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum CubeKind {
    Empty,
    GrassyDirt,
    Dirt,
    Stone,
}

impl CubeKind {
    pub fn sprite(&self) -> Option<&'static Sprite> {
        match self {
            | CubeKind::Empty => None,

            | CubeKind::GrassyDirt
            | CubeKind::Dirt
            | CubeKind::Stone => Some(&sprite::TERRAIN),
        }
    }

    pub fn floor_map(&self) -> FloorMap {
        match self {
            CubeKind::Empty => FloorMap::default(),
            _ => FloorMap::SOLID,
        }
    }

    /// A map of tiles which cannot be walked on
    pub fn collision_map(&self) -> CollisionMap {
        match self {
            CubeKind::Empty => CollisionMap::default(),
            _ => CollisionMap::SOLID,
        }
    }

    /// A map of tiles which can be walked on if the cube above has no floor
    pub fn ceiling_map(&self) -> CollisionMap {
        match self {
            CubeKind::Empty => CollisionMap::default(),
            _ => CollisionMap::SOLID,
        }
    }
}

impl Default for CubeKind {
    fn default() -> CubeKind {
        CubeKind::Empty
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct CubeDescriptor {
    kind: CubeKind,
    shape: TerrainShape,
    subtype: usize,
}

impl CubeDescriptor {
    pub const fn new(kind: CubeKind) -> Self {
        CubeDescriptor {
            kind,
            shape: TerrainShape::Solid,
            subtype: 0,
        }
    }

    pub const fn subtype(self, subtype: usize) -> Self {
        Self {
            subtype,
            ..self
        }
    }
}

impl CubeDescriptor {
    pub fn sprite(&self) -> Option<&'static Sprite> {
        self.kind.sprite()
    }

    pub fn frame(&self) -> usize {
        match self.kind {
            CubeKind::Empty => 0,

            CubeKind::GrassyDirt => 0,
            CubeKind::Dirt => 1,
            CubeKind::Stone => 2,
        }
    }

    pub fn floor_map(&self) -> FloorMap {
        self.kind.floor_map()
    }

    /// A map of tiles which cannot be walked on
    pub fn collision_map(&self) -> CollisionMap {
        self.kind.collision_map()
    }

    /// A map of tiles which can be walked on if the cube above has no floor
    pub fn ceiling_map(&self) -> CollisionMap {
        self.kind.ceiling_map()
    }
}
