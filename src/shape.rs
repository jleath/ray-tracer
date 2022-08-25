use crate::color::Color;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::pattern::Pattern;
use crate::ray::Ray;
use crate::transform::Transform;
use crate::tuple::Tuple;
use crate::EPSILON;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShapeType {
    Sphere,
    Plane,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    material: Material,
    transform: Transform,
    id: Option<usize>,
    kind: ShapeType,
}

// TODO: make materials and transforms copies instead of references

impl Shape {
    #[must_use]
    pub fn sphere() -> Shape {
        Shape {
            material: Material::new(),
            transform: Transform::new(),
            id: None,
            kind: ShapeType::Sphere,
        }
    }

    #[must_use]
    pub fn glass_sphere() -> Self {
        let mut s = Shape::sphere();
        s.set_transparency(1.0);
        s.set_refractive_index(1.5);
        s
    }

    #[must_use]
    pub fn plane() -> Shape {
        Shape {
            material: Material::new(),
            transform: Transform::new(),
            id: None,
            kind: ShapeType::Plane,
        }
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }

    pub fn set_pattern(&mut self, pattern: &Pattern) {
        self.material.set_pattern(pattern);
    }

    #[must_use]
    pub fn material(&self) -> &Material {
        &self.material
    }

    #[must_use]
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn set_color(&mut self, c: Color) {
        self.material.color = c;
    }

    pub fn set_ambient(&mut self, c: f64) {
        self.material.ambient = c;
    }

    pub fn set_specular(&mut self, c: f64) {
        self.material.specular = c;
    }

    pub fn set_reflective(&mut self, c: f64) {
        self.material.reflective = c;
    }

    pub fn set_diffuse(&mut self, c: f64) {
        self.material.diffuse = c;
    }

    pub fn set_refractive_index(&mut self, c: f64) {
        self.material.refractive_index = c;
    }

    pub fn set_transparency(&mut self, c: f64) {
        self.material.transparency = c;
    }

    pub fn scale(&mut self, x: f64, y: f64, z: f64) {
        let new_transform = self.transform.clone().scale(x, y, z);
        self.transform = new_transform;
    }

    pub fn translate(&mut self, x: f64, y: f64, z: f64) {
        self.transform = self.transform.clone().translate(x, y, z);
    }

    pub fn rotate_x(&mut self, r: f64) {
        self.transform = self.transform.clone().rotate_x(r);
    }

    pub fn rotate_z(&mut self, r: f64) {
        self.transform = self.transform.clone().rotate_z(r);
    }

    pub fn rotate_y(&mut self, r: f64) {
        self.transform = self.transform.clone().rotate_y(r);
    }

    pub fn shear(&mut self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) {
        self.transform = self.transform.clone().shear(xy, xz, yx, yz, zx, zy);
    }

    #[must_use]
    pub fn normal_at(&self, point: Tuple) -> Tuple {
        match self.kind {
            ShapeType::Sphere => {
                let mut t = self.transform.clone().inverse();
                let object_point = t.transform(&point);
                let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
                t = t.transpose();
                let mut world_normal = t.transform(&object_normal);
                world_normal.w = 0.0;
                world_normal.normalize()
            }
            ShapeType::Plane => Tuple::vector(0.0, 1.0, 0.0),
        }
    }

    #[must_use]
    pub fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        if let Some(id) = self.id {
            let hits = match self.kind {
                ShapeType::Sphere => sphere_intersect(&self.transform, ray, id),
                ShapeType::Plane => plane_intersect(&self.transform, ray, id),
            };
            Some(hits)
        } else {
            None
        }
    }
}

fn sphere_intersect(transform: &Transform, ray: &Ray, object_id: usize) -> Vec<Intersection> {
    let transformed_ray = transform.clone().inverse().transform_ray(ray);
    let sphere_to_ray = transformed_ray.origin - Tuple::point(0.0, 0.0, 0.0);

    let a = transformed_ray
        .direction
        .dot_product(&transformed_ray.direction);
    let b = 2.0 * transformed_ray.direction.dot_product(&sphere_to_ray);
    let c = sphere_to_ray.dot_product(&sphere_to_ray) - 1.0;

    let discriminant = (b * b) - 4.0 * a * c;

    if discriminant < 0.0 {
        return vec![];
    }
    let t1 = Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), object_id);
    let t2 = Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), object_id);
    if t2.t < t1.t {
        vec![t2, t1]
    } else {
        vec![t1, t2]
    }
}

fn plane_intersect(transform: &Transform, ray: &Ray, object_id: usize) -> Vec<Intersection> {
    let ray = transform.clone().inverse().transform_ray(ray);
    if ray.direction.y.abs() < EPSILON {
        return vec![];
    }

    let t = -ray.origin.y / ray.direction.y;
    vec![Intersection::new(t, object_id)]
}
