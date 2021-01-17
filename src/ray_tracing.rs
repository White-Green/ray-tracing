use std::f64::consts::PI;

use crate::geometry::{NormalizedVec3, Vec3};
use crate::ray_tracing::scene::camera::Camera;
use crate::ray_tracing::scene::Collision;
use crate::ray_tracing::scene::material::{Color, Material};
use crate::ray_tracing::scene::object::Sphere;

pub mod scene;

pub struct Ray {
    initial: Vec3<f64>,
    direction: NormalizedVec3<f64>,
}

pub fn draw(buffer: &mut [u8], width: usize, height: usize) {
    let camera = Camera::new(Vec3::new(0.0, 0.0, 4.0),
                             Vec3::new(0.0, 0.0, -1.0).normalize(),
                             Vec3::new(0.0, -1.0, 0.0).normalize(),
                             Vec3::new(1.0, 0.0, 0.0).normalize(),
                             width, height, PI / 4.0);
    let objects = [
        Sphere::new(Vec3::new(1e6 + 1f64, 0.0, 0.0), 1e6, Material::SolidColor(Color { r: 0.0, g: 1.0, b: 0.0 })),
        Sphere::new(Vec3::new(-1e6 - 1f64, 0.0, 0.0), 1e6, Material::SolidColor(Color { r: 1.0, g: 0.0, b: 0.0 })),
        Sphere::new(Vec3::new(0.0, -1e6 - 1f64, 0.0), 1e6, Material::SolidColor(Color { r: 1.0, g: 1.0, b: 1.0 })),
        Sphere::new(Vec3::new(0.0, 1e6 + 1f64, 0.0), 1e6, Material::SolidColor(Color { r: 1.0, g: 1.0, b: 1.0 })),
        Sphere::new(Vec3::new(0.0, 1e3 + 0.9999, 0.0), 1e3, Material::SolidColor(Color { r: 1.0, g: 1.0, b: 0.0 })),
        Sphere::new(Vec3::new(0.0, 0.0, -1e6 - 1f64), 1e6, Material::SolidColor(Color { r: 1.0, g: 1.0, b: 1.0 })),
        Sphere::new(Vec3::new(0.5, -0.75, 0.5), 0.25, Material::SolidColor(Color { r: 0.5, g: 0.5, b: 0.5 })),
        Sphere::new(Vec3::new(-0.5, -0.75, -0.5), 0.25, Material::SolidColor(Color { r: 0.5, g: 0.5, b: 0.5 })),
    ];
    for y in 0..height {
        for x in 0..width {
            let ray = camera.create_ray(x, y);
            let mut collision: Option<(f64, NormalizedVec3<f64>, Material)> = None;
            for object in &objects {
                let current = object.collision(&ray);
                match (&mut collision, &current) {
                    (Some(collision), Some(current)) => {
                        if current.0 < collision.0 {
                            *collision = current.clone();
                        }
                    }
                    (ref mut collision, Some(current)) => {
                        **collision = Some(current.clone());
                    }
                    _ => {}
                }
            }
            if let Some((_, normal, material)) = collision {
                let normal: Vec3<f64> = normal.into();
                match &material {
                    Material::SolidColor(color) => {
                        let cos = -normal.inner_product(ray.direction.clone().into());
                        buffer[(y * width + x) * 4] = (color.r * 256.0 * cos) as u8;
                        buffer[(y * width + x) * 4 + 1] = (color.g * 256.0 * cos) as u8;
                        buffer[(y * width + x) * 4 + 2] = (color.b * 256.0 * cos) as u8;
                        buffer[(y * width + x) * 4 + 3] = 255;
                    }
                }
            } else {
                buffer[(y * width + x) * 4] = 255;
                buffer[(y * width + x) * 4 + 1] = 255;
                buffer[(y * width + x) * 4 + 2] = 255;
                buffer[(y * width + x) * 4 + 3] = 255;
            }
        }
    }
}
