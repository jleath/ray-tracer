use ray_tracer::color::Color;
use ray_tracer::float_near_equal;
use ray_tracer::intersection::{Intersection, IntersectionList};
use ray_tracer::pattern::Pattern;
use ray_tracer::point_light::PointLight;
use ray_tracer::ray::Ray;
use ray_tracer::shape::Shape;
use ray_tracer::tuple::Tuple;
use ray_tracer::world::World;
use ray_tracer::MAX_REFLECT_DEPTH;

#[test]
fn init() {
    let w = World::new();
    assert_eq!(w.get_object(0), None);
    assert_eq!(w.get_light(0), None);
}

#[test]
fn default_world() {
    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let mut s1 = Shape::sphere();
    s1.set_color(Color::new(0.8, 1.0, 0.6));
    s1.set_id(0);
    s1.set_diffuse(0.7);
    s1.set_specular(0.2);
    let mut s2 = Shape::sphere();
    s2.set_id(1);
    s2.scale(0.5, 0.5, 0.5);

    let world = World::default();
    assert_eq!(world.get_light(0).unwrap(), &light);
    assert_eq!(world.get_object(0).unwrap(), &s1);
    assert_eq!(world.get_object(1).unwrap(), &s2);
}

#[test]
fn world_intersect() {
    let w = World::default();
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let xs = w.intersect(&r);
    assert_eq!(xs.len(), 4);
    assert!(float_near_equal(xs.get(0).unwrap().t, 4.0));
    assert!(float_near_equal(xs.get(1).unwrap().t, 4.5));
    assert!(float_near_equal(xs.get(2).unwrap().t, 5.5));
    assert!(float_near_equal(xs.get(3).unwrap().t, 6.0));
}

#[test]
fn hit_shading() {
    let mut w = World::default();
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let i = Intersection::new(4.0, 0);
    let comps = i.prepare_computation(&r, &w);
    let c = w.shade_hit(&comps, MAX_REFLECT_DEPTH);
    assert_eq!(
        c,
        Color::new(
            0.38066119308103435,
            0.47582649135129296,
            0.28549589481077575,
        )
    );

    w.set_light(
        0,
        &mut PointLight::new(Tuple::point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0)),
    )
    .unwrap();

    let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let i = Intersection::new(0.5, 1);
    let comps = i.prepare_computation(&r, &w);

    let c = w.shade_hit(&comps, MAX_REFLECT_DEPTH);
    assert_eq!(
        c,
        Color::new(0.9049844720832575, 0.9049844720832575, 0.9049844720832575)
    );
}

#[test]
fn shade_hit_in_shadow() {
    let mut w = World::new();
    let l = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
    w.add_light(l);
    let s1 = Shape::sphere();
    let mut s2 = Shape::sphere();
    s2.translate(0.0, 0.0, 10.0);
    w.add_object(s1);
    let s2_id = w.add_object(s2);
    let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
    let i = Intersection::new(4.0, s2_id);
    let comps = i.prepare_computation(&r, &w);
    let c = w.shade_hit(&comps, MAX_REFLECT_DEPTH);
    assert_eq!(c, Color::new(0.1, 0.1, 0.1));
}

#[test]
fn reflect_on_nonreflective() {
    let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let mut s1 = Shape::sphere();
    s1.set_color(Color::new(0.8, 1.0, 0.6));
    s1.set_diffuse(0.7);
    s1.set_specular(0.2);
    let mut s2 = Shape::sphere();
    s2.scale(0.5, 0.5, 0.5);
    s2.set_ambient(1.0);
    let mut new_world = World::new();
    new_world.add_object(s1);
    new_world.add_object(s2);
    new_world.add_light(light);

    let i = Intersection::new(1.0, 1);
    let comps = i.prepare_computation(&r, &new_world);
    let c = new_world.reflected_color(&comps, MAX_REFLECT_DEPTH);
    assert_eq!(c, Color::new(0.0, 0.0, 0.0));
}

