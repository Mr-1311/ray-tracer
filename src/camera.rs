use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(look_from: &Vec3, look_at: &Vec3, vup: &Vec3, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = Vec3::unit_vector(*look_from - *look_at);
        let u = Vec3::unit_vector(Vec3::cross(*vup, w));
        let v = Vec3::cross(w, u);

        Camera {
            lower_left_corner: *look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: *look_from,
        }
    }

    pub fn default() -> Camera {
        Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin.clone(),
            self.lower_left_corner.clone() + u * self.horizontal + v * self.vertical
                - self.origin.clone(),
        )
    }
}
