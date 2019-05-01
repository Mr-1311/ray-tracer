mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;

pub fn ray_tracer() {
    let nx = 400;
    let ny = 200;

    let lover_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
        
    println!("P3\n{} {}\n255",nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {

            let u: f64 = i as f64 / nx as f64;
            let v: f64 = j as f64 / ny as f64;

            let r = Ray::new(origin, lover_left_corner + u*horizontal + v*vertical);
            let col = color(&r);
            
            let ir = (255.99 * col.r()) as i64;
            let ig = (255.99 * col.g()) as i64;
            let ib = (255.99 * col.b()) as i64;
            
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(r: &Ray) -> Vec3 {
    let unit_direction = Vec3::unit_vector(r.direction);
    let t: f64 = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0) 
}
