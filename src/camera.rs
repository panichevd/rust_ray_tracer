use std::{
    f64::INFINITY,
    fs::File,
    io::{Error, Write},
};

use crate::{
    physics::{write_color, Color, Point3, Ray, Vec3},
    shapes::Hittable,
    utils::{degrees_to_radians, random_f64, Interval},
};

pub(crate) struct Camera {
    samples_per_pixel: usize,
    image_width: usize,
    image_height: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub(crate) fn new(
        samples_per_pixel: usize,
        vfov: f64,
        aspect_ratio: f64,
        image_width: usize,
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
    ) -> Self {
        let image_height = ((image_width as f64) / aspect_ratio) as usize;

        let center = look_from.clone();

        // Determine viewport dimensions.
        let focal_length = (&look_from - &look_at).length();
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * ((image_width as f64) / (image_height as f64));

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (look_from - look_at).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u.clone();
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = &viewport_u / (image_width as f64);
        let pixel_delta_v = &viewport_v / (image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            &center - (focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (&pixel_delta_u + &pixel_delta_v);

        Camera {
            samples_per_pixel,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub(crate) fn render(&self, out_filename: &str, world: &dyn Hittable) -> Result<(), Error> {
        const MAX_DEPTH: u8 = 50;

        let mut out_file = File::create(out_filename)?;
        writeln!(out_file, "P3").unwrap();
        writeln!(out_file, "{} {} 255", self.image_width, self.image_height).unwrap();

        for j in 0..self.image_height {
            println!("scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let color =
                    (0..self.samples_per_pixel).fold(Color::new(0.0, 0.0, 0.0), |color, _| {
                        let r = self.get_ray(i, j);
                        color + Self::ray_color(&r, world, MAX_DEPTH)
                    });

                write_color(&mut out_file, &color, self.samples_per_pixel)?;
            }
        }

        println!("done.");
        Ok(())
    }

    // Get a randomly sampled camera ray for the pixel at location i,j,
    // originating from the camera defocus disk.
    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center = &self.pixel00_loc
            + ((i as f64) * &self.pixel_delta_u)
            + ((j as f64) * &self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_direction = pixel_sample - &self.center;
        Ray::new(&self.center, &ray_direction)
    }

    fn ray_color(r: &Ray, world: &dyn Hittable, depth: u8) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, INFINITY)) {
            if let Some((scattered, attenuation)) = rec.material.scatter(r, &rec) {
                return attenuation * Self::ray_color(&scattered, world, depth - 1);
            }
        }

        let unit_direction = r.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    // Returns a random point in the square surrounding a pixel at the origin.
    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_f64();
        let py = -0.5 + random_f64();
        (px * &self.pixel_delta_u) + (py * &self.pixel_delta_v)
    }
}
