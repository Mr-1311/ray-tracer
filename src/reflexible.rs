use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord { t, p, normal }
    }

    pub fn zero() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

pub trait Reflexible {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct ReflexibleList {
    pub list: Vec<Box<dyn Reflexible>>,
}

impl ReflexibleList {
    pub fn new(list: Vec<Box<dyn Reflexible>>) -> ReflexibleList {
        ReflexibleList { list }
    }
}

impl Reflexible for ReflexibleList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for object in &self.list {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                hit_record = Some(hit);
                closest_so_far = hit.t;
            }
        }

        hit_record
    }
}
