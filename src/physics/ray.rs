use crate::physics::{Point3, Vec3};

#[derive(Default)]
pub(crate) struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub(crate) fn new(origin: &Point3, direction: &Vec3) -> Self {
        Ray {
            orig: origin.clone(),
            dir: direction.clone(),
        }
    }

    pub(crate) fn origin(&self) -> Point3 {
        self.orig.clone()
    }

    pub(crate) fn direction(&self) -> Vec3 {
        self.dir.clone()
    }

    pub(crate) fn at(&self, t: f64) -> Point3 {
        &self.orig + (&self.dir * t)
    }
}
