use std::time::Instant;

use clap::{Parser, Subcommand};

use crate::ulam_spiral::{generate_ulam_spiral_image, UlamSpiralOptions};

mod ulam_spiral;

fn main() {
    let args = Args::parse();

    let start = Instant::now();
    let image = match args.image_type {
        ImageType::UlamSpiral {
            size
        } => generate_ulam_spiral_image(UlamSpiralOptions::new(size)) 
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
        size: u32
    }
}