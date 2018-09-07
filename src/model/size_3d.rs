use std::ops::*;
use std::cmp::{Ordering, PartialOrd};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Size3D {
    /// Corresponds to X axis
    pub width: u32,
    /// Corresponds to Y axis
    pub height: u32,
    /// Corresponds to Z axis
    pub depth: u32,
}

impl Size3D {
    pub fn cube(size: u32) -> Self {
        Self {
            width: size,
            height: size,
            depth: size,
        }
    }

    pub fn width(width: u32) -> Self {
        Self {
            width,
            ..Self::default()
        }
    }

    pub fn height(height: u32) -> Self {
        Self {
            height,
            ..Self::default()
        }
    }

    pub fn depth(depth: u32) -> Self {
        Self {
            depth,
            ..Self::default()
        }
    }

    pub fn is_cube(&self) -> bool {
        self.width == self.height && self.width == self.depth
    }

    pub fn volume(&self) -> u32 {
        self.width * self.height * self.depth
    }
}

impl PartialOrd for Size3D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.width <= other.width && self.height <= other.height && self.depth <= other.depth {
            Some(Ordering::Less)
        } else if self.width >= other.width && self.height >= other.height && self.depth >= other.depth {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl Add for Size3D {
    type Output = Size3D;
    fn add(self, other: Size3D) -> Size3D {
        Size3D {
            width: self.width + other.width,
            height: self.height + other.height,
            depth: self.depth + other.depth,
        }
    }
}

impl Sub for Size3D {
    type Output = Size3D;
    fn sub(self, other: Size3D) -> Size3D {
        Size3D {
            width: self.width - other.width,
            height: self.height - other.height,
            depth: self.depth - other.depth,
        }
    }
}
