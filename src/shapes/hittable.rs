use std::rc::Rc;

use crate::{
    physics::{Material, Point3, Ray, Vec3},
    utils::Interval,
};

#[derive(Debug)]
pub(crate) struct HitRecord {
    pub(crate) p: Point3,
    pub(crate) normal: Vec3,
    pub(crate) material: Rc<dyn Material>,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}

impl HitRecord {
    pub(crate) fn new(material: Rc<dyn Material>) -> Self {
        HitRecord {
            p: Default::default(),
            normal: Default::default(),
            material,
            t: Default::default(),
            front_face: Default::default(),
        }
    }

    pub(crate) fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal
        };
    }
}

pub(crate) trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
