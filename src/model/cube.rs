use game_engine::prelude::*;
use super::FloorMap;
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
            _ => FloorMap::default()
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
    subtype: usize,
}

impl CubeDescriptor {
    pub const fn new(kind: CubeKind) -> Self {
        CubeDescriptor {
            kind,
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
}
