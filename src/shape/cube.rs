use std::f64::INFINITY;

use crate::{math::tuple::Tuple, misc::EPSILON, ray::Ray};

pub struct Cube;

impl Cube {
    pub fn local_intersect(local_ray: Ray) -> Vec<f64> {
        local_intersect(
            Tuple::point(-1., -1., -1.),
            Tuple::point(1., 1., 1.),
            local_ray,
        )
    }

    pub fn local_normal_at(local_point: Tuple) -> Tuple {
        let max_abs = [local_point.x, local_point.y, local_point.z]
            .iter()
            .copied()
            .map(f64::abs)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        if max_abs == local_point.x.abs() {
            Tuple::vector(local_point.x, 0., 0.)
        } else if max_abs == local_point.y.abs() {
            Tuple::vector(0., local_point.y, 0.)
        } else {
            Tuple::vector(0., 0., local_point.z)
        }
    }
}

pub fn local_intersect(min: Tuple, max: Tuple, local_ray: Ray) -> Vec<f64> {
    let (xt_min, xt_max) = check_axis(min.x, max.x, local_ray.origin.x, local_ray.direction.x);
    let (yt_min, yt_max) = check_axis(min.y, max.y, local_ray.origin.y, local_ray.direction.y);
    let (zt_min, zt_max) = check_axis(min.z, max.z, local_ray.origin.z, local_ray.direction.z);

    let t_min = *[xt_min, yt_min, zt_min]
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let t_max = *[xt_max, yt_max, zt_max]
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    if t_min > t_max {
        vec![]
    } else {
        vec![t_min, t_max]
    }
}

fn check_axis(min: f64, max: f64, origin: f64, direction: f64) -> (f64, f64) {
    let t_min_numerator = min - origin;
    let t_max_numerator = max - origin;

    let (mut t_min, mut t_max) = if direction.abs() >= EPSILON {
        (t_min_numerator / direction, t_max_numerator / direction)
    } else {
        (t_min_numerator * INFINITY, t_max_numerator * INFINITY)
    };

    if t_min > t_max {
        std::mem::swap(&mut t_min, &mut t_max)
    }

    (t_min, t_max)
}
