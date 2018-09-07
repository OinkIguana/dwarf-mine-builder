use game_engine::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum CubeKind {
    Empty,
}

impl CubeKind {
    pub fn sprite(&self) -> Option<&'static Sprite> {
        match self {
            CubeKind::Empty => Some(&crate::sprite::CUBE),
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
        self.subtype
    }
}
