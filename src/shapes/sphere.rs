use std::rc::Rc;

use crate::{
    physics::{Material, Point3, Ray},
    shapes::hittable::{HitRecord, Hittable},
    utils::Interval,
};

pub(crate) struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub(crate) fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = r.origin() - &self.center;
        let dir = r.direction();

        let a = dir.length_squared();
        let half_b = oc.dot(&dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut rec = HitRecord::new(self.material.clone());

        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (&rec.p - &self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }
}
