use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::world::World;
use crate::EPSILON;

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object_id: usize,
}

#[derive(Debug)]
pub struct Comp {
    pub object_id: usize,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

impl Intersection {
    #[must_use]
    pub fn new(t: f64, object_id: usize) -> Self {
        Intersection { t, object_id }
    }

    #[must_use]
    /// # Panics
    ///
    /// Will panic if the `Intersection` has an invalid `object_id`
    pub fn prepare_computation(&self, r: &Ray, world: &World) -> Comp {
        let t = self.t;
        let object = world.get_object(self.object_id).unwrap();
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
            object_id: self.object_id,
            point,
            eyev,
            normalv,
            inside,
            over_point,
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct IntersectionList {
    ix: Vec<Intersection>,
    sorted: bool,
}

impl IntersectionList {
    #[must_use]
    #[allow(clippy::ptr_arg)]
    /// # Panics
    ///
    /// May panic if the `Vec` passed into `new` contains `NaN`.
    pub fn new(ix: &Vec<Intersection>) -> Self {
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
