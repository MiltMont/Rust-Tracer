use crate::{image::Pixel, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color(pixel_color: &Color) -> Pixel {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let factor = 255.999;

    let r = (factor * r) as u8;
    let g = (factor * g) as u8;
    let b = (factor * b) as u8;

    Pixel { r, g, b }
}
