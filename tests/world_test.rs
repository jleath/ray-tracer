use ray_tracer::color::Color;
use ray_tracer::float_near_equal;
use ray_tracer::intersection::Intersection;
use ray_tracer::point_light::PointLight;
use ray_tracer::ray::Ray;
use ray_tracer::shape::Shape;
use ray_tracer::tuple::Tuple;
use ray_tracer::world::World;

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
    let c = w.shade_hit(&comps);
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

    let c = w.shade_hit(&comps);
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
    let c = w.shade_hit(&comps);
    assert_eq!(c, Color::new(0.1, 0.1, 0.1));
}

#[test]
fn ray_miss() {
    let w = World::default();
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
    let c = w.color_at(&r);
    assert_eq!(c, Color::new(0.0, 0.0, 0.0));
}

#[test]
fn ray_hit() {
    let w = World::default();
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let c = w.color_at(&r);
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
    let c = w.color_at(&r);
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
