//! Module for generating waves
//!
//! This is more experimental

use core::f64;

use csscolorparser::Color;
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_filled_circle_mut;

use crate::WaveType;

pub struct WaveOptions {
    color: Color,
    wave_type: WaveType,
    width: u32,
    height: u32,
}

impl WaveOptions {
    pub fn new(color: Color, wave_type: WaveType, width: u32, height: u32) -> Self {
        Self {
            color,
            wave_type,
            width,
            height,
        }
    }
}

pub fn generate_wave_images(options: WaveOptions) -> Vec<RgbaImage> {
    let mut images = Vec::new();

    let half_y = options.height / 2;
    let color_pixel = options.color.to_rgba8();
    for x in (0..options.width).step_by(4) {
        // copy last image or create new one
        let mut image = images
            .last()
            .map(|i: &ImageBuffer<Rgba<u8>, Vec<u8>>| i.clone())
            .unwrap_or_else(|| ImageBuffer::new(options.width, options.height));
        let distance_through_radians = 2.0 * f64::consts::PI * x as f64 / options.width as f64;

        let wave_function = match options.wave_type {
            WaveType::Sine => f64::sin,
            WaveType::Cosine => f64::cos,
            WaveType::Tangent => f64::tan,
        };
        let y = wave_function(distance_through_radians) * half_y as f64 * 0.5;
        let y = half_y as i32 + y as i32;
        draw_filled_circle_mut(&mut image, (x as i32, y), 30, Rgba(color_pixel));

        images.push(image);
    }

    images
}
