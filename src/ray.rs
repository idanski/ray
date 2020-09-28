use crate::{Color, Point3, Vec3};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(&self) -> Color {
        let unit_direction: Vec3 = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.);
        (1. - t)
            * Color {
                x: 1.,
                y: 1.,
                z: 1.,
            }
            + t * Color {
                x: 0.5,
                y: 0.7,
                z: 1.,
            }
    }
}
