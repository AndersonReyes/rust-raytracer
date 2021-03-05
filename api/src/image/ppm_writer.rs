use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::math::color::Color;
use crate::image::writer::Writer;

pub struct PPMWriter {
    image_name: String,
    height: u32,
    width: u32
}

impl Writer for PPMWriter {
    fn write(&self, data: &[Color]) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.image_name)?;

        // Write header
        writeln!(file, "P3\n{} {}\n255", self.width, self.height)?;
        for pix in data.iter() {
            writeln!(file, "{} {} {}", pix.r, pix.g, pix.b)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ppm_writter_write() {
        let writer = PPMWriter {
            image_name: String::from("test.ppm"),
            width: 1,
            height: 2,
        };

        let res = writer.write(&[Color::new(255.0, 255.0, 0.0), Color::new(0.0, 0.0, 255.0)]);

        assert!(res.is_ok());
    }
}
