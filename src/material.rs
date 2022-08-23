use crate::color::Color;
use crate::pattern::Pattern;
use crate::point_light::PointLight;
use crate::shape::Shape;
use crate::tuple::Tuple;

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub reflective: f64,
    pub shininess: f64,
    pub pattern: Option<Pattern>,
}

impl Default for Material {
    fn default() -> Self {
        Material::new()
    }
}

impl Material {
    #[must_use]
    pub fn new() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            reflective: 0.0,
            shininess: 200.0,
            pattern: None,
        }
    }

    pub fn set_pattern(&mut self, p: &Pattern) {
        self.pattern = Some(p.clone());
    }

    #[must_use]
    pub fn lighting(
        &self,
        light: &PointLight,
        object: &Shape,
        position: Tuple,
        eye: Tuple,
        normal: Tuple,
        in_shadow: bool,
    ) -> Color {
        let mut effective_color = self.color * light.intensity;
        if let Some(pattern) = &self.pattern {
            effective_color = pattern.color_at_object(object, position) * light.intensity;
        }
        let lightv = (light.position - position).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot_product(&normal);
        let mut diffuse = Color::new(0.0, 0.0, 0.0);
        let mut specular = Color::new(0.0, 0.0, 0.0);
        if light_dot_normal >= 0.0 && !in_shadow {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflect = (-lightv).reflect(&normal);
            let reflect_dot_eye = reflect.dot_product(&eye);
            if reflect_dot_eye > 0.0 {
                let factor = f64::powf(reflect_dot_eye, self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}
