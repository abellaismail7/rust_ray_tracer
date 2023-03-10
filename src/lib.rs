use utils::{comp::Comp, ray::Ray, vec3::Vec3};
use world::{w::World, light::Light};

pub mod scene;
pub mod utils;
pub mod world;

fn is_shadow(w: &World, ray: &Ray, comp: &Comp) -> bool {
    w.intersect(ray, Vec::new())
        .iter()
        .filter(|i| !std::ptr::eq(i.sp, comp.intersection.sp))
        .any(|i| i.t < 0.0)
}

fn shade_hit(w: &World, c: &Comp, light: &Light) -> Vec3 {
    let mut specular = Vec3::zero();
    let mut diff = Vec3::zero();

    let m = &c.intersection.sp.m;
    let color = &m.color * &light.intensity;
    let ray = light.ray_at(&c.hitp);
    let light_dot = (-&ray.dir).dot(&c.normalv);

    let intersected_with_light = light_dot >= 0.0;
    if intersected_with_light && !is_shadow(w, &ray, c) {
        diff = &color * m.diffuse * light_dot;

        let reflect = ray.dir.reflect(&c.normalv);
        let reflect_dot = reflect.dot(&c.eyev);

        if reflect_dot > 0.0 {
            let factor = reflect_dot.powf(m.shininess);
            specular = &light.intensity * m.specular * factor;
        }
    }
    (&color * m.ambient) + diff + specular
}

fn reflected_color(world: &World, comp: &Comp, depth: usize) -> Vec3 {
    let nearest = comp.intersection;
    if nearest.sp.m.reflective > 0.0 && depth < 10 {
        trace(
            world,
            &Ray::new(comp.hitp.clone(), comp.reflectv.clone()),
            depth + 1,
        ) * nearest.sp.m.reflective
    } else {
        Vec3::zero()
    }
}

pub fn trace(world: &World, ray: &Ray, depth: usize) -> Vec3 {
    let bg = Vec3::zero();
    if depth > 10 {
        return bg;
    }
    let container = Vec::with_capacity(world.spheres.len());
    let intersections = world.intersect(ray, container);
    if let Some(nearest) = intersections.first() {
        let comps = Comp::prepare_comp(ray, nearest);
        let mut surface = reflected_color(world, &comps, depth);
        for light in world.lights.iter() {
            surface = surface + shade_hit(world, &comps, light);
        }
        return surface;
    }
    bg
}

