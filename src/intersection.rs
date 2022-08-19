use crate::sphere::Sphere;

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    #[must_use]
    pub fn new(t: f64, object: &'a Sphere) -> Self {
        Intersection { t, object }
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct IntersectionList<'a> {
    ix: Vec<&'a Intersection<'a>>,
    sorted: bool,
}

impl<'a> IntersectionList<'a> {
    #[must_use]
    /// # Panics
    ///
    /// May panic if the `Vec` passed into `new` contains `NaN`.
    pub fn new(ix: &'a [&Intersection]) -> Self {
        let mut list = ix.to_owned();
        list.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        IntersectionList {
            ix: list,
            sorted: true,
        }
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
