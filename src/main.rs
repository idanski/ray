mod ray;
mod vec3;
use ray::Ray;
use vec3::Vec3;

type Point3 = Vec3;
impl Point3 {
    pub fn zeroed() -> Point3 {
        Point3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

type Color = Vec3;

impl Color {
    pub fn write_color(&self) -> String {
        format!(
            "{} {} {} ",
            (self.x * 255.99) as i32,
            (self.y * 255.99) as i32,
            (self.z * 255.99) as i32
        )
    }
}

// Image
const ASPECT_RATIO: f64 = 16. / 9.;
const IMG_WIDTH: i32 = 400;
const IMG_HEIGHT: i32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as i32;

fn main() {
    // Camera
    let viewport_height = 2.;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.;

    let origin = Point3::zeroed();
    let horizontal: Vec3 = Vec3::new(viewport_width, 0., 0.);
    let vertical: Vec3 = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    // Render
    // PPM metadata
    println!("P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("scanlines remaining: {}", j);
        for i in 0..IMG_WIDTH {
            let u: f64 = i as f64 / (IMG_WIDTH as f64 - 1.);
            let v: f64 = j as f64 / (IMG_HEIGHT as f64 - 1.);

            let r: Ray = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            let pixel_color = r.ray_color();
            println!("{}", pixel_color.write_color())
        }
    }
    eprintln!("Done rendering!");
}
