//! Perlin noise as per <https://en.wikipedia.org/wiki/Perlin_noise>
//!

use std::f64;

use csscolorparser::Color;
use image::{DynamicImage, Rgba, RgbaImage};

type Vec2 = (f64, f64);

pub struct PerlinNoiseOptions {
    size: u32,
    color1: Color,
    color2: Color,
}

impl PerlinNoiseOptions {
    pub fn new(size: u32, color1: Color, color2: Color) -> Self {
        Self {
            size,
            color1,
            color2,
        }
    }
}

pub fn generate_perlin_noise(options: PerlinNoiseOptions) -> DynamicImage {
    let PerlinNoiseOptions {
        size,
        color1,
        color2,
    } = options;
    let mut image = RgbaImage::new(size, size);

    // generate grid
    const GRID_SIZE: usize = 20;
    let grid_size = size as usize / GRID_SIZE + 1;
    let mut grid = vec![(0.0, 0.0); (grid_size * grid_size) as usize];

    grid.iter_mut().for_each(|x| {
        *x = random_vec2();
    });

    let grid_iter = (0..grid_size).flat_map(|x| std::iter::repeat(x).take(GRID_SIZE as usize));
    for (y, grid_y) in (0..size).zip(grid_iter.clone()) {
        for (x, grid_x) in (0..size).zip(grid_iter.clone()) {
            // top left, top right, bottom left, bottom right vecs
            let grid_left_x = grid_x;
            let grid_right_x = if grid_x + 1 >= grid_size {
                grid_x
            } else {
                grid_x + 1
            };
            let grid_top_y = grid_y;
            let grid_bottom_y = if grid_y + 1 >= grid_size {
                grid_y
            } else {
                grid_y + 1
            };
            
            let vec_1 = grid[grid_top_y * grid_size + grid_left_x];
            let vec_2 = grid[grid_top_y * grid_size + grid_right_x];
            let vec_3 = grid[grid_bottom_y * grid_size + grid_left_x];
            let vec_4 = grid[grid_bottom_y * grid_size + grid_right_x];

            let offset_1 = grid_distance(
                (grid_left_x as u32, grid_top_y as u32),
                (x, y),
                GRID_SIZE as u32,
            );
            let offset_2 = grid_distance(
                (grid_right_x as u32, grid_top_y as u32),
                (x, y),
                GRID_SIZE as u32,
            );
            let offset_3 = grid_distance(
                (grid_left_x as u32, grid_bottom_y as u32),
                (x, y),
                GRID_SIZE as u32,
            );
            let offset_4 = grid_distance(
                (grid_right_x as u32, grid_bottom_y as u32),
                (x, y),
                GRID_SIZE as u32,
            );

            let dot_1 = dot(vec_1, offset_1);
            let dot_2 = dot(vec_2, offset_2);
            let dot_3 = dot(vec_3, offset_3);
            let dot_4 = dot(vec_4, offset_4);
            
            let frac_x = fade((x % GRID_SIZE as u32) as f64 / GRID_SIZE as f64);
            let frac_y = fade((y % GRID_SIZE as u32) as f64 / GRID_SIZE as f64);
            
            let val1 = interpolate(dot_1, dot_2, frac_x);
            let val2 = interpolate(dot_3, dot_4, frac_x);
            let value = interpolate(val1, val2, frac_y);
            
            let value = (value as f32 + 1.0) / 2.0;
            let color = Color {
                r: color1.r * value + color2.r * (1.0 - value),
                g: color1.g * value + color2.g * (1.0 - value),
                b: color1.b * value + color2.b * (1.0 - value),
                a: color1.a * value + color2.a * (1.0 - value),
            };

            image[(x, y)] = Rgba(color.to_rgba8());
        }
    }

    DynamicImage::ImageRgba8(image)
}

fn interpolate(a: f64, b: f64, x: f64) -> f64 {
    a * (1.0 - x) + b * x
}

fn fade(val: f64) -> f64 {
    6.0 * val * val * val * val * val - 15.0 * val * val * val * val + 10.0 * val * val * val
}

// random unit length 2d vector
fn random_vec2() -> Vec2 {
    let angle = fastrand::f64() * 2.0 * f64::consts::PI;
    (f64::cos(angle), f64::sin(angle))
}

// dot product
// https://en.wikipedia.org/wiki/Dot_product
fn dot(vec1: Vec2, vec2: Vec2) -> f64 {
    vec1.0 * vec2.0 + vec1.1 * vec2.1
}

// get offset vector
fn offset(vec1: Vec2, vec2: Vec2) -> Vec2 {
    (vec2.0 - vec1.0, vec2.1 - vec1.1)
}

fn grid_distance(grid_pos: (u32, u32), point_pos: (u32, u32), grid_size: u32) -> Vec2 {
    let grid_pos = (grid_pos.0 as f64, grid_pos.1 as f64);
    let point_pos = (
        point_pos.0 as f64 / grid_size as f64,
        point_pos.1 as f64 / grid_size as f64,
    );

    offset(point_pos, grid_pos)
}
