use crate::model::Point3D;

#[derive(Copy, Clone, Default)]
pub struct HoveredCube(Option<Point3D>);

impl HoveredCube {
    pub fn set(&mut self, point: Option<Point3D>) {
        self.0 = point;
    }

    pub fn clear(&mut self) {
        self.set(None);
    }

    pub fn hover(&mut self, point: Point3D) {
        self.set(Some(point));
    }
}

impl PartialEq<Point3D> for HoveredCube {
    fn eq(&self, other: &Point3D) -> bool {
        self.0 == Some(*other)
    }
}
