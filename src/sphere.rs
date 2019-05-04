use super::ray::Ray;
use super::reflexible::HitRecord;
use super::reflexible::Reflexible;
use super::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Reflexible for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction) as f64;
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - f64::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(t, p, normal));
            }
            let temp = (-b + f64::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(t, p, normal));
            }
        }

        None
    }
}
