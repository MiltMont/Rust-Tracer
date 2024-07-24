use crate::{
    color::{write_color, Color},
    ray::{ray_color, Ray},
    vec3::{Point, Vec3},
};
use std::fmt::{Debug, Display};

#[derive(Default)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// An image is a matrix containing pixels.
pub struct Image {
    pixels: Vec<Vec<Pixel>>,
}

impl Image {
    pub fn new(height: usize, width: usize) -> Self {
        assert!(height > 0 && width > 0);

        // Creates a vector of lenght `height`
        let mut pixels = Vec::with_capacity(height);

        for _ in 0..height {
            // Creates a vector of lenght `width`
            let mut row = Vec::with_capacity(width);

            for _ in 0..width {
                row.push(Pixel::default());
            }
            pixels.push(row);
        }

        Self { pixels }
    }

    // Creates an image using a function (usize, usize) -> Pixel
    pub fn new_with_init(
        height: usize,
        width: usize,
        init: impl Fn(usize, usize) -> Pixel,
    ) -> Self {
        let mut image = Self::new(height, width);

        for row in 0..height {
            for col in 0..width {
                image.pixels[row][col] = init(row, col);
            }
        }

        image
    }

    pub fn height(&self) -> usize {
        self.pixels.len()
    }

    pub fn width(&self) -> usize {
        self.pixels[0].len()
    }
}

pub fn sample_image() -> Image {
    let image_width = 256;
    let image_height = 256;

    Image::new_with_init(image_height, image_width, |row, col| {
        let pixel_color = Color {
            x: col as f64 / ((image_width - 1) as f64),
            y: row as f64 / ((image_height - 1) as f64),
            z: 0.,
        };
        let pixel = write_color(&pixel_color);
        pixel
    })
}

pub fn test_image() -> Image {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    if image_height < 1 {
        let image_height = 1;
    }

    //Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let veiwport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and downs the vertical
    // viewport edges.
    let viewport_u = Vec3::new(veiwport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = &viewport_u / image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        &camera_center - &Vec3::new(0.0, 0.0, focal_length) - &viewport_u / 2.0 - &viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (&pixel_delta_u + &pixel_delta_v);

    // Render
    Image::new_with_init(image_height, image_width, |row, col| {
        let pixel_center =
            &pixel00_loc + &(col as f64 * &pixel_delta_u) + (row as f64 * &pixel_delta_v);
        let ray_direction = &pixel_center - &camera_center;
        // TODO: Use camera_center instead of using a new point on each iteration.
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), ray_direction);

        let pixel_color = ray_color(&ray);

        let pixel = write_color(&pixel_color);
        pixel
    })
}

// PPM Image format.
pub struct PPM<'a, T>(pub &'a T);

impl Display for PPM<'_, Pixel> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>3} {:>3} {:>3}", self.0.r, self.0.g, self.0.b)
    }
}

impl Debug for PPM<'_, Pixel> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for PPM<'_, Image> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.0.width(), self.0.height())?;
        writeln!(f, "255")?;

        for row in 0..self.0.height() {
            for col in 0..self.0.width() {
                writeln!(f, "{}", PPM(&self.0.pixels[row][col]))?;
            }
        }

        Ok(())
    }
}

impl Debug for PPM<'_, Image> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pixel() {
        let pixel = Pixel { r: 1, g: 2, b: 3 };
        k9::snapshot!(PPM(&pixel), "  1   2   3");
    }

    #[test]
    fn test_image() {
        let img = Image::new_with_init(2, 3, |row, col| Pixel {
            r: row as u8,
            g: col as u8,
            b: 12,
        });

        k9::snapshot!(
            PPM(&img),
            "
P3
3 2
255
  0   0  12
  0   1  12
  0   2  12
  1   0  12
  1   1  12
  1   2  12

"
        );
    }
}
