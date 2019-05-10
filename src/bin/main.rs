extern crate ray_tracer;

use ray_tracer::camera::Camera;
use ray_tracer::materials::{
    dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material,
};
use ray_tracer::reflexible::sphere::Sphere;
use ray_tracer::reflexible::Reflexible;
use ray_tracer::reflexible::ReflexibleList;
use ray_tracer::vec3::Vec3;

fn main() {
    let nx = 1000;
    let ny = 900;

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
        Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.1)),
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

    let lookfrom = Vec3::new(0.0, 5.0, 5.0);
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

    ray_tracer::ray_tracer(&world, &cam, nx, ny);
}
