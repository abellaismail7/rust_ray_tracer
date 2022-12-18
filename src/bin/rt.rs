use minirt::{
    scene::canvas::Canvas,
    utils::{
        material::Material,
        ray::Ray,
        vec3::{Float, Vec3},
    },
    world::{camera::Camera, light::Light, sphere::Sphere, w::World},
};

fn lighting(m: &Material, w: &World, eye_vn: &Vec3, normal_v: &Vec3, hitp: &Vec3) -> Vec3 {
    let l = &w.lights[0];
    let color = &m.color * &l.intensity;

    let ambient = &color * m.ambient;
    let mut specular = Vec3::from_float(0.0);
    let mut diff = Vec3::from_float(0.0);

    let light_dir = (&l.position - hitp).norm();
    let light_dot = light_dir.dot(normal_v);

    let mut t = Float::INFINITY;
    let opl_dir = -&light_dir;
    let nray = Ray::new(hitp, &opl_dir);
    for s in w.spheres.iter() {
        // this will compare raw pointers
        if std::ptr::eq(&s.m, m) {
            continue;
        }
        if let Some((t0, _t1)) = s.intersect(&nray) {
            if t0 < t {
                t = t0;
            }
        };
    }

    if light_dot >= 0.0 && t >= light_dir.mag() {
        diff = &color * m.diffuse * light_dot;

        let reflect = (-&light_dir).reflect(normal_v);
        let reflect_dot = reflect.dot(eye_vn);

        if reflect_dot > 0.0 {
            let factor = reflect_dot.powf(m.shininess);
            specular = &l.intensity * m.specular * factor;
        }
    }
    ambient + diff + specular
}

fn trace(w: &World, ray: &Ray) -> Vec3 {
    let bg = Vec3::new(0.0, 0.0, 0.0);
    let mut sphere: Option<&Sphere> = None;
    let mut t = Float::INFINITY;
    for s in w.spheres.iter() {
        if let Some((t0, _t1)) = s.intersect(ray) {
            if t0 < t {
                sphere = Some(s);
                t = t0;
            }
        };
    }
    match sphere {
        Some(s) => {
            let hitp = ray.position(t);
            let norm = s.normal_at(&hitp);
            lighting(&s.m, w, &(-ray.dir).norm(), &norm, &hitp)
        }
        None => bg,
    }
}

fn main() {
    let mut canvas = Canvas::new(1000, 1000);
    let spheres = vec![
        Sphere::new(
            Vec3::new(-1.0, -1.0, -1.0),
            Material {
                color: Vec3::new(0.0, 1.0, 1.0),
                ..Material::default()
            },
            0.5,
        ),
        Sphere::new(
            Vec3::new(-0.0, 0.0, -0.0),
            Material {
                color: Vec3::new(1.0, 0.2, 1.0),
                ..Material::default()
            },
            1.0,
        ),
    ];
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, -10.0),
        Vec3::new(0.0, 0.0, 1.0).norm(),
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        canvas.width,
        canvas.height,
    );
    let lights = vec![Light::new(
        Vec3::new(-10.0, -10.0, -10.0),
        Vec3::from_float(1.0),
    )];
    let w = World::new(camera, lights, spheres);
    canvas.for_each(|pixel, x, y| {
        let dir = w.camera.get_ray(x, y);
        let ray = Ray::new(&w.camera.org, &dir);
        let color = trace(&w, &ray);
        color.apply(pixel)
    });
    canvas.export_ppm("file.ppm").ok();
}
