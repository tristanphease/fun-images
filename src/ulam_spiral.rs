//! Module for generating a ulam spiral
//!
//! Can generate either the typical prime spiral or a spiral which shows the number of divisors
//!

use csscolorparser::Color;
use image::{DynamicImage, ImageBuffer, Rgba};
use imageproc::drawing::draw_filled_circle_mut;

use crate::UlamSpiralMode;

#[derive(Clone, Debug)]
pub struct UlamSpiralOptions {
    size: u32,
    color: Color,
    mode: UlamSpiralMode,
    background_color: Color,
}

impl UlamSpiralOptions {
    pub fn new(size: u32, color: Color, mode: UlamSpiralMode, background_color: Color) -> Self {
        Self {
            size,
            color,
            mode,
            background_color,
        }
    }

    fn get_image_size(&self) -> u32 {
        let mut image_size = self.size.isqrt();
        // since the square root rounds down, we want to round up instead if it's not exact
        if image_size * image_size != self.size {
            image_size += 1;
        }
        // if even, make it odd to centre the image
        if image_size.is_multiple_of(2) {
            image_size += 1;
        }
        image_size
    }
}

pub fn generate_ulam_spiral_image(options: UlamSpiralOptions) -> DynamicImage {
    match options.mode {
        UlamSpiralMode::PrimeOnly => generate_prime_ulam_spiral(options),
        UlamSpiralMode::Divisor => generate_divisor_ulam_spiral(options),
    }
}

fn generate_prime_ulam_spiral(options: UlamSpiralOptions) -> DynamicImage {
    let image_size = options.get_image_size();
    let mut image = ImageBuffer::<Rgba<u8>, _>::new(image_size, image_size);

    let spiral_pattern = SpiralPatternIterator::new(options.size, image_size);

    let converted_color = options.color.to_rgba8();
    let converted_background_color = options.background_color.to_rgba8();

    for (value, (x, y)) in spiral_pattern.enumerate() {
        let colour = if primal::is_prime(value as u64) {
            Rgba(converted_color)
        } else {
            Rgba(converted_background_color)
        };
        image[(x, y)] = colour;
    }

    DynamicImage::ImageRgba8(image)
}

fn generate_divisor_ulam_spiral(options: UlamSpiralOptions) -> DynamicImage {
    const DEFAULT_CIRCLE_SIZE: u32 = 10;

    let image_size = options.get_image_size();
    let image_dimension = image_size * DEFAULT_CIRCLE_SIZE;
    let mut image = ImageBuffer::<Rgba<u8>, _>::new(image_dimension, image_dimension);

    // set background
    let converted_background_color = options.background_color.to_rgba8();
    image
        .pixels_mut()
        .for_each(|x| *x = Rgba(converted_background_color));

    let converted_color = options.color.to_rgba8();

    let spiral_pattern = SpiralPatternIterator::new(options.size, image_size);

    for (value, (x, y)) in spiral_pattern.enumerate() {
        let square_root = (value as u32).isqrt();
        if square_root == 0 {
            continue;
        }
        let num_factors = get_factor_num(value as u32, square_root);
        // could we do something where we scale the circle size by the square root so
        // we don't bias in favour of images outside the centre?
        let circle_size = num_factors / 3;
        let x = (x * DEFAULT_CIRCLE_SIZE) as i32;
        let y = (y * DEFAULT_CIRCLE_SIZE) as i32;
        draw_filled_circle_mut(
            &mut image,
            (x, y),
            circle_size as i32,
            Rgba(converted_color),
        );
    }

    DynamicImage::ImageRgba8(image)
}

