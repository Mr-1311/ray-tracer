use super::ray::Ray;
use super::vec3::Vec3;
use rand::Rng;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: &Vec3,
        look_at: &Vec3,
        vup: &Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = Vec3::unit_vector(*look_from - *look_at);
        let u = Vec3::unit_vector(Vec3::cross(*vup, w));
        let v = Vec3::cross(w, u);
        let lower_left_corner = *look_from
            - u * half_width * focus_dist
            - v * half_height * focus_dist
            - w * focus_dist;
        let horizontal = 2.0 * focus_dist * u * half_width;
        let vertical = 2.0 * focus_dist * v * half_height;
        Camera {
            origin: *look_from,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
            _w: w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin.clone() - offset,
            self.lower_left_corner.clone() + s * self.horizontal + t * self.vertical
                - self.origin.clone()
                - offset,
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut p: Vec3;
    let mut rng = rand::thread_rng();

    loop {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();

        p = 2.0 * Vec3::new(x, y, 0.0) - Vec3::new(1.0, 1.0, 0.0);

        if Vec3::dot(&p, &p) >= 1.0 {
            break;
        }
    }
    p
}
