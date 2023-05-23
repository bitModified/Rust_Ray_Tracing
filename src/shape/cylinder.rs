use std::f64::{INFINITY, NEG_INFINITY};

use crate::{math::tuple::Tuple, misc::EPSILON, ray::Ray};

#[derive(Clone, Copy, Debug)]
pub struct Cylinder {
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

impl Cylinder {
    pub fn new() -> Self {
        Self {
            minimum: NEG_INFINITY,
            maximum: INFINITY,
            closed: false,
        }
    }

    pub fn local_intersect(&self, ray: Ray) -> Vec<f64> {
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

        if a.abs() < EPSILON {
            return self.intersect_caps(ray);
        }

        let b = 2. * ray.origin.x * ray.direction.x + 2. * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.;

        let disc = b.powi(2) - 4. * a * c;

        if disc < 0. {
            vec![]
        } else {
            let t0 = (-b - disc.sqrt()) / (2. * a);
            let t1 = (-b + disc.sqrt()) / (2. * a);

            let y0 = ray.origin.y + t0 * ray.direction.y;
            let y1 = ray.origin.y + t1 * ray.direction.y;
            let mut xs = Vec::with_capacity(2);

            if self.minimum < y0 && y0 < self.maximum {
                xs.push(t0);
            }

            if self.minimum < y1 && y1 < self.maximum {
                xs.push(t1);
            }

            xs.append(&mut self.intersect_caps(ray));
            xs
        }
    }

    pub fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        let dist = local_point.x.powi(2) + local_point.z.powi(2);

        if dist < 1. && local_point.y >= self.maximum - EPSILON {
            Tuple::vector(0., 1., 0.)
        } else if dist < 1. && local_point.y <= self.minimum + EPSILON {
            Tuple::vector(0., -1., 0.)
        } else {
            Tuple::vector(local_point.x, 0., local_point.z)
        }
    }

    fn intersect_caps(&self, ray: Ray) -> Vec<f64> {
        let mut xs = Vec::with_capacity(2);

        if !self.closed || ray.direction.y.abs() < EPSILON {
            return xs;
        }

        let t_min = (self.minimum - ray.origin.y) / ray.direction.y;
        let t_max = (self.maximum - ray.origin.y) / ray.direction.y;

        [t_min, t_max]
            .into_iter()
            .filter(|t| check_cap(ray, *t))
            .for_each(|t| xs.push(t));

        xs
    }
}

fn check_cap(ray: Ray, t: f64) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;
    let r = 1.;

    x.powi(2) + z.powi(2) <= r
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Self) -> bool {
        //This is wrong and bad, don't want to use (==) on float64s and I don't think you can use approx_equal 
        //becuase of infinities
        self.minimum == other.minimum && self.maximum == other.maximum
    }
}

