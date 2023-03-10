use std::{fs::File, io::Write, path::Path};

use crate::utils::vec3::{Float, Vec3};

#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub ar: Float,    // TODO: camera
    pub angle: Float, // TODO: camera
    pixels: Vec<u8>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let fov = 120_f32;
        let pixels = vec![0; width * height * 3];
        let ar = width as Float / height as Float;
        let angle = (fov.to_radians() * 0.5).tan();
        Self {
            width,
            height,
            ar,
            angle,
            pixels,
        }
    }

    fn pixel_at(&self, x: usize, y: usize) -> usize {
        3 * y * self.width + x * 3
    }

    pub fn write_at(&mut self, x: usize, y: usize, color: &Vec3) {
        let p = self.pixel_at(x, y);
        color.apply(&mut self.pixels[p..p + 3]);
    }

    pub fn for_each(&mut self, f: impl Fn(&mut [u8], usize, usize)) {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.pixel_at(x, y);
                f(&mut self.pixels[i..(i + 3)], x, y);
            }
        }
    }

    pub fn export_ppm(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        file.write_all(header.as_bytes())?;
        file.write_all(&self.pixels)?;
        Ok(())
    }
}
