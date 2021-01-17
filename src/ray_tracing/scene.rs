use crate::geometry::NormalizedVec3;
use crate::ray_tracing::Ray;

pub mod camera;
pub mod object;

trait Collision {
    fn collision(&self, ray: &Ray) -> Option<(f64, NormalizedVec3<f64>)>;
}
