use crate::ray::Ray;
use crate::reflexible::HitRecord;
use crate::vec3::Vec3;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub trait Scatterable {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Material {
    Lambertian(lambertian::Lambertian),
    Metal(metal::Metal),
    Dielectric(dielectric::Dielectric),
}

impl Scatterable for Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(r_in, rec, attenuation, scattered),
            Material::Metal(ref inner) => inner.scatter(r_in, rec, attenuation, scattered),
            Material::Dielectric(ref inner) => inner.scatter(r_in, rec, attenuation, scattered),
        }
    }
}

//#[derive(Clone, Copy, Debug, PartialEq)]
//pub struct Material {
//    lambertian: lambertian::Lambertian,
//    metal: metal::Metal,
//}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot(&v, &n) * n
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64, refracted: &mut Vec3) -> bool {
    let uv = Vec3::unit_vector(*v);
    let dt = Vec3::dot(&uv, &n);

    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        *refracted = ni_over_nt * (uv - (*n) * dt) - (*n) * f64::sqrt(discriminant);
        return true;
    }

    false
}

fn schlick(cosine: f64, ref_idx: f64) -> f64{
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powf((1.0 - cosine), 5.0)
}
