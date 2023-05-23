use crate::{intersection::Intersection, ray::Ray};

use super::{Object, SimpleObject};

#[derive(Clone, PartialEq, Debug)]
pub struct Csg {
    op: CsgOp,
    pub(crate) left: Box<Object>,
    pub(crate) right: Box<Object>,
}

impl Csg {
    fn new(op: CsgOp, left: Object, right: Object) -> Self {
        Self {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub(crate) fn union(left: Object, right: Object) -> Self {
        Self::new(CsgOp::Union, left, right)
    }

    pub(crate) fn intersection(left: Object, right: Object) -> Self {
        Self::new(CsgOp::Intersection, left, right)
    }

    pub(crate) fn difference(left: Object, right: Object) -> Self {
        Self::new(CsgOp::Difference, left, right)
    }

    pub(crate) fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        let left_intersections = self.left.intersect(local_ray);
        let right_intersections = self.right.intersect(local_ray);

        let mut xs = left_intersections
            .into_iter()
            .chain(right_intersections.into_iter())
            .collect::<Vec<_>>();
        xs.sort_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap());

        self.filter_intersections(xs)
    }

    #[allow(dead_code)]
    pub(crate) fn filter_intersections<'a>(
        &self,
        intersections: Vec<Intersection<'a>>,
    ) -> Vec<Intersection<'a>> {
        let mut inl = false;
        let mut inr = false;
        let mut result = vec![];

        for i in intersections {
            let left_hit = self.left.includes(i.object);

            if self.op.intersection_allowed(left_hit, inl, inr) {
                result.push(i);
            }

            if left_hit {
                inl = !inl;
            } else {
                inr = !inr;
            }
        }

        result
    }

    pub(crate) fn includes(&self, object: SimpleObject) -> bool {
        self.left.includes(object) || self.right.includes(object)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum CsgOp {
    Union,
    Intersection,
    Difference,
}

impl CsgOp {
    fn intersection_allowed(&self, left_hit: bool, inl: bool, inr: bool) -> bool {
        match self {
            CsgOp::Union => (left_hit && !inr) || (!left_hit && !inl),
            CsgOp::Intersection => (left_hit && inr) || (!left_hit && inl),
            CsgOp::Difference => (left_hit && !inr) || (!left_hit && inl),
        }
    }
}