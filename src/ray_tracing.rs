use std::f64::consts::PI;
use std::time::Instant;

use rand::{Rng, thread_rng};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator};
use rayon::iter::ParallelIterator;

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
        Sphere::new(Vec3::new(1e6 + 1f64, 0.0, 0.0), 1e6, Material::Solid { color: Color { r: 0.0, g: 1.0, b: 0.0 }, illuminate: Color::zero() }),
        Sphere::new(Vec3::new(-1e6 - 1f64, 0.0, 0.0), 1e6, Material::Solid { color: Color { r: 1.0, g: 0.0, b: 0.0 }, illuminate: Color::zero() }),
        Sphere::new(Vec3::new(0.0, -1e6 - 1f64, 0.0), 1e6, Material::Solid { color: Color { r: 1.0, g: 1.0, b: 1.0 }, illuminate: Color::zero() }),
        Sphere::new(Vec3::new(0.0, 1e6 + 1f64, 0.0), 1e6, Material::Solid { color: Color { r: 1.0, g: 1.0, b: 1.0 }, illuminate: Color::zero() }),
        Sphere::new(Vec3::new(0.0, 0.0, -1e6 - 1f64), 1e6, Material::Solid { color: Color { r: 1.0, g: 1.0, b: 1.0 }, illuminate: Color::zero() }),
        Sphere::new(Vec3::new(0.5, -0.75, 0.5), 0.25, Material::Solid { color: Color { r: 1.0, g: 1.0, b: 1.0 }, illuminate: Color::zero() }),
        Sphere::new(Vec3::new(-0.5, -0.75, -0.5), 0.25, Material::Solid { color: Color { r: 1.0, g: 1.0, b: 1.0 }, illuminate: Color::zero() }),
        Sphere::new(Vec3::new(0.0, 1e3 + 0.9999, 0.0), 1e3, Material::Solid { color: Color::zero(), illuminate: Color { r: 1.0, g: 1.0, b: 1.0 } }),
    ];
    let mut result = Vec::with_capacity(width * height);
    let start = Instant::now();
    (0..height).map(|y| (0..width).map(move |x| (x, y))).flatten().collect::<Vec<_>>().into_par_iter().map(|(x, y)| {
        let mut rng = thread_rng();
        let mut color_sum = Color::zero();
        const COUNT: usize = 100;
        let x = x as f64;
        let y = y as f64;
        for _ in 0..COUNT {
            let mut ray = camera.create_ray(rng.gen_range(x..x + 1.0), rng.gen_range(y..y + 1.0));
            let mut throughput = Color { r: 1.0, g: 1.0, b: 1.0 };
            let mut light = Color { r: 0.0, g: 0.0, b: 0.0 };
            for _ in 0..10 {
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
                            collision.replace(current.clone());
                        }
                        _ => {}
                    }
                }
                if let Some((d, normal, material)) = collision {
                    let normal: Vec3<f64> = normal.into();
                    match material {
                        Material::Solid { color, illuminate } => {
                            ray = {
                                let x_inner = normal.inner_product(Vec3::new(1.0, 0.0, 0.0)).abs();
                                let y_inner = normal.inner_product(Vec3::new(0.0, 1.0, 0.0)).abs();
                                let u = if x_inner < y_inner {
                                    normal.outer_product(Vec3::new(1.0, 0.0, 0.0))
                                } else {
                                    normal.outer_product(Vec3::new(0.0, 1.0, 0.0))
                                };
                                let v = u.clone().outer_product(normal).normalize();
                                let u = u.normalize();

                                if false {
                                    let theta = rng.gen_range(0.0..PI / 2.0);
                                    let phi = rng.gen_range(0.0..PI * 2.0);

                                    let direction: Vec3<_> = ray.direction.into();
                                    let initial = ray.initial + direction * d + normal * 1e-4;
                                    let direction = u.vec() * phi.cos() * theta.sin() + v.vec() * phi.sin() * theta.sin() + normal * theta.sin();
                                    Ray {
                                        initial,
                                        direction: direction.normalize(),
                                    }
                                } else {
                                    let r: f64 = rng.gen_range(0.0..1.0);
                                    let r = r.sqrt();
                                    let phi = rng.gen_range(0.0..PI * 2.0);

                                    let direction: Vec3<_> = ray.direction.into();
                                    let initial = ray.initial + direction * d + normal * 1e-4;
                                    let direction = u.vec() * phi.cos() * r + v.vec() * phi.sin() * r + normal * (1.0 - r * r);
                                    Ray {
                                        initial,
                                        direction: direction.normalize(),
                                    }
                                }
                            };
                            light = light + throughput.clone() * illuminate;
                            throughput = throughput * color;
                        }
                    }
                } else {
                    break;
                }
                if throughput.r <= 1e-4 && throughput.g <= 1e-4 && throughput.b <= 1e-4 { break; }
            }
            color_sum = color_sum + light;
        }
        Color { r: color_sum.r / COUNT as f64, g: color_sum.g / COUNT as f64, b: color_sum.b / COUNT as f64 }
    }).collect_into_vec(&mut result);
    println!("Time: {}ms", start.elapsed().as_millis());
    for i in 0..width * height {
        buffer[i * 4] = (result[i].r.powf(1.0 / 2.5) * 256.0) as u8;
        buffer[i * 4 + 1] = (result[i].g.powf(1.0 / 2.5) * 256.0) as u8;
        buffer[i * 4 + 2] = (result[i].b.powf(1.0 / 2.5) * 256.0) as u8;
        buffer[i * 4 + 3] = 255;
    }
}
