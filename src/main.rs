use std::{fs::File, io::BufWriter, time::Instant};

use clap::{Parser, Subcommand, ValueEnum};
use csscolorparser::Color;

use crate::{
    mandelbrot::{generate_mandelbrot_image, MandelbrotImageOptions}, sierpinski::generate_sierpinski_image, ulam_spiral::{generate_ulam_spiral_image, UlamSpiralOptions}, waves::{generate_wave_images, WaveOptions}
};

mod mandelbrot;
mod sierpinski;
mod ulam_spiral;
mod waves;

fn main() {
    let args = Args::parse();

    let format = args.image_type.get_format();
    match format {
        ImageFormat::Static => {
            save_static_image(args);
        }
        ImageFormat::Animated => {
            save_animated_image(args);
        }
    }
}

fn save_static_image(args: Args) {
    let start = Instant::now();

    let image = match args.image_type {
        ImageType::UlamSpiral {
            size,
            color,
            mode,
            background_color,
        } => {
            generate_ulam_spiral_image(UlamSpiralOptions::new(size, color, mode, background_color))
        }
        ImageType::Mandelbrot {
            color,
            background_color,
            gradient,
        } => generate_mandelbrot_image(MandelbrotImageOptions::new(
            color,
            background_color,
            gradient,
        )),
        ImageType::Wave { .. } => unreachable!(),
        ImageType::Sierpinski { color, size } => generate_sierpinski_image(color, size),
    };
    let end = Instant::now();
    println!("Generated image in {}ms", (end - start).as_millis());

    if let Err(image_error) = image.save(&args.output) {
        eprintln!("Error saving image: {:?}", image_error);
    } else {
        println!("Saved image to {}", &args.output);
    }
}

fn save_animated_image(args: Args) {
    match args.image_type {
        ImageType::UlamSpiral { .. } => unreachable!(),
        ImageType::Mandelbrot { .. } => unreachable!(),
        ImageType::Wave { color, wave_type } => {
                        let width = 500;
                        let height = 500;
                        let wave_images =
                            generate_wave_images(WaveOptions::new(color, wave_type, width, height));
                        let file_name = if args.output.ends_with(".png") {
                            args.output
                        } else {
                            format!("{}.png", args.output)
                        };
                        let file = File::create(file_name).unwrap();
                        let writer = &mut BufWriter::new(file);
        
                        let mut png_encoder = png::Encoder::new(writer, width, height);
                        png_encoder.set_color(png::ColorType::Rgba);
                        png_encoder.set_depth(png::BitDepth::Eight);
                        png_encoder
                            .set_animated(wave_images.len() as u32, 0)
                            .expect("Couldn't set animated");
                        let mut writer = png_encoder.write_header().expect("Couldn't write header");
                        for wave_image in wave_images.iter() {
                            writer
                                .write_image_data(&wave_image)
                                .expect("Couldn't write image data");
                        }
                        writer.finish().expect("Couldn't finish writing");
            }
        ImageType::Sierpinski { .. } => unreachable!(),
    }
}

/// Args for the program
#[derive(Parser, Debug)]
#[command(version, about = "A CLI for generating fun images", long_about = None)]
struct Args {
    /// The image type to generate
    #[command(subcommand)]
    image_type: ImageType,

    /// The image output file name
    #[arg(short, long, default_value = "image.webp")]
    output: String,
}

/// The image type to generate
#[derive(Debug, Subcommand)]
enum ImageType {
    UlamSpiral {
        /// The size of the spiral to go up to, defaults to 201 squared
        #[arg(short, long, default_value = "40401")]
        size: u32,
        
        #[arg(short, long, default_value = "black")]
        color: Color,
        
        #[arg(short, long, default_value = "prime-only")]
        mode: UlamSpiralMode,
        
        #[arg(short, long, default_value = "white")]
        background_color: Color,
    },
    Mandelbrot {
        #[arg(short, long, default_value = "black")]
        color: Color,
        
        #[arg(short, long, default_value = "white")]
        background_color: Color,
        
        #[arg(short, long, default_value = "false")]
        gradient: bool,
    },
    Wave {
        #[arg(short, long, default_value = "black")]
        color: Color,
        
        #[arg(short, long, default_value = "sine")]
        wave_type: WaveType,
    },
    Sierpinski {
        #[arg(short, long, default_value = "black")]
        color: Color,

        #[arg(short, long, default_value = "1000")]
        size: u32,
    }
}

impl ImageType {
    fn get_format(&self) -> ImageFormat {
        match self {
            ImageType::UlamSpiral { .. } => ImageFormat::Static,
            ImageType::Mandelbrot { .. } => ImageFormat::Static,
            ImageType::Wave { .. } => ImageFormat::Animated,
            ImageType::Sierpinski { .. } => ImageFormat::Static,
        }
    }
}

enum ImageFormat {
    Static,
    Animated,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum UlamSpiralMode {
    /// Generates pixels for the primes only
    PrimeOnly,
    /// Generates circles based on how many divisors a number has
    Divisor,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum WaveType {
    /// Generates a sine wave
    Sine,
    /// Generates a cosine wave
    Cosine,
    /// Generates a tangent wave
    Tangent,
}
