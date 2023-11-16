use std::f64::{consts::PI, INFINITY, NEG_INFINITY};

use rand::{random, thread_rng, Rng};

use crate::physics::{Color, Vec3};

pub(crate) fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[derive(Clone, Copy)]
pub(crate) struct Interval {
    pub(crate) min: f64,
    pub(crate) max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: INFINITY,
            max: NEG_INFINITY,
        }
    }
}

impl Interval {
    pub(crate) fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub(crate) fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub(crate) fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

pub(crate) fn random_f64() -> f64 {
    random()
}

pub(crate) fn random_f64_in_interval(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}

fn random_vec3() -> Vec3 {
    Vec3::new(random_f64(), random_f64(), random_f64())
}

pub(crate) fn random_vec3_in_interval(min: f64, max: f64) -> Vec3 {
    Vec3::new(
        random_f64_in_interval(min, max),
        random_f64_in_interval(min, max),
        random_f64_in_interval(min, max),
    )
}

pub(crate) fn random_color() -> Color {
    random_vec3().into()
}

pub(crate) fn random_color_in_interval(min: f64, max: f64) -> Color {
    random_vec3_in_interval(min, max).into()
}
