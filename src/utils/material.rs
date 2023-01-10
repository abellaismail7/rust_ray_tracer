use super::vec3::{Float, Vec3};

#[derive(Debug, Clone)]
pub enum Pattern {
    Stripped(Vec3, Vec3),
    Unified(Vec3),
}

#[derive(Debug, Clone)]
pub struct Material {
    pub pattern: Pattern,
    pub ambient: Float,
    pub diffuse: Float,
    pub specular: Float,
    pub reflective: Float,
    pub shininess: Float,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            pattern: Pattern::Unified(Vec3::from_float(1.0)),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            reflective: 0.0,
            shininess: 200.0,
        }
    }
}

impl Material {
    pub fn get_color(&self, point: &Vec3) -> &Vec3 {
        match &self.pattern {
            Pattern::Stripped(odd_color, even_color) => {
                if point.x.round() % 2.0 == 0.0 {
                    return even_color;
                }
                odd_color
            }
            Pattern::Unified(color) => color,
        }
    }
}

pub trait IMaterial: Sized {
    fn color(mut self, x: Float, y: Float, z: Float) -> Self {
        self.get_material().pattern = Pattern::Unified(Vec3::new(x, y, z));
        self
    }

    fn pattern(mut self, pattern: Pattern) -> Self {
        self.get_material().pattern = pattern;
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
