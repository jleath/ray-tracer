use crate::color::Color;
use crate::float_near_equal;
use crate::intersection::{Comp, IntersectionList};
use crate::point_light::PointLight;
use crate::ray::Ray;
use crate::shape::Shape;
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
    objects: Vec<Shape>,
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
        let mut s1 = Shape::sphere();
        s1.set_color(Color::new(0.8, 1.0, 0.6));
        s1.set_diffuse(0.7);
        s1.set_specular(0.2);
        let mut s2 = Shape::sphere();
        s2.scale(0.5, 0.5, 0.5);
        let mut new_world = Self::new();
        new_world.add_object(s1);
        new_world.add_object(s2);
        new_world.add_light(light);
        new_world
    }

    #[must_use]
    pub fn get_object(&self, id: usize) -> Option<&Shape> {
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
        new_light: &mut PointLight,
    ) -> Result<(), InvalidWorldAccess> {
        if id < self.lights.len() {
            new_light.id = id;
            self.lights[id] = *new_light;
            Ok(())
        } else {
            Err(InvalidWorldAccess)
        }
    }

    pub fn add_object(&mut self, mut o: Shape) -> usize {
        let id = self.objects.len();
        o.set_id(id);
        self.objects.push(o);
        id
    }

    pub fn add_light(&mut self, mut l: PointLight) -> usize {
        l.id = self.lights.len();
        self.lights.push(l);
        self.lights.len() - 1
    }

    #[must_use]
    pub fn intersect(&self, r: &Ray) -> IntersectionList {
        let mut intersections = Vec::new();
        for i in 0..self.objects.len() {
            if let Some(mut xs) = self.objects[i].intersect(r) {
                intersections.append(&mut xs);
            }
        }
        IntersectionList::new(&intersections)
    }

    #[must_use]
    /// # Panics
    ///
    /// Will panic if `comp` has an invalid value for `object_id`
    // This does not work very well for multiple light sources. It will render the shadows
    // appropriately but the shading won't look as realistic as it could. Need to look into
    // maybe some kind of lighten only color blending instead of just color addition.
    pub fn shade_hit(&self, comps: &Comp) -> Color {
        let object = self.get_object(comps.object_id).unwrap();
        let material = object.material();
        let mut color = Color::new(0.0, 0.0, 0.0);
        for light in &self.lights {
            let shadowed = self.is_shadowed(comps.over_point, light);
            color += material.lighting(
                light,
                object,
                comps.point,
                comps.eyev,
                comps.normalv,
                shadowed,
            );
            color += self.reflected_color(comps);
        }
        color
    }

    #[must_use]
    /// # Panics
    ///
    /// Will panic if `comp` has an invalid value for `object_id`
    pub fn reflected_color(&self, comps: &Comp) -> Color {
        let object = self.get_object(comps.object_id).unwrap();
        let material = object.material();
        if float_near_equal(material.reflective, 0.0) {
            return Color::new(0.0, 0.0, 0.0);
        }
        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(&reflect_ray);
        color * material.reflective
    }

    #[must_use]
    pub fn color_at(&self, r: &Ray) -> Color {
        let mut ix = self.intersect(r);
        if let Some(hit) = ix.hit() {
            let comps = hit.prepare_computation(r, self);
            self.shade_hit(&comps)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }

    #[must_use]
    pub fn is_shadowed(&self, p: Tuple, light: &PointLight) -> bool {
        let v = light.position - p;
        let distance = v.magnitude();
        let shadow_ray = Ray::new(p, v.normalize());
        let mut ix = self.intersect(&shadow_ray);
        if let Some(hit) = ix.hit() {
            if hit.t < distance {
                return true;
            }
        }
        false
    }
}
