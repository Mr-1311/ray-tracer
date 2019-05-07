use crate::ray::Ray;
use crate::reflexible::HitRecord;
use crate::vec3::Vec3;

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
