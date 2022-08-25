use crate::float_near_equal;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::world::World;
use crate::EPSILON;

#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub reflectv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub n1: f64,
    pub n2: f64,
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
        let reflectv = r.direction.reflect(&normalv);
        let over_point = point + normalv * EPSILON;
        let under_point = point - normalv * EPSILON;
        Comp {
            object_id: self.object_id,
            point,
            eyev,
            normalv,
            reflectv,
            inside,
            over_point,
            under_point,
            n1: 0.0,
            n2: 0.0,
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct IntersectionList {
    ix: Vec<Intersection>,
    sorted: bool,
    pub hit_index: usize,
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
            hit_index: 0,
        }
    }

    /// # Panics
    ///
    /// may panic
    #[must_use]
    pub fn prepare_computation(&self, idx: usize, r: &Ray, w: &World) -> Comp {
        let mut containers: Vec<usize> = vec![];
        let mut result = self.ix[idx].prepare_computation(r, w);
        for (i, intersection) in self.ix.iter().enumerate() {
            if i == idx {
                if containers.is_empty() {
                    result.n1 = 1.0;
                } else {
                    let object = w.get_object(containers[containers.len() - 1]).unwrap();
                    result.n1 = object.material().refractive_index;
                }
            }
            let mut in_containers = false;
            for (j, object_id) in containers.iter().enumerate() {
                if *object_id == intersection.object_id {
                    containers.remove(j);
                    in_containers = true;
                    break;
                }
            }
            if !in_containers {
                containers.push(intersection.object_id);
            }
            if i == idx {
                if containers.is_empty() {
                    result.n2 = 1.0;
                } else {
                    let object = w.get_object(containers[containers.len() - 1]).unwrap();
                    result.n2 = object.material().refractive_index;
                }
                return result;
            }
        }
        result
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
        for idx in 0..self.ix.len() {
            let i = &self.ix[idx];
            if !float_near_equal(i.t, 0.0) && i.t > 0.0 {
                self.hit_index = idx;
                return Some(i);
            }
        }
        None
    }
}
