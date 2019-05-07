mod camera;
mod materials;
mod ray;
mod reflexible;
mod vec3;

use crate::materials::Scatterable;
use camera::Camera;
use materials::{lambertian::Lambertian, metal::Metal, Material};
use rand::Rng;
use ray::Ray;
use reflexible::sphere::Sphere;
use reflexible::Reflexible;
use reflexible::ReflexibleList;
use vec3::Vec3;

pub fn ray_tracer() {
    let nx = 400;
    let ny = 200;
    let ns = 100;

    let sphere_1 = Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
    );
    let sphere_2 = Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    );
    let sphere_3 = Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0)),
    );
    let sphere_4 = Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Material::Metal(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3)),
    );

    let list: Vec<Box<dyn Reflexible>> = vec![
        Box::new(sphere_1),
        Box::new(sphere_2),
        Box::new(sphere_3),
        Box::new(sphere_4),
    ];
    let world = ReflexibleList::new(list);

    let cam = Camera::default();

    let mut rng = rand::thread_rng();

    println!("P3\n{} {}\n255", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let ur: f64 = rng.gen();
                let vr: f64 = rng.gen();
                let u: f64 = (i as f64 + ur) / nx as f64;
                let v: f64 = (j as f64 + vr) / ny as f64;

                let r = cam.get_ray(u, v);
                let p = r.point_at_parameter(2.0);
                col = col + color(&r, &world, 0);
            }

            col = col / (ns as f64);
            col = Vec3::new(f64::sqrt(col.x), f64::sqrt(col.y), f64::sqrt(col.z));
            let ir = (255.99 * col.r()) as i64;
            let ig = (255.99 * col.g()) as i64;
            let ib = (255.99 * col.b()) as i64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(r: &Ray, world: &Reflexible, depth: i64) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, std::f64::MAX) {
        let mut scattered: Ray = Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0));
        let mut attenuation: Vec3 = Vec3::new(1.0, 1.0, 1.0);

        if depth < 50
            && rec
                .material
                .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * color(&scattered, world, depth + 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = Vec3::unit_vector(r.direction);
        let t: f64 = 0.5 * (unit_direction.y + 1.0);

        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    let mut rng = rand::thread_rng();

    loop {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        let z: f64 = rng.gen();
        p = 2.0 * Vec3::new(x, y, z) - Vec3::new(0.0, 0.0, 0.0);

        if p.squared_length() >= 1.0 {
            break;
        }
    }
    p
}

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - *center;
    let a = ray.direction.dot(&ray.direction);
    let b = (2.0 * ray.direction.dot(&oc)) as f64;
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    match discriminant < -1.0 {
        true => -1.0,
        false => (-b - f64::sqrt(discriminant)) / (2.0 * a),
    }
}
