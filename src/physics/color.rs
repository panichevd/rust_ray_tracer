use std::{
    fs::File,
    io::{Error, Write},
};

use crate::{physics::Vec3, utils::Interval};

#[derive(Clone, Debug, Default)]
pub(crate) struct Color(Vec3);

impl Color {
    pub(crate) fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Color(Vec3::new(e0, e1, e2))
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Color(value)
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign<&Color> for Color {
    fn add_assign(&mut self, rhs: &Color) {
        self.0 += &rhs.0
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(self * rhs.0)
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(self.0 * rhs.0)
    }
}

pub(crate) fn write_color(
    out_file: &mut File,
    color: &Color,
    samples_per_pixel: usize,
) -> Result<(), Error> {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = color.0.x() * scale;
    let g = color.0.y() * scale;
    let b = color.0.z() * scale;

    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    let intensity = Interval::new(0.0, 0.999);

    writeln!(
        out_file,
        "{} {} {}",
        (256.0 * intensity.clamp(r)) as usize,
        (256.0 * intensity.clamp(g)) as usize,
        (256.0 * intensity.clamp(b)) as usize,
    )
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}
