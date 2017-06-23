extern crate num;
extern crate num_complex;
extern crate image;

mod mandelbrot;
use num_complex::Complex64;
use std::fs::File;
use std::path::Path;
use mandelbrot::mandelbrot;
use image::Luma;
use image::{
    ImageBuffer
};

const MIN: f64 = -2f64;
const MAX: f64 = 2f64;

fn main() {
    render(1000, 1000);
}

fn render(width: u32, height: u32) {
    let mut img: ImageBuffer<Luma<u8>, _> = ImageBuffer::new(width, height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let x_scale = scale(x as f64, &Bounds{min: 0f64, max: (width-1u32) as f64});
        let y_scale = scale(y as f64, &Bounds{min: 0f64, max: (height-1u32) as f64});

        let val = mandelbrot(Complex64::new(x_scale, y_scale), 1000);
        *pixel = Luma([val.unwrap_or(0) as u8]);
    }

    let mut file = File::create(&Path::new("mandelbrot.png")).unwrap();
    let _ = image::ImageLuma8(img).save(&mut file, image::PNG); 
}

#[derive(Debug)]
struct Bounds {
    min: f64,
    max: f64
}

fn scale(val: f64, bounds: &Bounds) -> f64 {
    assert!(val >= bounds.min && val <= bounds.max, "Value {} must be within bounds {:?}", val, bounds);

    let norm_val = |val| { (val - bounds.min) / (bounds.max - bounds.min) };
    let scale_norm = |norm| { (norm * (MAX - MIN)) + MIN };
    let scale_val = |val| { scale_norm(norm_val(val)) };
    return scale_val(val)
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_scale() {
        use super::*;

        let bounds = Bounds { min: 100f64, max: 200f64 };
        assert_eq!(scale(100f64, 200f64, &bounds), (MIN, MAX));
        assert_eq!(scale(150f64, 150f64, &bounds), ((MIN+MAX)/2f64, (MIN+MAX)/2f64))
    }
}
