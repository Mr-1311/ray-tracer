mod ray;
mod reflexible;
mod sphere;
mod vec3;

use ray::Ray;
use reflexible::HitRecord;
use reflexible::Reflexible;
use reflexible::ReflexibleList;
use sphere::Sphere;
use vec3::Vec3;

pub fn ray_tracer() {
    let nx = 400;
    let ny = 200;

    let lover_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let sphere_1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let sphere_2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);

    let list: Vec<Box<dyn Reflexible>> = vec![Box::new(sphere_1), Box::new(sphere_2)];
    let world = ReflexibleList::new(list);

    println!("P3\n{} {}\n255", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u: f64 = i as f64 / nx as f64;
            let v: f64 = j as f64 / ny as f64;

            let r = Ray::new(origin, lover_left_corner + u * horizontal + v * vertical);

            let p = r.point_at_parameter(2.0);
            let col = color(&r, &world);

            let ir = (255.99 * col.r()) as i64;
            let ig = (255.99 * col.g()) as i64;
            let ib = (255.99 * col.b()) as i64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(r: &Ray, world: &Reflexible) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.0, std::f64::MAX) {
        return 0.5 * Vec3::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0);
    } else {
        let unit_direction = Vec3::unit_vector(r.direction);
        let t: f64 = 0.5 * (unit_direction.y + 1.0);

        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
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
