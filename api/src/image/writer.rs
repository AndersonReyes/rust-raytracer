use crate::math::color::Color;

pub trait Writer {
    fn write(&self, data: &[Color]) -> std::io::Result<()>;
}