#[test]
fn reflect_on_reflective() {
    let mut w = World::default_world();
    let mut shape = Shape::plane();
    shape.set_reflective(0.5);
    shape.translate(0.0, -1.0, 0.0);
    w.add_object(shape);
    let r = Ray::new(
        Tuple::point(0.0, 0.0, -3.0),
        Tuple::vector(0.0, -(2_f64.sqrt()) / 2.0, 2_f64.sqrt() / 2.0),
    );
    let i = Intersection::new(2_f64.sqrt(), 2);
    let comps = i.prepare_computation(&r, &w);
    let color = w.reflected_color(&comps, MAX_REFLECT_DEPTH);
    println!("{:#?}", comps);
    assert_eq!(color, Color::new(0.19033, 0.23791, 0.14274));
}

#[test]
fn shade_hit_with_reflective() {
    let mut w = World::default_world();
    let mut shape = Shape::plane();
    shape.set_reflective(0.5);
    shape.translate(0.0, -1.0, 0.0);
    w.add_object(shape);
    let r = Ray::new(
        Tuple::point(0.0, 0.0, -3.0),
        Tuple::vector(0.0, -(2_f64.sqrt()) / 2.0, 2_f64.sqrt() / 2.0),
    );
    let i = Intersection::new(2_f64.sqrt(), 2);
    let comps = i.prepare_computation(&r, &w);
    let color = w.shade_hit(&comps, MAX_REFLECT_DEPTH);
    assert_eq!(color, Color::new(0.87675, 0.92434, 0.82917));
}

#[test]
#[allow(unused_must_use)]
fn color_at_avoid_infinite_recursion() {
    let mut w = World::new();
    let light = PointLight::new(Tuple::point(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0));
    let mut lower = Shape::plane();
    lower.set_reflective(1.0);
    lower.translate(0.0, -1.0, 0.0);
    let mut upper = Shape::plane();
    upper.set_reflective(1.0);
    upper.translate(0.0, 1.0, 0.0);
    w.add_object(lower);
    w.add_object(upper);
    w.add_light(light);
    w.color_at(
        &Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)),
        0,
    );
}

#[test]
fn refracted_color_of_opaque_object() {
    let w = World::default_world();
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let xs = IntersectionList::new(&vec![Intersection::new(4.0, 0), Intersection::new(6.0, 0)]);
    let comps = xs.prepare_computation(0, &r, &w);
    let c = w.refracted_color(&comps, 5);
    assert_eq!(c, Color::new(0.0, 0.0, 0.0));
}

#[test]
fn refracted_color_at_maximum_depth() {
    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let mut s1 = Shape::sphere();
    s1.set_color(Color::new(0.8, 1.0, 0.6));
    s1.set_diffuse(0.7);
    s1.set_specular(0.2);
    s1.set_transparency(1.0);
    s1.set_refractive_index(1.5);
    let mut s2 = Shape::sphere();
    s2.scale(0.5, 0.5, 0.5);
    let mut new_world = World::new();
    new_world.add_object(s1);
    new_world.add_object(s2);
    new_world.add_light(light);

    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let xs = IntersectionList::new(&vec![Intersection::new(4.0, 0), Intersection::new(6.0, 0)]);
    let comps = xs.prepare_computation(0, &r, &new_world);
    let c = new_world.refracted_color(&comps, 0);
    assert_eq!(c, Color::new(0.0, 0.0, 0.0));
}