/// gets half the factors, searches up to the num which should be the square root
fn get_factor_num(num: u32, search_num: u32) -> u32 {
    1 + (2..=search_num).filter(|&x| num.is_multiple_of(x)).count() as u32
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn get_next(self, direction: SpiralDirection) -> Self {
        match (self, direction) {
            (Direction::Up, SpiralDirection::Clockwise) => Direction::Right,
            (Direction::Up, SpiralDirection::AntiClockwise) => Direction::Left,
            (Direction::Left, SpiralDirection::Clockwise) => Direction::Up,
            (Direction::Left, SpiralDirection::AntiClockwise) => Direction::Down,
            (Direction::Right, SpiralDirection::Clockwise) => Direction::Down,
            (Direction::Right, SpiralDirection::AntiClockwise) => Direction::Up,
            (Direction::Down, SpiralDirection::Clockwise) => Direction::Left,
            (Direction::Down, SpiralDirection::AntiClockwise) => Direction::Right,
        }
    }

    fn same_axis(self, direction: Self) -> bool {
        !Self::different_axis(self, direction)
    }

    fn different_axis(self, direction: Self) -> bool {
        match (self, direction) {
            (Direction::Up, Direction::Up) => false,
            (Direction::Up, Direction::Left) => true,
            (Direction::Up, Direction::Right) => true,
            (Direction::Up, Direction::Down) => false,
            (Direction::Left, Direction::Up) => true,
            (Direction::Left, Direction::Left) => false,
            (Direction::Left, Direction::Right) => false,
            (Direction::Left, Direction::Down) => true,
            (Direction::Right, Direction::Up) => true,
            (Direction::Right, Direction::Left) => false,
            (Direction::Right, Direction::Right) => false,
            (Direction::Right, Direction::Down) => true,
            (Direction::Down, Direction::Up) => false,
            (Direction::Down, Direction::Left) => true,
            (Direction::Down, Direction::Right) => true,
            (Direction::Down, Direction::Down) => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SpiralDirection {
    #[allow(dead_code)]
    Clockwise,
    AntiClockwise,
}

#[derive(Clone, Copy, Debug)]
struct SpiralPatternIterator {
    /// The direction the spiral is currently running
    direction: Direction,
    /// The spiral direction (clockwise or anti-clockwise)
    spiral_direction: SpiralDirection,
    /// The amount through the current direction
    amount_through_direction: u32,
    /// The length of the current spiral
    spiral_num: u32,
    /// The current x pos
    x: u32,
    /// The current y pos
    y: u32,
    /// The total number to go through in the spiral
    total_size: u32,
    /// The direction we start at, necessary to know
    start_direction: Direction,
    /// Amount through the total size
    amount_through: u32,
}

impl SpiralPatternIterator {
    fn new(total_size: u32, image_width: u32) -> Self {
        let start_direction = Direction::Right;
        Self {
            direction: start_direction,
            spiral_direction: SpiralDirection::AntiClockwise,
            amount_through_direction: 0,
            spiral_num: 1,
            x: image_width / 2,
            y: image_width / 2,
            total_size,
            start_direction,
            amount_through: 0,
        }
    }
}

impl Iterator for SpiralPatternIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.amount_through >= self.total_size {
            return None;
        }

        let result = (self.x, self.y);

        self.amount_through += 1;

        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
        }

        self.amount_through_direction += 1;
        if self.amount_through_direction >= self.spiral_num {
            let new_direction = self.direction.get_next(self.spiral_direction);
            if Direction::same_axis(self.start_direction, new_direction) {
                self.spiral_num += 1;
            }
            self.amount_through_direction = 0;
            self.direction = new_direction;
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_pattern() {
        let total = 21;
        let centre = total / 2;
        let mut spiral_pattern = SpiralPatternIterator::new(total, total);
        assert_eq!(Some((centre, centre)), spiral_pattern.next());
        assert_eq!(Some((centre + 1, centre)), spiral_pattern.next());
        assert_eq!(Some((centre + 1, centre - 1)), spiral_pattern.next());
        assert_eq!(Some((centre, centre - 1)), spiral_pattern.next());
        assert_eq!(Some((centre - 1, centre - 1)), spiral_pattern.next());
        assert_eq!(Some((centre - 1, centre)), spiral_pattern.next());
        assert_eq!(Some((centre - 1, centre + 1)), spiral_pattern.next());
        assert_eq!(Some((centre, centre + 1)), spiral_pattern.next());
        assert_eq!(Some((centre + 1, centre + 1)), spiral_pattern.next());
        assert_eq!(Some((centre + 2, centre + 1)), spiral_pattern.next());
        assert_eq!(Some((centre + 2, centre)), spiral_pattern.next());
        assert_eq!(Some((centre + 2, centre - 1)), spiral_pattern.next());
        assert_eq!(Some((centre + 2, centre - 2)), spiral_pattern.next());
        assert_eq!(Some((centre + 1, centre - 2)), spiral_pattern.next());
        assert_eq!(Some((centre, centre - 2)), spiral_pattern.next());
        assert_eq!(Some((centre - 1, centre - 2)), spiral_pattern.next());
        assert_eq!(Some((centre - 2, centre - 2)), spiral_pattern.next());
        assert_eq!(Some((centre - 2, centre - 1)), spiral_pattern.next());
        assert_eq!(Some((centre - 2, centre)), spiral_pattern.next());
        assert_eq!(Some((centre - 2, centre + 1)), spiral_pattern.next());
        assert_eq!(Some((centre - 2, centre + 2)), spiral_pattern.next());
        assert_eq!(None, spiral_pattern.next());
    }
}
