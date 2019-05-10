extern crate image;

pub mod camera;
pub mod materials;
pub mod ray;
pub mod reflexible;
pub mod vec3;

use crate::materials::Scatterable;
use camera::Camera;
use rand::Rng;
use ray::Ray;
use reflexible::Reflexible;
use reflexible::ReflexibleList;
use vec3::Vec3;

use std::time::Instant;

pub fn ray_tracer(world: &ReflexibleList, cam: &Camera, nx: u32, ny: u32) {
    let ns = 100;

    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let mut rng = rand::thread_rng();

    println!("render started and it may take some time.");
    let now = Instant::now();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let ur: f64 = rng.gen();
                let vr: f64 = rng.gen();
                let u: f64 = (i as f64 + ur) / nx as f64;
                let v: f64 = (j as f64 + vr) / ny as f64;

                let r = cam.get_ray(u, v);
                col = col + color(&r, world, 0);
            }

            col = col / (ns as f64);
            col = Vec3::new(f64::sqrt(col.x), f64::sqrt(col.y), f64::sqrt(col.z));
            let ir = (255.99 * col.r()) as u8;
            let ig = (255.99 * col.g()) as u8;
            let ib = (255.99 * col.b()) as u8;

            let pixel = imgbuf.get_pixel_mut(i, ny - (j + 1));
            *pixel = image::Rgb([ir, ig, ib]);
            print!(
                "\rprogress ==> {}/{} = %{}",
                1 + i + nx * (ny - (j + 1)),
                nx * ny,
                (1 + i + nx * (ny - (j + 1))) * 100 / (nx * ny)
            );
        }
    }

    let duration = now.elapsed();

    let _ = image::ImageRgb8(imgbuf).save("output.jpg");

    println!(
        "\n\n'output.jpg' generated to the current path in {} milliseconds! ",
        duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );
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
