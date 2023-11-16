use std::rc::Rc;

use crate::{
    physics::Ray,
    shapes::hittable::{HitRecord, Hittable},
    utils::Interval,
};

#[derive(Default)]
pub(crate) struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub(crate) fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut record = None;
        let mut closest_so_far = ray_t.max;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                record = Some(rec)
            }
        }

        record
    }
}
