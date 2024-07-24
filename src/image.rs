use crate::color::{write_color, Color};
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
