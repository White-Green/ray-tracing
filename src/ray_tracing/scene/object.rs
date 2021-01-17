use crate::geometry::{NormalizedVec3, Vec3};
use crate::ray_tracing::Ray;
use crate::ray_tracing::scene::Collision;
use crate::ray_tracing::scene::material::Material;

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Vec3<f64>,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3<f64>, radius: f64, material: Material) -> Self {
        Self { center, radius, material }
    }
}

impl Collision for Sphere {
    fn collision(&self, ray: &Ray) -> Option<(f64, NormalizedVec3<f64>, Material)> {
        let d: Vec3<_> = ray.direction.into();
        let c = self.center - ray.initial;
        let a = d.squared_len();
        let half_b = -d.inner_product(c);
        let c = c.squared_len() - self.radius * self.radius;
        if half_b * half_b - a * c < 0.0 {
            None
        } else {
            let x = (-half_b - (half_b * half_b - a * c).sqrt()) / a;
            let x = if x > 0.0 {
                x
            } else {
                (-half_b + (half_b * half_b - a * c).sqrt()) / a
            };
            if x > 0f64 {
                Some((x, (ray.initial + d * x - self.center).normalize(), self.material.clone()))
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::Vec3;
    use crate::ray_tracing::Ray;
    use crate::ray_tracing::scene::Collision;
    use crate::ray_tracing::scene::material::{Color, Material};
    use crate::ray_tracing::scene::object::Sphere;

    #[test]
    fn sphere_collision_test() {
        let sphere = Sphere::new(Vec3::new(0f64, 0f64, 0f64), 1.0, Material::SolidColor(Color { r: 0f64, g: 0f64, b: 0f64 }));
        let collision = sphere.collision(&Ray { initial: Vec3::new(2f64, 0f64, 0f64), direction: Vec3::new(-1f64, 0f64, 0f64).normalize() });
        if let Some((x, normal)) = collision {
            let normal: Vec3<_> = normal.into();
            assert!((x - 1.0).abs() < 1e-3);
            assert!((normal.x() - 1.0).abs() < 1e-3);
            assert!((normal.y() - 0.0).abs() < 1e-3);
            assert!((normal.z() - 0.0).abs() < 1e-3);
        } else {
            unreachable!()
        }

        let collision = sphere.collision(&Ray { initial: Vec3::new(0f64, 2f64, 0f64), direction: Vec3::new(0f64, -1f64, 0f64).normalize() });
        if let Some((x, normal)) = collision {
            let normal: Vec3<_> = normal.into();
            assert!((x - 1.0).abs() < 1e-3);
            assert!((normal.x() - 0.0).abs() < 1e-3);
            assert!((normal.y() - 1.0).abs() < 1e-3);
            assert!((normal.z() - 0.0).abs() < 1e-3);
        } else {
            unreachable!()
        }

        let collision = sphere.collision(&Ray { initial: Vec3::new(0f64, 0f64, 2f64), direction: Vec3::new(0f64, 0f64, -1f64).normalize() });
        if let Some((x, normal)) = collision {
            let normal: Vec3<_> = normal.into();
            assert!((x - 1.0).abs() < 1e-3);
            assert!((normal.x() - 0.0).abs() < 1e-3);
            assert!((normal.y() - 0.0).abs() < 1e-3);
            assert!((normal.z() - 1.0).abs() < 1e-3);
        } else {
            unreachable!()
        }


        let sphere = Sphere::new(Vec3::new(2f64, 0f64, 0f64), 1.0, Material::SolidColor(Color { r: 0f64, g: 0f64, b: 0f64 }));
        let collision = sphere.collision(&Ray { initial: Vec3::new(0f64, 0f64, 0f64), direction: Vec3::new(1f64, 0f64, 0f64).normalize() });
        if let Some((x, normal)) = collision {
            let normal: Vec3<_> = normal.into();
            assert!((x - 1.0).abs() < 1e-3);
            assert!((normal.x() + 1.0).abs() < 1e-3);
            assert!((normal.y() - 0.0).abs() < 1e-3);
            assert!((normal.z() - 0.0).abs() < 1e-3);
        } else {
            unreachable!()
        }

        let sphere = Sphere::new(Vec3::new(0f64, 2f64, 0f64), 1.0, Material::SolidColor(Color { r: 0f64, g: 0f64, b: 0f64 }));
        let collision = sphere.collision(&Ray { initial: Vec3::new(0f64, 0f64, 0f64), direction: Vec3::new(0f64, 1f64, 0f64).normalize() });
        if let Some((x, normal)) = collision {
            let normal: Vec3<_> = normal.into();
            assert!((x - 1.0).abs() < 1e-3);
            assert!((normal.x() - 0.0).abs() < 1e-3);
            assert!((normal.y() + 1.0).abs() < 1e-3);
            assert!((normal.z() - 0.0).abs() < 1e-3);
        } else {
            unreachable!()
        }

        let sphere = Sphere::new(Vec3::new(0f64, 0f64, 2f64), 1.0, Material::SolidColor(Color { r: 0f64, g: 0f64, b: 0f64 }));
        let collision = sphere.collision(&Ray { initial: Vec3::new(0f64, 0f64, 0f64), direction: Vec3::new(0f64, 0f64, 1f64).normalize() });
        if let Some((x, normal)) = collision {
            let normal: Vec3<_> = normal.into();
            assert!((x - 1.0).abs() < 1e-3);
            assert!((normal.x() - 0.0).abs() < 1e-3);
            assert!((normal.y() - 0.0).abs() < 1e-3);
            assert!((normal.z() + 1.0).abs() < 1e-3);
        } else {
            unreachable!()
        }

        let sphere = Sphere::new(Vec3::new(0f64, 0f64, 0f64), 1.0, Material::SolidColor(Color { r: 0f64, g: 0f64, b: 0f64 }));
        let collision = sphere.collision(&Ray { initial: Vec3::new(0f64, 0f64, 0f64), direction: Vec3::new(1f64, 0f64, 0f64).normalize() });
        if let Some((x, normal)) = collision {
            let normal: Vec3<_> = normal.into();
            assert!((x - 1.0).abs() < 1e-3);
            assert!((normal.x() - 1.0).abs() < 1e-3);
            assert!((normal.y() - 0.0).abs() < 1e-3);
            assert!((normal.z() - 0.0).abs() < 1e-3);
        } else {
            unreachable!()
        }
    }
}
