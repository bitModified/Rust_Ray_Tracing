use crate::color::Color;
use crate::intersection::{ComputedIntersection, Intersection};
use crate::light::Light;
use crate::material;
use crate::math::tuple::Tuple;
use crate::ray::Ray;
use crate::shape::Object;

const DEFAULT_ALLOWED_DEPTH: i32 = 8;

pub struct World {
    pub objects: Vec<Object>,
    lights: Vec<Light>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light)
    }

    pub fn add_object(&mut self, object: Object) -> usize {
        self.objects.push(object);
        self.objects.len() - 1
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        self.color_at_with_depth(ray, DEFAULT_ALLOWED_DEPTH)
    }

    pub fn color_at_with_depth(&self, ray: Ray, remaining_depth: i32) -> Color {
        let intersections = self.intersect(ray);

        let hit = Intersection::hit(&intersections);

        if let Some(i) = hit {
            self.shade_hit(i.prepare_computations(ray, &intersections), remaining_depth)
        } else {
            Color::black()
        }
    }

    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = self
            .objects
            .iter()
            .flat_map(|object| object.intersect(ray))
            .collect();

        intersections.sort_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap());

        intersections
    }

    fn shade_hit(&self, comps: ComputedIntersection, remaining_depth: i32) -> Color {
        let surface_color = self
            .lights
            .iter()
            .map(|light| {
                material::lighting(
                    comps.object.material(),
                    comps.object,
                    *light,

                    comps.over_point,
                    comps.eye_vector,
                    comps.normal_vector,
                    self.is_shadowed(comps.over_point, *light),
                )
            })
            .fold(Color::black(), |c1, c2| c1 + c2);

        let reflected_color = self.reflected_color(comps, remaining_depth);
        let refracted_color = self.refracted_color(comps, remaining_depth);

        let material = comps.object.material();

        if material.reflective > 0. && material.transparency > 0. {
            let reflectance = comps.schlick();

            surface_color + reflected_color * reflectance + refracted_color * (1. - reflectance)
        } else {
            surface_color + reflected_color + refracted_color
        }
    }

    fn is_shadowed(&self, point: Tuple, light: Light) -> bool {
        let vector = light.position - point;
        let distance = vector.magnitude();

        let ray = Ray::new(point, vector.normalize());

        Intersection::hit(&self.intersect(ray))
            // Check if light or object is closer
            .map(|hit| hit.object.material.casts_shadows && hit.t < distance)
            .unwrap_or(false)
    }

    fn reflected_color(&self, comps: ComputedIntersection, remaining_depth: i32) -> Color {
        let no_depth_remaining = remaining_depth <= 0;
        let default_color = Color::black();

        if no_depth_remaining {
            return default_color;
        }
        let reflective = comps.object.material().reflective;
        if reflective > 0. {
            let reflect_ray = Ray::new(comps.over_point, comps.reflect_vector);
            let color = self.color_at_with_depth(reflect_ray, remaining_depth - 1);

            color * reflective
        } else {
            default_color
        }
    }

    fn refracted_color(&self, comps: ComputedIntersection, remaining_depth: i32) -> Color {
        let object_is_opaque = comps.object.material().transparency == 0.;
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eye_vector.dot(comps.normal_vector);
        let sin2_t = n_ratio.powi(2) * (1. - cos_i.powi(2));
        let total_internal_reflection = sin2_t > 1.;

        if remaining_depth == 0 || object_is_opaque || total_internal_reflection {
            Color::black()
        } else {
            let cos_t = (1. - sin2_t).sqrt();
            let direction =
                comps.normal_vector * (n_ratio * cos_i - cos_t) - comps.eye_vector * n_ratio;

            let refract_ray = Ray::new(comps.under_point, direction);

            let color = self.color_at_with_depth(refract_ray, remaining_depth - 1)
                * comps.object.material().transparency;

            color
        }
    }
}