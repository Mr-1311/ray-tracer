use super::Scatterable;
use crate::ray::Ray;
use crate::reflexible::HitRecord;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo }
    }
}

impl Scatterable for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = super::reflect(Vec3::unit_vector(r_in.direction), rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo.clone();
        Vec3::dot(&scattered.direction, &rec.normal) > 0.0
    }
}
