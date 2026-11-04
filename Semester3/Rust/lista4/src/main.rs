use std::fs::File;
use std::io::{self, Write};
use std::ops::{Add, Div, Mul, Sub};

pub type Color = [u8; 3];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![[0, 0, 0]; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        self.pixels[y * self.width + x] = color;
        true
    }

    pub fn pixel(&self, x: usize, y: usize) -> Option<Color> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.pixels[y * self.width + x])
    }

    pub fn save_ppm(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(file, "P3")?;
        writeln!(file, "{} {}", self.width, self.height)?;
        writeln!(file, "255")?;

        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.pixel(x, y).unwrap();
                writeln!(file, "{} {} {}", color[0], color[1], color[2])?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imaginary: f64,
}

impl Complex {
    pub fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }

    pub fn norm_squared(self) -> f64 {
        self.real * self.real + self.imaginary * self.imaginary
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.real + rhs.real, self.imaginary + rhs.imaginary)
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.real - rhs.real, self.imaginary - rhs.imaginary)
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.real * rhs.real - self.imaginary * rhs.imaginary,
            self.real * rhs.imaginary + self.imaginary * rhs.real,
        )
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denominator = rhs.norm_squared();
        Self::new(
            (self.real * rhs.real + self.imaginary * rhs.imaginary) / denominator,
            (self.imaginary * rhs.real - self.real * rhs.imaginary) / denominator,
        )
    }
}

pub fn mandelbrot(
    width: usize,
    height: usize,
    minimum: Complex,
    maximum: Complex,
    max_iterations: u32,
) -> Image {
    let mut image = Image::new(width, height);

    if max_iterations == 0 {
        return image;
    }

    let horizontal_pixels = if width > 1 { width - 1 } else { 1 };
    let vertical_pixels = if height > 1 { height - 1 } else { 1 };

    for y in 0..height {
        let y_ratio = y as f64 / vertical_pixels as f64;
        let imaginary = minimum.imaginary + y_ratio * (maximum.imaginary - minimum.imaginary);

        for x in 0..width {
            let x_ratio = x as f64 / horizontal_pixels as f64;
            let real = minimum.real + x_ratio * (maximum.real - minimum.real);
            let point = Complex::new(real, imaginary);
            let mut value = Complex::new(0.0, 0.0);
            let mut iterations = 0;

            while value.norm_squared() <= 4.0 && iterations < max_iterations {
                value = value * value + point;
                iterations += 1;
            }

            let color = if iterations == max_iterations {
                [0, 0, 0]
            } else {
                let brightness = (255 * iterations / max_iterations) as u8;
                [brightness, 255 - brightness, 180]
            };
            image.set_pixel(x, y, color);
        }
    }

    image
}

fn main() -> io::Result<()> {
    let image = mandelbrot(
        800,
        600,
        Complex::new(-2.5, -1.5),
        Complex::new(1.0, 1.5),
        1_000,
    );
    image.save_ppm("mandelbrot.ppm")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn image_stores_pixels_and_writes_ppm_header() {
        let mut image = Image::new(2, 1);
        assert!(image.set_pixel(1, 0, [12, 34, 56]));
        assert_eq!(image.pixel(1, 0), Some([12, 34, 56]));
        assert!(!image.set_pixel(2, 0, [0, 0, 0]));

        let path = std::env::temp_dir().join("lista4_image_test.ppm");
        image.save_ppm(path.to_str().unwrap()).unwrap();
        let contents = fs::read_to_string(&path).unwrap();
        fs::remove_file(path).unwrap();

        assert!(contents.starts_with("P3\n2 1\n255\n"));
        assert!(contents.contains("12 34 56"));
    }

    #[test]
    fn complex_operations_follow_standard_formulas() {
        let first = Complex::new(3.0, 2.0);
        let second = Complex::new(1.0, -4.0);

        assert_eq!(first + second, Complex::new(4.0, -2.0));
        assert_eq!(first - second, Complex::new(2.0, 6.0));
        assert_eq!(first * second, Complex::new(11.0, -10.0));
        assert_eq!(
            first / second,
            Complex::new(-0.29411764705882354, 0.8235294117647058)
        );
    }

    #[test]
    fn fractal_supports_different_views_and_resolutions() {
        let overview = mandelbrot(
            32,
            24,
            Complex::new(-2.5, -1.5),
            Complex::new(1.0, 1.5),
            100,
        );
        let detail = mandelbrot(
            17,
            11,
            Complex::new(-0.8, -0.05),
            Complex::new(-0.7, 0.05),
            250,
        );

        assert_eq!((overview.width(), overview.height()), (32, 24));
        assert_eq!((detail.width(), detail.height()), (17, 11));
        assert_ne!(overview.pixel(0, 0), Some([0, 0, 0]));
        assert_eq!(detail.pixel(8, 5), Some([0, 0, 0]));
    }
}
