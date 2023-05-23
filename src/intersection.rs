use crate::math::tuple::Tuple;
use crate::misc::{approx_equal, EPSILON};
use crate::ray::Ray;
use crate::shape::triangle::UVT;
use crate::shape::SimpleObject;

pub(crate) enum TorUVT {
    JustT { t: f64 },
    UVT { uvt: UVT },
}

#[derive(Clone, Copy, Debug)]
pub struct Intersection<'a> {
    pub t: f64,
    uv: Option<(f64, f64)>,
    pub object: SimpleObject<'a>,
}

impl<'a> Intersection<'a> {
    pub(crate) fn new(t_or_uvt: &TorUVT, object: SimpleObject<'a>) -> Self {
        match t_or_uvt {
            &TorUVT::JustT { t } => Self {
                t,
                uv: None,
                object,
            },
            &TorUVT::UVT { uvt } => Self {
                t: uvt.t,
                uv: Some((uvt.u, uvt.v)),
                object,
            },
        }
    }

    // Returns intersection with the smallest non-negative t value.
    pub fn hit(intersections: &[Self]) -> Option<&Self> {
        intersections
            .iter()
            .filter(|i| (**i).t >= 0.)
            .min_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap())
    }

    pub(crate) fn prepare_computations(
        &self,
        ray: Ray,
        all_intersections: &[Intersection],
    ) -> ComputedIntersection {
        let object = self.object;
        let _t = self.t;
        let point = ray.position(self.t);
        let eye_vector = -ray.direction;

        let tentative_normal = self.object.normal_at(*self, point);

        let (_inside, normal_vector) = if tentative_normal.dot(eye_vector) < 0. {
            (true, -tentative_normal)
        } else {
            (false, tentative_normal)
        };

        let reflect_vector = ray.direction.reflect(normal_vector);
        let over_point = point + normal_vector * EPSILON;
        let under_point = point - normal_vector * EPSILON;

        let (n1, n2) = self.compute_refractive_indices(all_intersections);

        ComputedIntersection {
            eye_vector,
            normal_vector,
            reflect_vector,
            over_point,
            under_point,
            n1: n1,
            n2: n2,
            object,
            #[cfg(test)]
            inside: _inside,
            #[cfg(test)]
            t: _t,
            #[cfg(test)]
            point,
        }
    }

    fn compute_refractive_indices<'b>(
        &'a self,
        all_intersections: &[Intersection<'a>],
    ) -> (f64, f64)
    where
        'a: 'b,
    {
        let mut containers: Vec<SimpleObject<'b>> = vec![];
        let mut n1 = 1.0;
        let mut n2 = 1.0;

        for &i in all_intersections {

            let is_hit = i == *self;

            if is_hit {
                if let Some(last) = containers.last() {
                    n1 = last.material().refractive_index;
                } else {
                    n1 = 1.0;
                }
            }

            let position = containers.iter().position(|o| *o == i.object);

            if let Some(index) = position {
                containers.remove(index);
            } else {
                containers.push(i.object);
            }

            if is_hit {
                if let Some(last) = containers.last() {
                    n2 = last.material().refractive_index;
                } else {
                    n2 = 1.0;
                }
                break;
            }
        }

        (n1, n2)
    }

    pub(crate) fn uvt(&self) -> Option<UVT> {
        self.uv.map(|(u, v)| UVT { t: self.t, u, v })
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        approx_equal(self.t, other.t) && self.object == other.object
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct ComputedIntersection<'a> {
    pub object: SimpleObject<'a>,
    pub eye_vector: Tuple,
    pub normal_vector: Tuple,
    pub reflect_vector: Tuple,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub n1: f64,
    pub n2: f64,
    #[cfg(test)]
    t: f64,
    #[cfg(test)]
    point: Tuple,
    #[cfg(test)]
    inside: bool,
}

impl<'a> ComputedIntersection<'a> {
    pub fn schlick(&self) -> f64 {
        // find the cosine of the angle between the camera and normal vectors
        let mut cos = self.eye_vector.dot(self.normal_vector);

        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));

            if sin2_t > 1.0 {
                return 1.0;
            }
            // compute cosine of theta_t using trig identity
            let cos_t = (1. - sin2_t).sqrt();

            cos = cos_t
        }
        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);

        return r0 + (1. - r0) * (1. - cos).powi(5);
    }
}
