mod camera;
mod physics;
mod shapes;
mod utils;

use std::{env, io::Error, rc::Rc};

use crate::{
    camera::Camera,
    physics::{Color, Dielectric, LambertianMaterial, Material, Metal, Point3, Vec3},
    shapes::{HittableList, Sphere},
    utils::{random_color, random_color_in_interval, random_f64, random_f64_in_interval},
};

fn generate_material() -> Rc<dyn Material> {
    let choose_mat = random_f64();
    if choose_mat < 0.8 {
        // diffuse
        let albedo = random_color() * random_color();
        Rc::new(LambertianMaterial::new(albedo))
    } else if choose_mat < 0.95 {
        //metal
        let albedo = random_color_in_interval(0.5, 1.0);
        let fuzz = random_f64_in_interval(0.0, 0.5);
        Rc::new(Metal::new(albedo, fuzz))
    } else {
        //glass
        Rc::new(Dielectric::new(1.5))
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: ray_tracer <file>");
        return Ok(());
    }
    let out_filename = &args[1];

    let mut world = HittableList::default();

    let material_ground = Rc::new(LambertianMaterial::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in (-11..11).map(|a| a as f64) {
        for b in (-11..11).map(|b| b as f64) {
            let center = Point3::new(a + 0.9 * random_f64(), 0.2, b + 0.9 * random_f64());
            if (&center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = generate_material();
                world.add(Rc::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(LambertianMaterial::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    const SAMPLES_PER_PIXEL: usize = 500;
    const VFOV: f64 = 20.0;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 1200;

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);

    let camera = Camera::new(
        SAMPLES_PER_PIXEL,
        VFOV,
        ASPECT_RATIO,
        IMAGE_WIDTH,
        look_from,
        look_at,
        view_up,
    );
    camera.render(out_filename, &world)
}
