mod ray;
mod vec3;

use pixels::{Pixels, SurfaceTexture};
use ray::Ray;
use vec3::Vec3;
use winit::{event::Event, event_loop::EventLoop};

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

    pub fn as_u8_buffer(&self) -> [u8; 4] {
        [
            (self.x * 255.99) as u8,
            (self.y * 255.99) as u8,
            (self.z * 255.99) as u8,
            0xFF,
        ]
    }

    pub fn red() -> Color {
        Color {
            x: 1.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn white() -> Color {
        Color {
            x: 1.,
            y: 1.,
            z: 1.,
        }
    }

    pub fn black() -> Color {
        Color {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

// Image
const ASPECT_RATIO: f64 = 16. / 9.;
const IMG_WIDTH: i32 = 400;
const IMG_HEIGHT: i32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as i32;

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    )
}

pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> bool {
    let oc: Vec3 = ray.origin - center;
    let a = dot(&ray.direction, &ray.direction);
    let b = 2. * dot(&oc, &ray.direction);
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    discriminant > 0.
}

pub fn ray_color(ray: &Ray) -> Color {
    let sphere_center: Point3 = Point3::new(0., 0., -1.);
    let sphere_radius: f64 = 0.5;

    if hit_sphere(sphere_center, sphere_radius, ray) {
        return Color::red();
    }

    let unit_direction: Vec3 = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.);
    (1. - t) * Color::white()
        + t * Color {
            x: 0.5,
            y: 0.7,
            z: 1.,
        }
}

fn main() {
    let event_loop = EventLoop::new();
    let mut input = winit_input_helper::WinitInputHelper::new();

    let window = {
        let size = winit::dpi::LogicalSize::new(IMG_WIDTH, IMG_HEIGHT);
        winit::window::WindowBuilder::new()
            .with_title("Ray Tracing")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let surface_texture = SurfaceTexture::new(IMG_WIDTH as u32, IMG_HEIGHT as u32, &window);
    let mut pixels = Pixels::new(IMG_WIDTH as u32, IMG_HEIGHT as u32, surface_texture).unwrap();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            draw(pixels.get_frame());
            pixels.render().unwrap();
        }

        if input.update(&event) {
            // handle quit
            if input.key_pressed(winit::event::VirtualKeyCode::Escape) || input.quit() {
                *control_flow = winit::event_loop::ControlFlow::Exit;
                return;
            }
        }

        window.request_redraw();
    })
}

fn draw(frame: &mut [u8]) {
    let mut frame_iter = frame.chunks_exact_mut(4);

    // Camera
    let viewport_height = 2.;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.;

    let origin = Point3::zeroed();
    let horizontal: Vec3 = Vec3::new(viewport_width, 0., 0.);
    let vertical: Vec3 = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    for j in (0..IMG_HEIGHT).rev() {
        for i in 0..IMG_WIDTH {
            let u: f64 = i as f64 / (IMG_WIDTH as f64 - 1.);
            let v: f64 = j as f64 / (IMG_HEIGHT as f64 - 1.);

            let r: Ray = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            let pixel_color = ray_color(&r);

            let pixel = frame_iter.next().unwrap();
            pixel.copy_from_slice(&pixel_color.as_u8_buffer());
        }
    }
}
