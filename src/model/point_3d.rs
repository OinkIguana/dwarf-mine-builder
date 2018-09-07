use std::ops::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3D {
    pub fn x(x: i32) -> Self {
        Self {
            x,
            ..Self::default()
        }
    }

    pub fn y(y: i32) -> Self {
        Self {
            y,
            ..Self::default()
        }
    }

    pub fn z(z: i32) -> Self {
        Self {
            z,
            ..Self::default()
        }
    }

    /// The gridwise distance between two points
    pub fn distance_from(&self, other: Point3D) -> u32 {
        (
            (self.x - other.x).abs() +
            (self.y - other.y).abs() +
            (self.z - other.z).abs()
        ) as u32
    }
}

impl Add for Point3D {
    type Output = Point3D;
    fn add(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point3D {
    type Output = Point3D;
    fn sub(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Point3D {
    type Output = Point3D;
    fn neg(self) -> Point3D {
        Point3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<i32> for Point3D {
    type Output = Point3D;
    fn mul(self, scale: i32) -> Point3D {
        Point3D {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}

impl Div<i32> for Point3D {
    type Output = Point3D;
    fn div(self, scale: i32) -> Point3D {
        Point3D {
            x: self.x / scale,
            y: self.y / scale,
            z: self.z / scale,
        }
    }
}
