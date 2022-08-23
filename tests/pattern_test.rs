use ray_tracer::material::Material;
use ray_tracer::pattern::Pattern;
use ray_tracer::point_light::PointLight;
use ray_tracer::shape::Shape;
use ray_tracer::tuple::Tuple;
use ray_tracer::{BLACK, WHITE};
#[test]
fn constant_in_y() {
    let pattern = Pattern::stripes(WHITE, BLACK);
    assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.color_at(Tuple::point(0.0, 1.0, 0.0)), WHITE);
    assert_eq!(pattern.color_at(Tuple::point(0.0, 2.0, 0.0)), WHITE);
}

#[test]
fn constant_in_z() {
    let pattern = Pattern::stripes(WHITE, BLACK);
    assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 1.0)), WHITE);
    assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 2.0)), WHITE);
}

#[test]
fn alternates_in_x() {
    let pattern = Pattern::stripes(WHITE, BLACK);
    assert_eq!(pattern.color_at(Tuple::point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.color_at(Tuple::point(0.9, 0.0, 0.0)), WHITE);
    assert_eq!(pattern.color_at(Tuple::point(1.0, 0.0, 0.0)), BLACK);
    assert_eq!(pattern.color_at(Tuple::point(-0.1, 0.0, 0.0)), BLACK);
    assert_eq!(pattern.color_at(Tuple::point(-1.0, 0.0, 0.0)), BLACK);
    assert_eq!(pattern.color_at(Tuple::point(-1.1, 0.0, 0.0)), WHITE);
}

#[test]
fn lighting_with_pattern() {
    let mut m = Material::new();
    let pattern = Pattern::stripes(WHITE, BLACK);
    let object = Shape::sphere();
    m.ambient = 1.0;
    m.diffuse = 0.0;
    m.specular = 0.0;
    m.set_pattern(&pattern);
    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), WHITE);
    let c1 = m.lighting(
        &light,
        &object,
        Tuple::point(0.9, 0.0, 0.0),
        eyev,
        normalv,
        false,
    );
    let c2 = m.lighting(
        &light,
        &object,
        Tuple::point(1.1, 0.0, 0.0),
        eyev,
        normalv,
        false,
    );
    assert_eq!(c1, WHITE);
    assert_eq!(c2, BLACK);
}

#[test]
fn stripes_with_object_transform() {
    let mut object = Shape::sphere();
    object.scale(2.0, 2.0, 2.0);
    let pattern = Pattern::stripes(WHITE, BLACK);
    let c = pattern.color_at_object(&object, Tuple::point(1.5, 0.0, 0.0));
    assert_eq!(c, WHITE);
}

#[test]
fn stripes_with_pattern_transform() {
    let object = Shape::sphere();
    let mut pattern = Pattern::stripes(WHITE, BLACK);
    pattern.scale(2.0, 2.0, 2.0);
    let c = pattern.color_at_object(&object, Tuple::point(1.5, 0.0, 0.0));
    assert_eq!(c, WHITE);
}

#[test]
fn stripes_with_both_transforms() {
    let mut object = Shape::sphere();
    object.scale(2.0, 2.0, 2.0);
    let mut pattern = Pattern::stripes(WHITE, BLACK);
    pattern.scale(2.0, 2.0, 2.0);
    let c = pattern.color_at_object(&object, Tuple::point(1.5, 0.0, 0.0));
    assert_eq!(c, WHITE);
}
