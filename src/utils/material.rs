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
