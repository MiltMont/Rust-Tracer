use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // Dot product
    pub fn dot(&self, other: &Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    // Norm
    pub fn norm(&self) -> f64 {
        self.dot(&self).sqrt()
    }

    pub fn norm_square(&self) -> f64 {
        self.dot(&self)
    }

    // Normalize
    pub fn normalize(&self) -> Vec3 {
        self / self.norm()
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        self + &(-rhs)
    }
}

// Scalar multiplication implementation
// TODO: Refactor this using generics!
impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

// Division by a scalar
impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, lhs: f64) -> Self::Output {
        Vec3::new(self.x / lhs, self.y / lhs, self.z / lhs)
    }
}

// "Division" by another vector
impl Div<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_addition() {
        let vec1 = Vec3::new(1., 1., 1.);
        let vec2 = Vec3::new(2., 2., 2.);

        let vec3 = &vec1 + &vec2;

        k9::snapshot!(vec3, "(3, 3, 3)");
    }

    #[test]
    fn test_neg() {
        let vec1 = Vec3::new(2., 2., 1.);
        let vec2 = Vec3::new(23.2, 2., 1.);
        let neg_vec1 = -vec1;
        let neg_vec2 = -&vec2;

        k9::snapshot!(neg_vec1, "(-2, -2, -1)");
        k9::snapshot!(neg_vec2, "(-23.2, -2, -1)");
    }

    #[test]
    fn test_substraction() {
        let vec1 = Vec3::new(1., 1., 1.);
        let vec2 = Vec3::new(2., 2., 2.);
        let vec3 = &vec1 - &vec2;

        k9::snapshot!(vec3, "(-1, -1, -1)");
    }

    #[test]
    fn test_scalarmul() {
        let vec1 = Vec3::new(23., 12., 12.);
        let vec2 = 2. * &vec1;

        k9::snapshot!(vec2, "(46, 24, 24)");
    }

    #[test]
    fn test_division_by_scalar() {
        let vec1 = Vec3::new(23., 12., 12.);
        let vec2 = &vec1 / 3.;

        k9::snapshot!(vec2, "(7.666666666666667, 4, 4)");
    }

    #[test]
    fn test_vector_divison() {
        let vec1 = Vec3::new(4., 8., 16.);
        let vec2 = Vec3::new(2., 2., 2.);
        let vec3 = &vec1 / &vec2;
        k9::snapshot!(vec3, "(2, 4, 8)");
    }

    #[test]
    fn test_dot_product() {
        let vec1 = Vec3::new(1., 0., 0.);
        let vec2 = Vec3::new(0., 1., 0.);

        let dot = vec1.dot(&vec2);

        k9::snapshot!(dot, "0.0");
        k9::assert_equal!(dot, 0.);
    }

    #[test]
    fn test_norm() {
        let vec1 = Vec3::new(0., 1., 0.);
        let norm = vec1.norm();

        k9::snapshot!(norm, "1.0");
        k9::assert_equal!(norm, 1.);
    }

    #[test]
    fn test_normalize() {
        let vec1 = Vec3::new(1.3, 45.23, 23.0);
        let unit_vector = vec1.normalize();

        k9::snapshot!(&vec1, "(1.3, 45.23, 23)");
        k9::snapshot!(
            &unit_vector,
            "(0.025611386573193257, 0.8910792420811776, 0.453124531679573)"
        );
        // This is approx 1.
        k9::snapshot!(unit_vector.norm(), "0.9999999999999999");
    }
}
