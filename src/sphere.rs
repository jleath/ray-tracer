use crate::intersection::Intersection;
use crate::material::Material;
use crate::ray::Ray;
use crate::transform::Transform;
use crate::tuple::Tuple;

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    center: Tuple,
    radius: f64,
    pub transform: Transform,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

impl Sphere {
    #[must_use]
    pub fn new() -> Self {
        Sphere {
            center: Tuple::point(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Transform::new(),
            material: Material::new(),
        }
    }

    #[must_use]
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let t = self.transform.clone();
        let transformed_ray = t.inverse().transform_ray(ray);
        let sphere_to_ray = transformed_ray.origin - self.center;

        let a = transformed_ray
            .direction
            .dot_product(&transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot_product(&sphere_to_ray);
        let c = sphere_to_ray.dot_product(&sphere_to_ray) - 1.0;

        let discriminant = (b * b) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }
        let t1 = Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), self);
        let t2 = Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), self);
        if t2.t < t1.t {
            vec![t2, t1]
        } else {
            vec![t1, t2]
        }
    }

    #[must_use]
    pub fn normal_at(&self, point: Tuple) -> Tuple {
        let mut t = self.transform.clone().inverse();
        let object_point = t.transform(&point);
        let object_normal = object_point - self.center;
        t = t.transpose();
        let mut world_normal = t.transform(&object_normal);
        world_normal.w = 0.0;
        world_normal.normalize()
    }
}
