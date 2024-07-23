use super::vec3::{Float, Vec3};

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Vec3,
    pub ambient: Float,
    pub diffuse: Float,
    pub specular: Float,
    pub reflective: Float,
    pub shininess: Float,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Vec3::from_float(1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            reflective: 0.0,
            shininess: 200.0,
        }
    }
}

pub trait IMaterial: Sized {
    fn color(mut self, x: Float, y: Float, z: Float) -> Self {
        self.get_material().color.set_scalar(x, y, z);
        self
    }

    fn diffuse(mut self, f: Float) -> Self {
        self.get_material().diffuse = f;
        self
    }

    fn specular(mut self, f: Float) -> Self {
        self.get_material().specular = f;
        self
    }

    fn ambient(mut self, f: Float) -> Self {
        self.get_material().ambient = f;
        self
    }

    fn reflective(mut self, f: Float) -> Self {
        self.get_material().reflective = f;
        self
    }

    fn shininess(mut self, f: Float) -> Self {
        self.get_material().shininess = f;
        self
    }

    fn get_material(&mut self) -> &mut Material;
}
