mod ray;
mod vec3;
use vec3::Vec3;

type Point3 = Vec3;

type Color = Vec3;

impl Color {
    pub fn write_color(&self) -> String {
        format!("{} {} {} ", self.x as i32, self.y as i32, self.z as i32)
    }
}

const IMG_WIDTH: i32 = 256;
const IMG_HEIGHT: i32 = 256;

fn main() {
    // PPM metadata
    println!("P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);

    let b = 0.25 * 255.9;

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("scanlines remaining: {}", j);
        for i in 0..IMG_WIDTH {
            let r = (i as f64 / (IMG_WIDTH as f64 - 1.0)) as f64 * 255.99;
            let g = (j as f64 / (IMG_HEIGHT as f64 - 1.0)) as f64 * 255.99;

            let pixel_color = Color { x: r, y: g, z: b };
            println!("{}", pixel_color.write_color())
        }
    }
    eprintln!("Done rendering!");
}
