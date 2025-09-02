use clap::{Parser, ValueEnum};

use crate::ulam_spiral::{generate_ulam_spiral_image, UlamSpiralOptions};

mod ulam_spiral;

fn main() {
    let args = Args::parse();

    let image = match args.image_type {
        ImageType::UlamSpiral => generate_ulam_spiral_image(UlamSpiralOptions::new(201 * 201)) 
    };

    if let Err(image_error) = image.save(args.output) {
        eprintln!("Error occured saving: {:?}", image_error);
    }
    
}


#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    image_type: ImageType,

    #[arg(short, long, default_value = "image.webp")]
    output: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ImageType {
    UlamSpiral
}