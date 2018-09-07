use std::ops::*;
use super::{Point3D, Size3D};

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Tetra {
    pub origin: Point3D,
    pub size: Size3D,
}

impl Tetra {
    pub fn cube(origin: Point3D, size: u32) -> Tetra {
        Tetra {
            origin,
            size: Size3D::cube(size)
        }
    }

    pub fn is_cube(&self) -> bool {
        self.size.is_cube()
    }

    pub fn volume(&self) -> u32 {
        self.size.volume()
    }
}

impl Add<Point3D> for Tetra {
    type Output = Tetra;
    fn add(self, offset: Point3D) -> Tetra {
        Tetra {
            origin: self.origin + offset,
            size: self.size,
        }
    }
}

impl Sub<Point3D> for Tetra {
    type Output = Tetra;
    fn sub(self, offset: Point3D) -> Tetra {
        Tetra {
            origin: self.origin - offset,
            size: self.size,
        }
    }
}

impl Add<Size3D> for Tetra {
    type Output = Tetra;
    fn add(self, extension: Size3D) -> Tetra {
        Tetra {
            origin: self.origin,
            size: self.size + extension,
        }
    }
}

impl Sub<Size3D> for Tetra {
    type Output = Tetra;
    fn sub(self, extension: Size3D) -> Tetra {
        Tetra {
            origin: self.origin,
            size: self.size - extension,
        }
    }
}
