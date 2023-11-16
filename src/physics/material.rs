use crate::{
    physics::{Color, Ray, Vec3},
    shapes::HitRecord,
    utils::random_f64,
};

pub(crate) trait Material: std::fmt::Debug {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Debug)]
pub(crate) struct LambertianMaterial {
    albedo: Color,
}

impl LambertianMaterial {
    pub(crate) fn new(albedo: Color) -> Self {
        LambertianMaterial { albedo }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = &hit_record.normal + Vec3::new_random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal.clone();
        }

        let scattered = Ray::new(&hit_record.p, &scatter_direction);
        Some((scattered, self.albedo.clone()))
    }
}

#[derive(Debug)]
pub(crate) struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub(crate) fn new(albedo: Color, f: f64) -> Self {
        let fuzz = if f < 1.0 { f } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = r_in.direction().unit().reflect(&hit_record.normal);
        let scattered = Ray::new(
            &hit_record.p,
            &(reflected + self.fuzz * &Vec3::new_random_unit()),
        );

        if scattered.direction().dot(&hit_record.normal) > 0.0 {
            Some((scattered, self.albedo.clone()))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub(crate) struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub(crate) fn new(ir: f64) -> Self {
        Dielectric { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> bool {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        let refl = r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
        refl > random_f64()
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit();
        let cos_theta = (-unit_direction.dot(&hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) {
            unit_direction.reflect(&hit_record.normal)
        } else {
            unit_direction.refract(&hit_record.normal, refraction_ratio)
        };

        let scattered = Ray::new(&hit_record.p, &direction);
        Some((scattered, attenuation))
    }
}
