use crate::math::tuple::Tuple;
use crate::ray::Ray;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {}

impl Sphere {
    pub fn local_intersect(local_ray: Ray) -> Vec<f64> {
        let sphere_to_ray = local_ray.origin - Tuple::point(0., 0., 0.);
        let a = local_ray.direction.magnitude_squared();
        let b = 2. * local_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.magnitude_squared() - 1.;

        let discriminant = b.powi(2) - 4. * a * c;

        if discriminant < 0. {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2. * a);
            let t2 = (-b + discriminant.sqrt()) / (2. * a);

            vec![t1, t2]
        }
    }

    pub fn local_normal_at(local_point: Tuple) -> Tuple {
        // Warning: do not remove
        local_point - Tuple::point(0., 0., 0.)
    }
}