use minirt::{
    scene::canvas::Canvas,
    utils::{material::Material, matrix::Mat, ray::Ray, vec3::Vec3},
    world::{
        camera::Camera,
        light::Light,
        sphere::Sphere,
        w::{Comp, World},
    },
};

fn shade_hit(c: &Comp, l: &Light) -> Vec3 {
    let m = &c.intersection.sp.m;
    let color = &m.color * &l.intensity;

    let ambient = &color * m.ambient;
    let mut specular = Vec3::from_float(0.0);
    let mut diff = Vec3::from_float(0.0);

    let light_dir = (&l.position - &c.hitp).norm();
    let light_dot = light_dir.dot(&c.normalv);

    if light_dot >= 0.0 {
        diff = &color * m.diffuse * light_dot;

        let reflect = (-&light_dir).reflect(&c.normalv);
        let reflect_dot = reflect.dot(&c.eyev);

        if reflect_dot > 0.0 {
            let factor = reflect_dot.powf(m.shininess);
            specular = &l.intensity * m.specular * factor;
        }
    }
    ambient + diff + specular
}

fn trace(w: &World, ray: &Ray) -> Vec3 {
    let bg = Vec3::new(0.0, 0.0, 0.0);
    let intersections = Vec::with_capacity(w.spheres.len());
    let intersections = w.intersect(ray, intersections);
    if let Some(i) = intersections.first() {
        let hitp = ray.position(i.t0);
        let norm = i.sp.normal_at(&hitp);
        let c = Comp {
            intersection: i,
            hitp,
            normalv: norm,
            eyev: -&ray.dir,
            inside: false,
        };
        let mut color = Vec3::from_float(0.0);
        for l in w.lights.iter() {
            color = color + shade_hit(&c, l);
        }
        return color;
    }
    bg
}

fn main() {
    let mut canvas = Canvas::new(1000, 1000);
    let spheres = vec![
        Sphere::new(
            Material {
                color: Vec3::new(0.0, 1.0, 1.0),
                ..Material::default()
            },
            Mat::identity(4).translation(0.0, 0.0, 0.0),
        ),
        Sphere::new(
            Material {
                color: Vec3::new(1.0, 0.2, 1.0),
                ..Material::default()
            },
            Mat::identity(4).translation(-2.0, 1.0, -1.0),
        ),
    ];
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, -15.0),
        Vec3::new(0.0, 0.0, 1.0).norm(),
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        canvas.width,
        canvas.height,
    );
    let lights = vec![
        Light::new(Vec3::new(-10.0, 10.0, -10.0), Vec3::from_float(1.0)),
        Light::new(Vec3::new(-0.0, -3.0, -0.0), Vec3::from_float(1.0)),
    ];
    let w = World::new(camera, lights, spheres);
    canvas.for_each(|pixel, x, y| {
        let dir = w.camera.get_ray(x, y);
        let ray = Ray::new(w.camera.org.clone(), dir);
        let color = trace(&w, &ray);
        print!("\r{} pixel", 1000 * y + x);
        color.apply(pixel)
    });
    canvas.export_ppm("file.ppm").ok();
}
