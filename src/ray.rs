use crate::math::matrix4::Matrix4;
use crate::math::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    pub fn position(self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn transform(self, matrix: Matrix4) -> Self {
        Self {
            origin: matrix * self.origin,
            direction: matrix * self.direction,
        }
    }
}