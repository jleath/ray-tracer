use ray_tracer::color::Color;
use ray_tracer::float_near_equal;
use ray_tracer::material::Material;
use ray_tracer::point_light::PointLight;
use ray_tracer::shape::Shape;
use ray_tracer::tuple::Tuple;

#[test]
fn light_init() {
    let intensity = Color::new(1.0, 1.0, 1.0);
    let position = Tuple::point(0.0, 0.0, 0.0);
    let light = PointLight::new(position, intensity);
    assert_eq!(light.position, position);
    assert_eq!(light.intensity, intensity);
}

#[test]
fn material_init() {
    let m = Material::new();
    assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
    assert!(float_near_equal(m.ambient, 0.1));
    assert!(float_near_equal(m.diffuse, 0.9));
    assert!(float_near_equal(m.specular, 0.9));
    assert!(float_near_equal(m.shininess, 200.0))
}

#[test]
fn lighting() {
    let m = Material::new();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let object = Shape::sphere();

    let mut eye = Tuple::vector(0.0, 0.0, -1.0);
    let normal = Tuple::vector(0.0, 0.0, -1.0);
    let mut light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let mut result = m.lighting(&light, &object, position, eye, normal, false);
    assert_eq!(result, Color::new(1.9, 1.9, 1.9));

    eye = Tuple::vector(0.0, 2_f64.sqrt() / 2.0, -(2_f64.sqrt()) / 2.0);
    result = m.lighting(&light, &object, position, eye, normal, false);
    assert_eq!(result, Color::new(1.0, 1.0, 1.0));

    eye = Tuple::vector(0.0, 0.0, -1.0);
    light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    result = m.lighting(&light, &object, position, eye, normal, false);
    assert_eq!(
        result,
        Color::new(0.7363961030678927, 0.7363961030678927, 0.7363961030678927)
    );

    eye = Tuple::vector(0.0, -(2_f64.sqrt()) / 2.0, -(2_f64.sqrt()) / 2.0);
    result = m.lighting(&light, &object, position, eye, normal, false);
    assert_eq!(
        result,
        Color::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928)
    );

    eye = Tuple::vector(0.0, 0.0, -1.0);
    light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
    result = m.lighting(&light, &object, position, eye, normal, false);
    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
}

#[test]
fn lighting_in_shadow() {
    let m = Material::new();
    let object = Shape::sphere();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let result = m.lighting(&light, &object, position, eyev, normalv, true);
    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
}
