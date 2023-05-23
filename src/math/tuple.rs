use crate::misc::approx_equal;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 1.0)
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 0.0)
    }

    pub fn is_point(self) -> bool {
        approx_equal(self.w, 1.0)
    }

    pub fn is_vector(self) -> bool {
        approx_equal(self.w, 0.0)
    }

    pub fn magnitude(self) -> f64 {
        let Self { x, y, z, .. } = self;
        assert!(self.is_vector());

        (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn magnitude_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn cross(self, other: Self) -> Self {
        // Don't know...protection for my limited knowledge.
        assert!(self.is_vector());
        assert!(other.is_vector());

        Self::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(self, normal: Tuple) -> Self {
        self - normal * 2. * self.dot(normal)
    }

    pub(crate) fn zip_with(&self, other: &Self, f: impl Fn(f64, f64) -> f64) -> Self {
        Self {
            x: f(self.x, other.x),
            y: f(self.y, other.y),
            z: f(self.z, other.z),
            w: f(self.w, other.w),
        }
    }

    pub(crate) fn min(&self, other: &Self) -> Self {
        self.zip_with(other, f64::min)
    }

    pub(crate) fn max(&self, other: &Self) -> Self {
        self.zip_with(other, f64::max)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        rhs * self
    }
}

impl PartialEq<Tuple> for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        approx_equal(self.x, other.x)
            && approx_equal(self.y, other.y)
            && approx_equal(self.z, other.z)
            && approx_equal(self.w, other.w)
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1. / rhs)
    }
}
