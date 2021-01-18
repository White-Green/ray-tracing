use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3<V> {
    x: V,
    y: V,
    z: V,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct NormalizedVec3<V> {
    x: V,
    y: V,
    z: V,
}

impl<V> Vec3<V> {
    pub fn new(x: V, y: V, z: V) -> Self {
        Self { x, y, z }
    }

    pub fn inner_product<Rhs, P: Add<Output=P>>(self, rhs: Vec3<Rhs>) -> P
        where V: Mul<Rhs, Output=P> {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn outer_product<Rhs: Clone, P: Sub<Output=P>>(self, rhs: Vec3<Rhs>) -> Vec3<P>
        where V: Mul<Rhs, Output=P> + Clone {
        Vec3::new(
            self.y.clone() * rhs.z.clone() - self.z.clone() * rhs.y.clone(),
            self.z * rhs.x.clone() - self.x.clone() * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn squared_len<P: Add<Output=P>>(self) -> P
        where V: Mul<Output=P>, Self: Clone {
        self.clone().inner_product(self)
    }

    pub fn x(&self) -> &V {
        &self.x
    }

    pub fn y(&self) -> &V {
        &self.y
    }

    pub fn z(&self) -> &V {
        &self.z
    }
}

impl Vec3<f64> {
    pub fn normalize(self) -> NormalizedVec3<f64> {
        let len = self.squared_len().sqrt();
        NormalizedVec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

impl<V> From<NormalizedVec3<V>> for Vec3<V> {
    fn from(value: NormalizedVec3<V>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl<V> NormalizedVec3<V> {
    pub fn vec(self) -> Vec3<V> {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl<V: Add> Add for Vec3<V> {
    type Output = Vec3<V::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<V: Sub> Sub for Vec3<V> {
    type Output = Vec3<V::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<V: Neg> Neg for Vec3<V> {
    type Output = Vec3<V::Output>;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl<Rhs: Clone, V: Mul<Rhs>> Mul<Rhs> for Vec3<V> {
    type Output = Vec3<V::Output>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Vec3::new(self.x * rhs.clone(), self.y * rhs.clone(), self.z * rhs)
    }
}

impl<Rhs: Clone, V: Div<Rhs>> Div<Rhs> for Vec3<V> {
    type Output = Vec3<V::Output>;

    fn div(self, rhs: Rhs) -> Self::Output {
        Vec3::new(self.x / rhs.clone(), self.y / rhs.clone(), self.z / rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::Vec3;

    #[test]
    fn vec3_add_test() {
        assert_eq!(Vec3::new(1, 2, 3) + Vec3::new(5, 7, 9), Vec3::new(6, 9, 12));
    }

    #[test]
    fn vec3_sub_test() {
        assert_eq!(Vec3::new(1, 2, 3) - Vec3::new(5, 7, 9), Vec3::new(-4, -5, -6));
    }

    #[test]
    fn vec3_mul_test() {
        assert_eq!(Vec3::new(1, 2, 3) * 3, Vec3::new(3, 6, 9));
    }

    #[test]
    fn vec3_div_test() {
        assert_eq!(Vec3::new(3, 7, 11) / 3, Vec3::new(1, 2, 3));
    }

    #[test]
    fn vec3_inner_product_test() {
        assert_eq!(Vec3::new(1, 2, 3).inner_product(Vec3::new(5, 4, 6)), 5 + 8 + 18);
    }

    #[test]
    fn vec3_outer_product_test() {
        assert_eq!(Vec3::new(1, 2, 3).outer_product(Vec3::new(5, 4, 7)), Vec3::new(2, 8, -6));
    }

    #[test]
    fn vec3_normalize_test() {
        let normalized: Vec3<_> = Vec3::new(1f64, 2f64, 3f64).normalize().into();
        let len_expect = 3.7416573867739413855837487323165;
        let diff = normalized - Vec3::new(1f64 / len_expect, 2f64 / len_expect, 3f64 / len_expect);
        assert!(diff.x().abs() < 1e-3);
        assert!(diff.y().abs() < 1e-3);
        assert!(diff.z().abs() < 1e-3);
    }

    #[test]
    fn vec3_util_test() {
        assert_eq!(Vec3::new(1, 2, 3).x(), &1);
        assert_eq!(Vec3::new(1, 2, 3).y(), &2);
        assert_eq!(Vec3::new(1, 2, 3).z(), &3);
    }
}
