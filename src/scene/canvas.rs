use std::{fs::File, io::Write, path::Path, usize};

use crate::utils::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pixels: Vec<u8>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = vec![0; (width * height * 3) as usize];
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn update_size(&mut self, width: u32, height: u32) {
        self.height = height;
        self.width = width;
        self.pixels = vec![0; (width * height * 3) as usize];
    }

    fn pixel_at(&self, x: u32, y: u32) -> usize {
        (3 * y * self.width + x * 3) as usize
    }

    pub fn write_at(&mut self, x: u32, y: u32, color: &Vec3) {
        let p = self.pixel_at(x, y);
        color.apply(&mut self.pixels[p..p + 3]);
    }

    pub fn for_each(&mut self, f: impl Fn(&mut [u8], u32, u32)) {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.pixel_at(x, y);
                f(&mut self.pixels[i..(i + 3)], x, y);
            }
        }
    }

    pub fn export_ppm(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(path)?;
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        file.write_all(header.as_bytes())?;
        file.write_all(&self.pixels)?;
        Ok(())
    }

    pub fn as_rgba8(&self) -> &[u8] {
        &self.pixels
    }
}

