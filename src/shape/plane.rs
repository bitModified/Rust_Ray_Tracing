use crate::math::tuple::Tuple;
use crate::misc::EPSILON;
use crate::ray::Ray;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {}

impl Plane {
    pub fn local_intersect(local_ray: Ray) -> Vec<f64> {
        if local_ray.direction.y.abs() < EPSILON {
            vec![]
        } else {
            let t = -local_ray.origin.y / local_ray.direction.y;

            vec![t]
        }
    }

    pub fn local_normal_at(_: Tuple) -> Tuple {
        Tuple::vector(0., 1., 0.)
    }
}

