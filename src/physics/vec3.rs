use std::{
    fmt::Display,
    ops::{Mul, MulAssign},
    slice::SliceIndex,
};

use crate::utils::{random_f64_in_interval, random_vec3_in_interval};

#[derive(Clone, Debug)]
pub(crate) struct Vec3 {
    e: [f64; 3],
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
}

impl<Idx> std::ops::Index<Idx> for Vec3
where
    Idx: SliceIndex<[f64], Output = f64>,
{
    type Output = f64;

    fn index(&self, index: Idx) -> &Self::Output {
        self.e.index(index)
    }
}

impl<Idx> std::ops::IndexMut<Idx> for Vec3
where
    Idx: SliceIndex<[f64], Output = f64>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.e.index_mut(index)
    }
}

impl std::ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

impl std::ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl std::ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        self.add(&rhs)
    }
}

impl std::ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        (&self).add(rhs)
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl std::ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl std::ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl std::ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self.sub(&rhs)
    }
}

impl std::ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        (&self).sub(rhs)
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self.sub(&rhs)
    }
}

impl std::ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl std::ops::Mul<Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.mul(&rhs)
    }
}

impl std::ops::Mul<&Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        (&self).mul(&rhs)
    }
}

impl std::ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl std::ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs.mul(self)
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.mul(&rhs)
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl std::ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self.mul(1.0 / rhs)
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (&self).div(rhs)
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.mul_assign(1.0 / rhs);
    }
}

impl Vec3 {
    pub(crate) fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub(crate) fn new_random_unit() -> Self {
        Self::new_random_in_unit_sphere().unit()
    }

    #[allow(dead_code)]
    pub(crate) fn new_random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(
                random_f64_in_interval(-1.0, 1.0),
                random_f64_in_interval(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub(crate) fn unit(&self) -> Vec3 {
        self / self.length()
    }

    pub(crate) fn x(&self) -> f64 {
        self.e[0]
    }

    pub(crate) fn y(&self) -> f64 {
        self.e[1]
    }

    pub(crate) fn z(&self) -> f64 {
        self.e[2]
    }

    pub(crate) fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub(crate) fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub(crate) fn dot(&self, rhs: &Vec3) -> f64 {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
    }

    // Return true if the vector is close to zero in all dimensions.
    pub(crate) fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    pub(crate) fn reflect(&self, normal: &Vec3) -> Vec3 {
        self - 2.0 * self.dot(normal) * normal
    }

    pub(crate) fn refract(&self, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-self.dot(normal)).min(1.0);
        let r_out_perpend = etai_over_etat * (self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perpend.length_squared()).abs().sqrt() * normal;
        r_out_perpend + r_out_parallel
    }

    fn new_random_in_unit_sphere() -> Self {
        loop {
            let p = random_vec3_in_interval(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub(crate) fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
            self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
            self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
        )
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

pub(crate) type Point3 = Vec3;
