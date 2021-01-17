use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3<V> {
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
}
