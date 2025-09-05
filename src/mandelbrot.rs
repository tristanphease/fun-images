//! Module for generating a mandelbrot image
//! 
//! The standard cool image so got to have it here:
//! See <https://en.wikipedia.org/wiki/Mandelbrot_set> for more info

use csscolorparser::Color;
use image::{DynamicImage, ImageBuffer, Rgba};
use num_complex::{Complex64};

pub struct MandelbrotImageOptions {
    color: Color,
    background_color: Color,
    use_gradient: bool,
} 

impl MandelbrotImageOptions {
    pub fn new(color: Color, background_color: Color, use_gradient: bool) -> Self {
        Self { color, background_color, use_gradient }
    }
}

const MAX_ITER_NUM: u32 = 200;

pub fn generate_mandelbrot_image(options: MandelbrotImageOptions) -> DynamicImage {
    const IMAGE_WIDTH: u32 = 1600;
    const IMAGE_HEIGHT: u32 = 1200;
    let mut image = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let viewport = ViewPort::normal_mandelbrot();

    let converted_color = options.color.to_rgba8();
    let converted_background_color = options.background_color.to_rgba8();

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let real = (x as f64) / (IMAGE_WIDTH as f64) * viewport.real_diameter - viewport.real_diameter / 2.0 + viewport.centre.re;
            let imaginary = (y as f64) / (IMAGE_HEIGHT as f64) * viewport.imaginary_diameter - viewport.imaginary_diameter / 2.0 + viewport.centre.im;

            let complex = Complex64::new(real, imaginary);

            if let Some(iter_num) = check_mandelbrot(complex) {
                if options.use_gradient {
                    let grad_color = get_interp(converted_background_color, converted_color,
                        iter_num as f64 / MAX_ITER_NUM as f64);
                    image[(x, y)] = Rgba(grad_color);
                } else {
                    image[(x, y)] = Rgba(converted_color);
                }
            } else {
                image[(x, y)] = Rgba(converted_background_color);
            }
        }
    }

    DynamicImage::ImageRgba8(image)
}

fn check_mandelbrot(complex: Complex64) -> Option<u32> {
    let z = Complex64::new(0.0, 0.0);

    check_mandelbrot_recursion(z, complex, 0)
}

fn check_mandelbrot_recursion(z: Complex64, c: Complex64, iteration_num: u32) -> Option<u32> {
    let new_z = z * z + c;
    if new_z.re.abs() > 20.0 || new_z.im.abs() > 20.0 {
        // return the iteration number for gradient
        return Some(iteration_num);
    }

    if iteration_num > MAX_ITER_NUM {
        return None;
    }

    check_mandelbrot_recursion(new_z, c, iteration_num + 1)
}

fn get_interp(color1: [u8; 4], color2: [u8; 4], amount: f64) -> [u8; 4] {
    let interp = |x: u8, y: u8| ((x as f64) * amount + (y as f64) * (1.0 - amount)) as u8; 

    [
        interp(color1[0], color2[0]),
        interp(color1[1], color2[1]),
        interp(color1[2], color2[2]),
        interp(color1[3], color2[3]),
    ]
}

struct ViewPort {
    centre: Complex64,
    real_diameter: f64,
    imaginary_diameter: f64,
}

impl ViewPort {
    fn normal_mandelbrot() -> Self {
        Self {
            centre: Complex64::new(-0.7, 0.0),
            real_diameter: 3.0769,
            imaginary_diameter: 2.307675,
        }
    }
}