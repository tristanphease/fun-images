use std::time::Instant;

use clap::{Parser, Subcommand, ValueEnum};
use csscolorparser::Color;

use crate::{mandelbrot::{generate_mandelbrot_image, MandelbrotImageOptions}, ulam_spiral::{generate_ulam_spiral_image, UlamSpiralOptions}};

mod mandelbrot;
mod ulam_spiral;

fn main() {
    let args = Args::parse();

    let start = Instant::now();
    let image = match args.image_type {
        ImageType::UlamSpiral {
            size,
            color,
            mode,
            background_color
        } => {
            generate_ulam_spiral_image(UlamSpiralOptions::new(size, color, mode, background_color))
        },
        ImageType::Mandelbrot { 
            color,
            background_color,
            gradient,
        } => generate_mandelbrot_image(MandelbrotImageOptions::new(color, background_color, gradient))
    };
    let end = Instant::now();
    println!("Generated image in {}ms", (end - start).as_millis());

    if let Err(image_error) = image.save(&args.output) {
        eprintln!("Error saving image: {:?}", image_error);
    } else {
        println!("Saved image to {}", &args.output);
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
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum UlamSpiralMode {
    /// Generates pixels for the primes only
    PrimeOnly,
    /// Generates circles based on how many divisors a number has
    Divisor
}
