use crate::color::Color;
use crate::math::tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
}

impl Light {
    pub fn point_light(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