#[test]
fn test_internal_reflection() {
    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let mut s1 = Shape::sphere();
    s1.set_color(Color::new(0.8, 1.0, 0.6));
    s1.set_diffuse(0.7);
    s1.set_specular(0.2);
    s1.set_transparency(1.0);
    s1.set_refractive_index(1.5);
    let mut s2 = Shape::sphere();
    s2.scale(0.5, 0.5, 0.5);
    let mut new_world = World::new();
    new_world.add_object(s1);
    new_world.add_object(s2);
    new_world.add_light(light);

    let r = Ray::new(
        Tuple::point(0.0, 0.0, 2_f64.sqrt() / 2.0),
        Tuple::vector(0.0, 1.0, 0.0),
    );
    let xs = IntersectionList::new(&vec![
        Intersection::new(-(2_f64.sqrt()) / 2.0, 0),
        Intersection::new(2_f64.sqrt() / 2.0, 0),
    ]);

    let comps = xs.prepare_computation(1, &r, &new_world);
    let c = new_world.refracted_color(&comps, MAX_REFLECT_DEPTH);
    assert_eq!(c, Color::new(0.0, 0.0, 0.0));
}

#[test]
fn refracted_color() {
    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let mut s1 = Shape::sphere();
    s1.set_color(Color::new(0.8, 1.0, 0.6));
    s1.set_diffuse(0.7);
    s1.set_specular(0.2);
    s1.set_ambient(1.0);
    s1.set_pattern(&Pattern::test_pattern());
    let mut s2 = Shape::sphere();
    s2.scale(0.5, 0.5, 0.5);
    s2.set_transparency(1.0);
    s2.set_refractive_index(1.5);
    let mut new_world = World::new();
    new_world.add_object(s1);
    new_world.add_object(s2);
    new_world.add_light(light);

    let r = Ray::new(Tuple::point(0.0, 0.0, 0.1), Tuple::vector(0.0, 1.0, 0.0));
    let xs = IntersectionList::new(&vec![
        Intersection::new(-0.9899, 0),
        Intersection::new(-0.4899, 1),
        Intersection::new(0.4899, 1),
        Intersection::new(0.9899, 0),
    ]);
    let comps = xs.prepare_computation(2, &r, &new_world);
    let c = new_world.refracted_color(&comps, MAX_REFLECT_DEPTH);
    assert_eq!(c, Color::new(0.0, 0.99888, 0.047219));
}

#[test]
fn schlick_test() {
    let s = Shape::glass_sphere();
    let mut world = World::new();
    world.add_object(s);
    let mut r = Ray::new(
        Tuple::point(0.0, 0.0, 2_f64.sqrt() / 2.0),
        Tuple::vector(0.0, 1.0, 0.0),
    );
    let mut xs = IntersectionList::new(&vec![
        Intersection::new(-(2_f64).sqrt() / 2.0, 0),
        Intersection::new(2_f64.sqrt() / 2.0, 0),
    ]);
    let mut comps = xs.prepare_computation(1, &r, &world);
    let mut reflectance = World::schlick(&comps);
    assert!(float_near_equal(reflectance, 1.0));

    r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
    xs = IntersectionList::new(&vec![Intersection::new(-1.0, 0), Intersection::new(1.0, 0)]);
    comps = xs.prepare_computation(1, &r, &world);
    reflectance = World::schlick(&comps);
    assert!(float_near_equal(reflectance, 0.04));

    r = Ray::new(Tuple::point(0.0, 0.99, -2.0), Tuple::vector(0.0, 0.0, 1.0));
    xs = IntersectionList::new(&vec![Intersection::new(1.8589, 0)]);
    comps = xs.prepare_computation(0, &r, &world);
    reflectance = World::schlick(&comps);
    assert!(float_near_equal(reflectance, 0.48873));
}

#[test]
fn render_with_schlick() {
    let mut w = World::default_world();
    let r = Ray::new(
        Tuple::point(0.0, 0.0, -3.0),
        Tuple::vector(0.0, -(2_f64).sqrt() / 2.0, 2_f64.sqrt() / 2.0),
    );
    let mut floor = Shape::plane();
    floor.translate(0.0, -1.0, 0.0);
    floor.set_reflective(0.5);
    floor.set_transparency(0.5);
    floor.set_refractive_index(1.5);
    w.add_object(floor);

    let mut ball = Shape::sphere();
    ball.set_color(Color::new(1.0, 0.0, 0.0));
    ball.set_ambient(0.5);
    ball.translate(0.0, -3.5, -0.5);
    w.add_object(ball);

    let xs = IntersectionList::new(&vec![Intersection::new(2_f64.sqrt(), 2)]);
    let comps = xs.prepare_computation(0, &r, &w);
    let color = w.shade_hit(&comps, 5);
    assert_eq!(color, Color::new(0.93391, 0.69643, 0.69243));
}

