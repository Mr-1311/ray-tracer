extern crate ray_tracer;

use ray_tracer::camera::Camera;
use ray_tracer::materials::{
    dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material,
};
use ray_tracer::reflexible::sphere::Sphere;
use ray_tracer::reflexible::Reflexible;
use ray_tracer::reflexible::ReflexibleList;
use ray_tracer::vec3::Vec3;

use rand::Rng;

fn main() {
    //screen resolution
    let nx = 1920;
    let ny = 1080;

    let mut rng = rand::thread_rng();

    let sphere_ground = Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(Lambertian::new(Vec3::new(0.4, 0.6, 0.4))),
    );

    let mut list: Vec<Box<dyn Reflexible>> = vec![Box::new(sphere_ground)];

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();

            let center = Vec3::new(
                f64::from(a) + 0.9 * rng.gen::<f64>(),
                0.2,
                f64::from(b) + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                //diffuse
                if choose_mat < 0.5 {
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian(Lambertian::new(Vec3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        ))),
                    )));
                //metal
                } else if choose_mat < 0.95 {
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Metal(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            0.1 * rng.gen::<f64>(),
                        )),
                    )));
                } else {
                    // dielectric
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    list.push(Box::new(Sphere::new(
        Vec3::new(-4., 5., -7.),
        5.0,
        Material::Metal(Metal::new(Vec3::new(0.4, 0.2, 0.1), 0.0)),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(5.9, 5., -9.),
        5.0,
        Material::Dielectric(Dielectric::new(1.5)),
    )));

    let world = ReflexibleList::new(list);

    let lookfrom = Vec3::new(0.0, 0.5, 7.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.0;

    let cam = Camera::new(
        &lookfrom,
        &lookat,
        &Vec3::new(0.0, 1.0, 0.0),
        30.0,
        f64::from(nx) / f64::from(ny),
        aperture,
        dist_to_focus,
    );

    ray_tracer::ray_tracer(&world, &cam, nx, ny, "output.jpg");
}
