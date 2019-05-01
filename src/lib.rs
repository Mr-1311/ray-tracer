mod vec3;
mod ray;

use vec3::Vec3;

pub fn generate_img() {
    let nx = 400;
    let ny = 300;

    let mut col = Vec3::new(0.0,0.0,0.2);
    
    println!("P3\n{} {}\n255",nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {

            col.x = i as f64 / nx as f64;
            col.y = j as f64 / ny as f64;
            
            let ir :i64 = (255.99 * col.r()) as i64;
            let ig :i64 = (255.99 * col.g()) as i64;
            let ib :i64 = (255.99 * col.b()) as i64;
            
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
