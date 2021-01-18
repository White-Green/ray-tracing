use crate::geometry::{NormalizedVec3, Vec3};
use crate::ray_tracing::Ray;

pub struct Camera {
    position: Vec3<f64>,
    direction_forward: Vec3<f64>/*z*/,
    direction_bottom: Vec3<f64>/*y*/,
    direction_right: Vec3<f64>/*x*/,
    inverse_direction_x: Vec3<f64>,
    inverse_direction_y: Vec3<f64>,
    inverse_direction_z: Vec3<f64>,
    width: usize,
    height: usize,
    unit_per_pixel: f64,
}

impl Camera {
    pub fn new(position: Vec3<f64>,
               direction_forward: NormalizedVec3<f64>,
               direction_bottom: NormalizedVec3<f64>,
               direction_right: NormalizedVec3<f64>,
               width: usize,
               height: usize,
               fov: f64) -> Self {
        let direction_forward = direction_forward.into();
        let direction_bottom = direction_bottom.into();
        let direction_right = direction_right.into();
        let (inverse_direction_x,
            inverse_direction_y,
            inverse_direction_z, ) = {
            let r: Vec3<_> = direction_right;
            let b: Vec3<_> = direction_bottom;
            let f: Vec3<_> = direction_forward;
            let det = (
                r.x() * b.y() * f.z()
                    + r.y() * b.z() * f.x()
                    + r.z() * b.x() * f.y()
                    - r.x() * b.z() * f.y()
                    - r.y() * b.x() * f.z()
                    - r.z() * b.y() * f.x()
            ).abs() / 6.0;
            (
                Vec3::new((b.y() * f.z() - b.z() * f.y()) / det,
                          -(b.x() * f.z() - b.z() * f.x()) / det,
                          (b.x() * f.y() - b.y() * f.x()) / det),
                Vec3::new(-(r.y() * f.z() - r.z() * f.y()) / det,
                          (r.x() * f.z() - r.z() * f.x()) / det,
                          -(r.x() * f.y() - r.y() * f.x()) / det),
                Vec3::new((r.y() * b.z() - r.z() * b.y()) / det,
                          -(r.x() * b.z() - r.z() * b.x()) / det,
                          (r.x() * b.y() - r.y() * b.x()) / det)//あとで行列計算は別に分ける
            )
        };
        Self {
            position,
            direction_forward,
            direction_bottom,
            direction_right,
            inverse_direction_x,
            inverse_direction_y,
            inverse_direction_z,
            width,
            height,
            unit_per_pixel: fov.sin() / (width / 2) as f64,
        }
    }

    pub fn create_ray(&self, x: f64, y: f64) -> Ray {
        let x = x - (self.width / 2) as f64;
        let y = y - (self.height / 2) as f64;
        Ray {
            initial: self.position,
            direction: (self.direction_forward + self.direction_right * x * self.unit_per_pixel + self.direction_bottom * y * self.unit_per_pixel).normalize(),
        }
    }

    pub fn transform_direction(&self, direction: NormalizedVec3<f64>) -> NormalizedVec3<f64> {
        let direction: Vec3<_> = direction.into();
        Vec3::new(
            self.inverse_direction_x.inner_product(direction),
            self.inverse_direction_y.inner_product(direction),
            self.inverse_direction_z.inner_product(direction),
        ).normalize()
    }

    pub fn transform_position(&self, position: Vec3<f64>) -> Vec3<f64> {
        position - self.position
    }
}
