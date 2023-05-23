use crate::{math::tuple::Tuple, misc::EPSILON, ray::Ray, shape::BoundingBox};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub(crate) p1: Tuple,
    pub(crate) p2: Tuple,
    pub(crate) p3: Tuple,
    kind: TriangleKind,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum TriangleKind {
    Flat,
    Smooth { n1: Tuple, n2: Tuple, n3: Tuple },
}

impl Triangle {
    pub(crate) fn new(p1: Tuple, p2: Tuple, p3: Tuple) -> Self {
        Self {
            p1,
            p2,
            p3,
            kind: TriangleKind::Flat,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn smooth(p1: Tuple, p2: Tuple, p3: Tuple, n1: Tuple, n2: Tuple, n3: Tuple) -> Self {
        Self {
            p1,
            p2,
            p3,
            kind: TriangleKind::Smooth { n1, n2, n3 },
        }
    }

    fn edge1(&self) -> Tuple {
        self.p2 - self.p1
    }

    fn edge2(&self) -> Tuple {
        self.p3 - self.p1
    }

    fn normal(&self) -> Tuple {
        self.edge2().cross(self.edge1()).normalize()
    }

    pub(crate) fn local_normal_at(&self, uvt: &UVT) -> Tuple {
        let UVT { u, v, .. } = uvt;

        match self.kind {
            TriangleKind::Flat => self.normal(),
            TriangleKind::Smooth { n1, n2, n3 } => {
                (n2 * *u + n3 * *v + n1 * (1. - *u - *v)).normalize()
            }
        }
    }

    pub(crate) fn local_intersect(&self, local_ray: Ray) -> Vec<UVT> {
        let dir_cross_edge2 = local_ray.direction.cross(self.edge2());
        let det = self.edge1().dot(dir_cross_edge2);

        if det.abs() < EPSILON {
            return vec![];
        }

        let f = 1.0 / det;
        let p1_to_origin = local_ray.origin - self.p1;
        let u = f * p1_to_origin.dot(dir_cross_edge2);
        if u < 0. || u > 1. {
            return vec![];
        }

        let origin_cross_e1 = p1_to_origin.cross(self.edge1());
        let v = f * local_ray.direction.dot(origin_cross_e1);
        if v < 0. || (u + v) > 1. {
            return vec![];
        }

        let t = f * self.edge2().dot(origin_cross_e1);
        vec![UVT { u, v, t }]
    }

    pub(crate) fn bounding_box(&self) -> BoundingBox {
        BoundingBox::from_points(&[self.p1, self.p2, self.p3])
    }
}

#[derive(Clone, Copy)]
pub(crate) struct UVT {
    pub(crate) t: f64,
    pub(crate) u: f64,
    pub(crate) v: f64,
}