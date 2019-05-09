use super::Scatterable;
use crate::ray::Ray;
use crate::reflexible::HitRecord;
use crate::vec3::Vec3;

use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx }
    }
}

impl Scatterable for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut rng = rand::thread_rng();

        let outward_normal: Vec3;
        let reflected = super::reflect(r_in.direction, rec.normal);
        let ni_over_nt: f64;

        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut refracted = Vec3::new(0.0, 0.0, 0.0);
        let reflect_prob;
        let cosine;

        if Vec3::dot(&r_in.direction, &rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine =
                self.ref_idx * Vec3::dot(&r_in.direction, &rec.normal) / r_in.direction.length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -Vec3::dot(&r_in.direction, &rec.normal) / r_in.direction.length();
        }

        if super::refract(&r_in.direction, &outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = super::schlick(cosine, self.ref_idx);
        } else {
            *scattered = Ray::new(rec.p, reflected);
            reflect_prob = 1.0;
        }
        let r: f64 = rng.gen();
        if r < reflect_prob {
            *scattered = Ray::new(rec.p, reflected);
        } else {
            *scattered = Ray::new(rec.p, refracted);
        }

        true
    }
}
