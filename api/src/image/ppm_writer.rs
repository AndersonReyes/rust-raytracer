use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::image::writer::Writer;
use crate::math::color::Color;

pub struct PPMWriter {
    image_name: String,
    height: u32,
    width: u32,
}

impl Writer for PPMWriter {
    fn write(&self, data: &[Color]) -> std::io::Result<()> {
        if data.len() != (self.width * self.height) as usize {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "data must be of size (width * height)",
            ));
        }
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

    use std::fs;

    #[test]
    fn test_ppm_writer_write() {
        let writer = PPMWriter {
            image_name: String::from("test.ppm"),
            width: 1,
            height: 2,
        };

        let res = writer.write(&[Color::new(255.0, 255.0, 0.0), Color::new(0.0, 0.0, 255.0)]);

        assert!(res.is_ok());

        // Read in file contents
        let mut contents = String::new();
        let mut file = OpenOptions::new().read(true).open("test.ppm").unwrap();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "P3\n1 2\n255\n255 255 0\n0 0 255\n");

        fs::remove_file("test.ppm").unwrap();
    }

    #[test]
    fn test_invalid_number_of_data_items() {
        let writer = PPMWriter {
            image_name: String::from("test.ppm"),
            width: 1,
            height: 2,
        };

        match writer.write(&[
            Color::new(255.0, 255.0, 0.0),
            Color::new(0.0, 0.0, 255.0),
            Color::new(0.0, 0.0, 0.0),
        ]) {
            Err(e) => assert_eq!(e.kind(), std::io::ErrorKind::InvalidData),
            Ok(_) => panic!("Expected error!"),
        }
    }
}
