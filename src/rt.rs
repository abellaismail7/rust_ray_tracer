use crate::{
    utils::{comp::Comp, ray::Ray, vec3::Vec3},
    world::{light::Light, w::World},
};

pub struct RayTracer {
    pub world: World,
}

impl RayTracer {
    pub fn new(world: World) -> Self {
        Self { world }
    }

    fn is_shadow(&self, ray: &Ray, comp: &Comp) -> bool {
        self.world
            .intersect(ray, Vec::new())
            .iter()
            .filter(|i| !std::ptr::eq(i.sp, comp.intersection.sp))
            .any(|i| i.t < 0.0)
    }

    pub fn update_size(&mut self, width: u32, height: u32) {
        self.world.camera.update_size(width, height);
    }

    pub fn trace(&self, ray: &Ray, depth: usize) -> Vec3 {
        let bg = Vec3::zero();
        if depth > 10 {
            return bg;
        }
        let container = Vec::with_capacity(self.world.spheres.len());
        let intersections = self.world.intersect(ray, container);
        if let Some(nearest) = intersections.first() {
            let comps = Comp::prepare_comp(ray, nearest);
            let mut surface = self.reflected_color(&comps, depth);
            for light in self.world.lights.iter() {
                surface = surface + self.shade_hit(&comps, light);
            }
            return surface;
        }
        bg
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn mut_world(&mut self) -> &mut World {
        &mut self.world
    }

    fn shade_hit(&self, c: &Comp, light: &Light) -> Vec3 {
        let mut specular = Vec3::zero();
        let mut diff = Vec3::zero();

        let m = &c.intersection.sp.m;
        let color = &m.color * &light.intensity;
        let ray = light.ray_at(&c.hitp);
        let light_dot = (-&ray.dir).dot(&c.normalv);

        let intersected_with_light = light_dot >= 0.0;
        if intersected_with_light && !self.is_shadow(&ray, c) {
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

    fn reflected_color(&self, comp: &Comp, depth: usize) -> Vec3 {
        let nearest = comp.intersection;
        if nearest.sp.m.reflective > 0.0 && depth < 10 {
            self.trace(
                &Ray::new(comp.hitp.clone(), comp.reflectv.clone()),
                depth + 1,
            ) * nearest.sp.m.reflective
        } else {
            Vec3::zero()
        }
    }
}
