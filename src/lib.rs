extern crate image;

mod camera;
mod materials;
mod ray;
mod reflexible;
mod vec3;

use crate::materials::Scatterable;
use camera::Camera;
use materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material};
use rand::Rng;
use ray::Ray;
use reflexible::sphere::Sphere;
use reflexible::Reflexible;
use reflexible::ReflexibleList;
use vec3::Vec3;

pub fn ray_tracer() {
    let nx = 1000;
    let ny = 500;
    let ns = 100;

    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let sphere_1 = Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambertian(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
    );
    let sphere_2 = Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    );
    let sphere_3 = Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.6)),
    );
    let sphere_4 = Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Material::Dielectric(Dielectric::new(1.5)),
    );
    let sphere_5 = Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Material::Dielectric(Dielectric::new(1.5)),
    );

    let list: Vec<Box<dyn Reflexible>> = vec![
        Box::new(sphere_1),
        Box::new(sphere_2),
        Box::new(sphere_3),
        Box::new(sphere_4),
        Box::new(sphere_5),
    ];
    let world = ReflexibleList::new(list);

    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.0;

    let cam = Camera::new(
        &lookfrom,
        &lookat,
        &Vec3::new(0.0, 1.0, 0.0),
        50.0,
        f64::from(nx) / f64::from(ny),
        aperture,
        dist_to_focus,
    );

    let mut rng = rand::thread_rng();

    println!("render started and it can take some time.");

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let ur: f64 = rng.gen();
                let vr: f64 = rng.gen();
                let u: f64 = (i as f64 + ur) / nx as f64;
                let v: f64 = (j as f64 + vr) / ny as f64;

                let r = cam.get_ray(u, v);
                col = col + color(&r, &world, 0);
            }

            col = col / (ns as f64);
            col = Vec3::new(f64::sqrt(col.x), f64::sqrt(col.y), f64::sqrt(col.z));
            let ir = (255.99 * col.r()) as u8;
            let ig = (255.99 * col.g()) as u8;
            let ib = (255.99 * col.b()) as u8;

            let pixel = imgbuf.get_pixel_mut(i, ((j - ny + 1) as i32).abs() as u32);
            *pixel = image::Rgb([ir, ig, ib]);

            print!(
                "\rprogress ==> {}/{} = %{}",
                1 + i + nx * ((j - ny + 1) as i32).abs() as u32,
                nx * ny,
                (1 + i + nx * ((j - ny + 1) as i32).abs() as u32) * 100 / (nx * ny)
            );
        }
    }

    let _ = image::ImageRgb8(imgbuf).save("out.png");
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
