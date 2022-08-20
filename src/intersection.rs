use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use crate::EPSILON;

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

pub struct Comp<'a> {
    pub object: &'a Sphere,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

impl<'a> Intersection<'a> {
    #[must_use]
    pub fn new(t: f64, object: &'a Sphere) -> Self {
        Intersection { t, object }
    }

    #[must_use]
    pub fn prepare_computation(&self, r: &Ray) -> Comp {
        let t = self.t;
        let object = self.object;
        let point = r.position(t);
        let eyev = -r.direction;
        let mut normalv = object.normal_at(point);
        let mut inside = false;
        if normalv.dot_product(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }
        let over_point = point + normalv * EPSILON;
        Comp {
            object,
            point,
            eyev,
            normalv,
            inside,
            over_point,
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct IntersectionList<'a> {
    ix: Vec<Intersection<'a>>,
    sorted: bool,
}

impl<'a> IntersectionList<'a> {
    #[must_use]
    #[allow(clippy::ptr_arg)]
    /// # Panics
    ///
    /// May panic if the `Vec` passed into `new` contains `NaN`.
    pub fn new(ix: &Vec<Intersection<'a>>) -> Self {
        let mut list = ix.clone();
        list.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        IntersectionList {
            ix: list,
            sorted: true,
        }
    }

    #[must_use]
    pub fn get(&self, i: usize) -> Option<&Intersection> {
        if i < self.ix.len() {
            Some(&self.ix[i])
        } else {
            None
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.ix.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ix.is_empty()
    }

    #[must_use]
    /// # Panics
    ///
    /// Will panic if the `IntersectionList`'s ix field contains an intersection with t set to NaN
    pub fn hit(&mut self) -> Option<&Intersection> {
        if !self.sorted {
            self.ix
                .sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
            self.sorted = true;
        }
        for i in &self.ix {
            if i.t >= 0.0 {
                return Some(i);
            }
        }
        None
    }
}
