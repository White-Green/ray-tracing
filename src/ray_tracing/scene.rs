use crate::geometry::NormalizedVec3;
use crate::ray_tracing::Ray;
use crate::ray_tracing::scene::material::Material;

pub mod camera;
pub mod object;
pub mod material;

pub trait Collision {
    fn collision(&self, ray: &Ray) -> Option<(f64, NormalizedVec3<f64>, Material)>;
}
