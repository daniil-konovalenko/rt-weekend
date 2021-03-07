use std::fmt;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

use rand::{random, Rng};
use std::iter::Sum;
pub use Vec3 as Point;

impl Vec3 {
    pub fn zero() -> Self {
        Vec3 {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    #[inline]
    pub fn random() -> Self {
        Vec3::new(random(), random(), random())
    }

    pub fn random_unit() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    #[inline]
    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let vec = Self::random_range(-1.0, 1.0);
            if vec.length_squared() < 1.0 {
                return vec;
            }
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    #[inline]
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    #[inline]
    pub fn cross(&self, rhs: &Self) -> Self {
        let x = self.y * rhs.z - self.z * rhs.y;
        let y = self.z * rhs.x - self.x * rhs.z;
        let z = self.x * rhs.y - self.y * rhs.x;
        Self::new(x, y, z)
    }
    #[inline]
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn is_near_zero(&self) -> bool {
        let eps = 1e-8;

        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * 2.0 * self.dot(normal)
    }

    pub fn refract(&self, normal: &Self, refraction_ratio: f64) -> Self {
        let r_perpendicular = (*self + (*normal * -self.dot(normal))) * refraction_ratio;

        let r_parallel = -*normal * ((1.0 - r_perpendicular.length_squared()).sqrt());

        r_perpendicular + r_parallel
    }

    pub fn refract_ot(&self, normal: &Self, refraction_ratio: f64) -> Self {
        let cos_theta = f64::min(normal.dot(&-*self), 1.0);

        let r_perpendicular = (*self + *normal * cos_theta) * refraction_ratio;
        let r_parallel = *normal * (1.0 - r_perpendicular.length_squared()).abs().sqrt() * -1.0;
        r_perpendicular + r_parallel
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl fmt::Display for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Self {
        let mut result = Vec3::zero();
        for vec in iter {
            result += vec;
        }
        result
    }
}