#[test]
fn shade_hit_with_transparent_material() {
    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let mut s1 = Shape::sphere();
    s1.set_color(Color::new(0.8, 1.0, 0.6));
    s1.set_diffuse(0.7);
    s1.set_specular(0.2);
    s1.set_ambient(1.0);
    s1.set_pattern(&Pattern::test_pattern());
    let mut s2 = Shape::sphere();
    s2.scale(0.5, 0.5, 0.5);
    s2.set_transparency(1.0);
    s2.set_refractive_index(1.5);
    let mut new_world = World::new();
    new_world.add_object(s1);
    new_world.add_object(s2);
    new_world.add_light(light);

    let mut floor = Shape::plane();
    floor.translate(0.0, -1.0, 0.0);
    floor.set_transparency(0.5);
    floor.set_refractive_index(1.5);
    new_world.add_object(floor);

    let mut ball = Shape::sphere();
    ball.set_color(Color::new(1.0, 0.0, 0.0));
    ball.set_ambient(0.5);
    ball.translate(0.0, -3.5, -0.5);
    new_world.add_object(ball);

    let r = Ray::new(
        Tuple::point(0.0, 0.0, -3.0),
        Tuple::vector(0.0, -(2_f64.sqrt()) / 2.0, 2_f64.sqrt() / 2.0),
    );
    let xs = IntersectionList::new(&vec![Intersection::new(2_f64.sqrt(), 2)]);
    let comps = xs.prepare_computation(0, &r, &new_world);
    let color = new_world.shade_hit(&comps, MAX_REFLECT_DEPTH);
    assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
}

#[test]
fn ray_miss() {
    let w = World::default();
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
    let c = w.color_at(&r, MAX_REFLECT_DEPTH);
    assert_eq!(c, Color::new(0.0, 0.0, 0.0));
}

#[test]
fn ray_hit() {
    let w = World::default();
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let c = w.color_at(&r, MAX_REFLECT_DEPTH);
    assert_eq!(
        c,
        Color::new(
            0.38066119308103435,
            0.47582649135129296,
            0.28549589481077575,
        )
    );
}

#[test]
fn intersection_behind_ray() {
    let mut w = World::new();
    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let mut s1 = Shape::sphere();
    s1.set_color(Color::new(0.8, 1.0, 0.6));
    s1.set_diffuse(0.7);
    s1.set_specular(0.2);
    s1.set_ambient(1.0);
    let mut s2 = Shape::sphere();
    s2.scale(0.5, 0.5, 0.5);
    s2.set_ambient(1.0);
    w.add_object(s1);
    w.add_object(s2.clone());
    w.add_light(light);

    let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
    let c = w.color_at(&r, MAX_REFLECT_DEPTH);
    assert_eq!(c, s2.material().color);
}

#[test]
fn is_shadowed() {
    let w = World::default();
    let mut p = Tuple::point(0.0, 10.0, 0.0);
    assert!(!w.is_shadowed(p, w.get_light(0).unwrap()));

    p = Tuple::point(10.0, -10.0, 10.0);
    assert!(w.is_shadowed(p, w.get_light(0).unwrap()));

    p = Tuple::point(-20.0, 20.0, -20.0);
    assert!(!w.is_shadowed(p, w.get_light(0).unwrap()));

    p = Tuple::point(-2.0, 2.0, -2.0);
    assert!(!w.is_shadowed(p, w.get_light(0).unwrap()));
}
