use crate::misc::approx_equal;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn black() -> Self {
        Self {
            red: 0.,
            green: 0.,
            blue: 0.,
        }
    }

    pub fn white() -> Self {
        Self {
            red: 1.,
            green: 1.,
            blue: 1.,
        }
    }

    #[allow(dead_code)]
    pub fn red() -> Self {
        Self {
            red: 1.,
            green: 0.,
            blue: 0.,
        }
    }

    #[allow(dead_code)]
    pub fn green() -> Self {
        Self {
            red: 0.,
            green: 1.,
            blue: 0.,
        }
    }

    #[allow(dead_code)]
    pub fn blue() -> Self {
        Self {
            red: 0.,
            green: 0.,
            blue: 1.,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        let d_red = self.red - other.red;
        let d_green = self.green - other.green;
        let d_blue = self.blue - other.blue;
        let dist_squared = d_red.powi(2) + d_green.powi(2) + d_blue.powi(2);

        approx_equal(dist_squared, 0.)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            red: self.red * scalar,
            green: self.green * scalar,
            blue: self.blue * scalar,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}