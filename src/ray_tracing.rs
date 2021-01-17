use std::f64::consts::PI;

use crate::geometry::{NormalizedVec3, Vec3};
use crate::ray_tracing::scene::camera::Camera;
use crate::ray_tracing::scene::Collision;
use crate::ray_tracing::scene::object::Sphere;

pub mod scene;

pub struct Ray {
    initial: Vec3<f64>,
    direction: NormalizedVec3<f64>,
}

pub fn draw(buffer: &mut [u8], width: usize, height: usize) {
    let camera = Camera::new(Vec3::new(3.0, 0.0, 0.0),
                             Vec3::new(-1.0, 0.0, 0.0).normalize(),
                             Vec3::new(0.0, -1.0, 0.0).normalize(),
                             Vec3::new(0.0, 0.0, 1.0).normalize(),
                             width, height, PI / 4.0);
    let objects = [
        Sphere::new(Vec3::new(0.0, 0.0, 0.5), 1.0),
        Sphere::new(Vec3::new(0.0, 0.0, -0.5), 1.0)
    ];
    for y in 0..height {
        for x in 0..width {
            let ray = camera.create_ray(x, y);
            let mut collision: Option<(f64, NormalizedVec3<f64>)> = None;
            for object in &objects {
                let current = object.collision(&ray);
                match (&mut collision, current) {
                    (Some(collision), Some(current)) => {
                        if current.0 < collision.0 {
                            *collision = current;
                        }
                    }
                    (collision, Some(current)) => {
                        *collision = Some(current);
                    }
                    _ => {}
                }
            }
            if let Some((_, normal)) = collision {
                let normal: Vec3<_> = camera.transform_direction(normal).into();
                buffer[(y * width + x) * 4] = ((normal.x() + 1.0) * 127.0) as u8;
                buffer[(y * width + x) * 4 + 1] = ((normal.y() + 1.0) * 127.0) as u8;
                buffer[(y * width + x) * 4 + 2] = ((normal.z() + 1.0) * 127.0) as u8;
                buffer[(y * width + x) * 4 + 3] = 255;
            } else {
                buffer[(y * width + x) * 4] = 255;
                buffer[(y * width + x) * 4 + 1] = 255;
                buffer[(y * width + x) * 4 + 2] = 255;
                buffer[(y * width + x) * 4 + 3] = 255;
            }
            // let yy = y as f64 - (height / 2) as f64;
            // let xx = x as f64 - (width / 2) as f64;
            // let radius = (width.min(height) / 2) as f64;
            // if xx * xx + yy * yy <= radius * radius {
            //     buffer[(y * width + x) * 4] = 255;
            //     buffer[(y * width + x) * 4 + 3] = 255;
            // }
        }
    }
}
