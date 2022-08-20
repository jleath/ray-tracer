use crate::color::Color;
use crate::intersection::{Comp, IntersectionList};
use crate::point_light::PointLight;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

use std::fmt;

#[derive(Debug)]
pub struct InvalidWorldAccess;

impl fmt::Display for InvalidWorldAccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid id for world item")
    }
}

impl std::error::Error for InvalidWorldAccess {}

#[derive(Clone, PartialEq, Debug)]
pub struct World {
    objects: Vec<Sphere>,
    lights: Vec<PointLight>,
}

impl Default for World {
    fn default() -> Self {
        World::default_world()
    }
}

impl World {
    #[must_use]
    pub fn new() -> Self {
        Self {
            objects: vec![],
            lights: vec![],
        }
    }

    #[must_use]
    pub fn default_world() -> Self {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
        s2.transform = s2.transform.scale(0.5, 0.5, 0.5);
        let mut new_world = Self::new();
        new_world.add_object(s1);
        new_world.add_object(s2);
        new_world.add_light(light);
        new_world
    }

    #[must_use]
    pub fn get_object(&self, id: usize) -> Option<&Sphere> {
        if id < self.objects.len() {
            Some(&self.objects[id])
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_light(&self, id: usize) -> Option<&PointLight> {
        if id < self.lights.len() {
            Some(&self.lights[id])
        } else {
            None
        }
    }

    /// # Errors
    ///
    /// Returns an error if id is not valid
    pub fn set_light(
        &mut self,
        id: usize,
        new_light: &PointLight,
    ) -> Result<(), InvalidWorldAccess> {
        if id < self.lights.len() {
            self.lights[id] = new_light.clone();
            Ok(())
        } else {
            Err(InvalidWorldAccess)
        }
    }

    /// # Errors
    ///
    /// Returns an error if id is not valid
    pub fn set_object(&mut self, id: usize, new_object: &Sphere) -> Result<(), InvalidWorldAccess> {
        if id < self.objects.len() {
            self.objects[id] = new_object.clone();
            Ok(())
        } else {
            Err(InvalidWorldAccess)
        }
    }

    pub fn add_object(&mut self, o: Sphere) -> usize {
        self.objects.push(o);
        self.objects.len() - 1
    }

    pub fn add_light(&mut self, l: PointLight) -> usize {
        self.lights.push(l);
        self.lights.len() - 1
    }

    #[must_use]
    pub fn intersect(&self, r: &Ray) -> IntersectionList {
        let mut intersections = Vec::new();
        for obj in &self.objects {
            let mut xs = obj.intersect(r);
            intersections.append(&mut xs);
        }
        IntersectionList::new(&intersections)
    }

    #[must_use]
    pub fn shade_hit(&self, comps: &Comp) -> Color {
        let material = comps.object.material;
        let mut color = Color::new(0.0, 0.0, 0.0);
        for light in &self.lights {
            color += material.lighting(light, comps.point, comps.eyev, comps.normalv);
        }
        color
    }

    #[must_use]
    pub fn color_at(&self, r: &Ray) -> Color {
        let mut ix = self.intersect(r);
        if let Some(hit) = ix.hit() {
            let comps = hit.prepare_computation(r);
            self.shade_hit(&comps)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }
}
